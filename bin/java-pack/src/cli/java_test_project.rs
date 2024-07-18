#![allow(dead_code, unused_imports)]
use std::{
    fs::{self},
    path::{Path, PathBuf},
};

use super::{
    cli_utils::{self, create_file},
    java_templates::test::*,
};
use color_eyre::eyre::{bail, Context};
use java_bindgen_core::{
    consts, ffi_store::FFIStore, project_info::ProjectInfo, utils::create_or_get_dir,
};

pub fn process_template(template: &str, project_info: &ProjectInfo) -> String {
    template
        .replace("[[package_name]]", &project_info.java_package_name)
        .replace("[[lib-name]]", &project_info.lib_name)
        .replace("[[lib-version]]", &project_info.lib_version)
        .replace("[[java-class-name]]", &project_info.get_java_class_name())
}

pub fn setup_tests_java_project(
    project_dir: &Path,
    project_info: &ProjectInfo,
) -> color_eyre::Result<()> {
    let test_dir = project_dir.join(project_info.tests_java_dir_name());
    if test_dir.exists() {
        // todo validate project
        return Ok(());
    }
    let test_dir = create_or_get_dir(&test_dir)?;

    let src = create_or_get_dir(&test_dir.join("src"))?;
    let test = create_or_get_dir(&src.join("test"))?;
    let java = create_or_get_dir(&test.join("java"))?;
    let resources = create_or_get_dir(&test.join("resources"))?;
    let java_bindgen = create_or_get_dir(&java.join("bindgen"))?;

    // Create pom
    create_file(
        &test_dir,
        "pom.xml",
        &process_template(JAVA_TEST_POM_TEMPLATE, project_info),
    )?;

    // Create pom
    create_file(
        &test_dir,
        ".gitignore",
        &process_template(JAVA_TEST_GIT_IGNORE, project_info),
    )?;    
    
    // Create pom
    create_file(
        &resources,
        "log4j.properties",
        &process_template(JAVA_TEST_LOG4J_PROPERTIES, project_info),
    )?;

    // Create test file
    create_file(
        &java_bindgen,
        &format!("{}Test.java", &project_info.get_java_class_name()),
        &process_template(JAVA_TEST_TEMPLATE, project_info),
    )?;

    Ok(())
}

pub fn install_jar(
    project_dir: &Path,
    jar_path: &Path,
    project_info: &ProjectInfo,
) -> color_eyre::Result<()> {
    let test_dir = project_dir.join(project_info.tests_java_dir_name());
    if !test_dir.exists() {
        return Ok(())
    }

    let jar_path = jar_path.to_string_lossy().to_string();
    let group_id = &project_info.java_package_name;
    let artefact_id = &project_info.lib_name;
    let varsion = &project_info.lib_version;
    let command = format!(
        "mvn deploy:deploy-file 
    -Dfile={jar_path}  
    -DgroupId={group_id} 
    -DartifactId={artefact_id} 
    -Dversion={varsion} 
    -DrepositoryId=local-maven-repo 
    -Durl=file:./local-maven-repo/ 
    -DupdateReleaseInfo=true"
    );

    cli_utils::exec_command(&test_dir, &command, "Install Jar")?;

    Ok(())
}


pub fn runt_tests(
    project_dir: &Path,
    project_info: &ProjectInfo,
) -> color_eyre::Result<()> {
    let test_dir = project_dir.join(project_info.tests_java_dir_name());
    if !test_dir.exists() {
        return Ok(())
    }

    cli_utils::exec_command(&test_dir, "mvn test", "Run Java Tests")?;
    Ok(())
}



