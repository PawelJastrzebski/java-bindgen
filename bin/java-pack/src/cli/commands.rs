use color_eyre::eyre::bail;
use java_bindgen_core::{cargo_parser::parse_toml, consts, project_info::ProjectInfo, utils::create_or_get_dir};
use std::path::{Path, PathBuf};

use super::{cli_utils::{self, header}, java_build_project, java_test_project};

pub fn get_jar_path(project_dir: &PathBuf, project_info: &ProjectInfo) -> (String, Option<PathBuf>) {
    let target_path = project_dir.join("target");
    let jar_name = project_info.jar_final_name();
    let jar_name_path = target_path.join(&jar_name);

    if let Ok(path) = jar_name_path.canonicalize() {
        if path.is_file() {
            return (jar_name, Some(path));
        }
    }

    (jar_name, None)
}


pub fn build(project_dir: &Path) -> color_eyre::Result<()> {
    let toml_path = consts::cargo_toml_path(&project_dir);
    let target_java_build = consts::java_build_dir(&project_dir);
    let target_path = create_or_get_dir(&project_dir.join("target"))?;

    let toml = parse_toml(&toml_path)?.toml_parsed;
    let project_info = ProjectInfo::from(&toml);
    
    cli_utils::exec_command(project_dir, "cargo build --color always", "Binary")?;
    cli_utils::sleep(100);

    java_build_project::setup_java_project(&project_dir, &target_java_build, &project_info)?;
    cli_utils::sleep(100);

    let binary = java_build_project::find_native_lib(&project_info.get_native_lib_name(), &project_dir);
    java_build_project::build_jar(&target_java_build, &project_info, &binary)?;
    cli_utils::sleep(100);

    java_build_project::copy_jar_to(&target_java_build, &target_path, &project_info)
}

pub(crate) fn clear(project_dir: &PathBuf) -> color_eyre::Result<()> {
    cli_utils::exec_command(&project_dir, "cargo clean --color always", "Clean")
}

pub(crate) fn run_jar(project_dir: &PathBuf) -> color_eyre::Result<()> {
    let target_path = project_dir.join("target");
    let toml_path = consts::cargo_toml_path(&project_dir);

    let toml = parse_toml(&toml_path)?.toml_parsed;
    let project_info = ProjectInfo::from(&toml);

    let (_, jar) = get_jar_path(&project_dir, &project_info);
    if jar.is_none() {
        println!("{}", header("Jar not extist"));
        println!("Building new jar..\n");
        build(project_dir)?;
    }

    let (jar_name, jar)  = get_jar_path(&project_dir, &project_info);
    if jar.is_none() {
        bail!("Jar not exist")
    }

    let command = format!("java -jar ./{jar_name}");
    cli_utils::exec_command(&target_path, &command, "Run jar")
}

pub(crate) fn init_cargo_project(_project_dir: &PathBuf) -> color_eyre::Result<()> {
    // let command = format!("cargo init --bin --color always");
    // utils::exec_command(&project_dir, &command, "Init Cargo Project")

    // todo
    Ok(())
}

pub(crate) fn setup_test_project(project_dir: &PathBuf) -> color_eyre::Result<()> {
    let toml_path = consts::cargo_toml_path(&project_dir);
    let toml = parse_toml(&toml_path)?.toml_parsed;
    let project_info = ProjectInfo::from(&toml);

    // Setup Java Project
    java_test_project::setup_tests_java_project(project_dir, &project_info)?;
    // Run tests
    run_tests(project_dir)
}


pub(crate) fn run_tests(project_dir: &PathBuf) -> color_eyre::Result<()> {
    let toml_path = consts::cargo_toml_path(&project_dir);
    let toml = parse_toml(&toml_path)?.toml_parsed;
    let project_info = ProjectInfo::from(&toml);

        // Install Jar
        let (_, jar) = get_jar_path(&project_dir, &project_info);
        if jar.is_none() {
            println!("{}", header("Jar not extist"));
            println!("Building new jar..\n");
            build(project_dir)?;
        }
    
        let (_, jar)  = get_jar_path(&project_dir, &project_info);
        if jar.is_none() {
            bail!("Jar not exist")
        }
    

    java_test_project::install_jar(&project_dir, &jar.unwrap(), &project_info)?;

    java_test_project::runt_tests(&project_dir, &project_info)?;
    Ok(())
}

