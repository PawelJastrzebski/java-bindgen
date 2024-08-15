use std::{
    fs::{self},
    path::{Path, PathBuf},
};

use super::{cli_utils::{self, create_file}, java_templates::build::*};
use color_eyre::eyre::Context;
use java_bindgen_core::{
    consts, ffi_store::FFIStore, project_info::ProjectInfo, utils::create_or_get_dir,
};

#[derive(Debug, Default)]
pub struct RustBinaryInfo {
    pub linux_binary_path: Option<PathBuf>,
    pub mac_binary_path: Option<PathBuf>,
    pub windows_binary_path: Option<PathBuf>,
}

pub struct JavaClass {
    file_name: String,
    class_import: String,
    file_content: String,
}

pub fn produce_java_classes(project_info: &ProjectInfo, ffi: &FFIStore) -> Vec<JavaClass> {
    let mut result = vec![];
    for class in ffi.get_classes() {
        let class_fields: Vec<String> = class
            .fields
            .iter()
            .map(|f| format!("\t{} {};", f.1, f.0))
            .collect();

        let file_content = JAVA_CLASS_TEMPLATE
            .replace("[[package_name]]", &project_info.java_package_name)
            .replace("[[java-class-name]]", &class.id)
            .replace("[[java-class-fields]]", &class_fields.join("\n"));

        result.push(JavaClass {
            file_name: format!("{}.java", &class.id),
            file_content,
            class_import: format!("import {}.{};", &project_info.java_package_name, &class.id),
        })
    }

    result
}

pub fn process_template(
    template: &str,
    project_info: &ProjectInfo,
    ffi: &FFIStore,
    java_clesses: &[JavaClass],
) -> String {
    let date = chrono::Local::now().to_utc();
    let release_date = date.format("%Y-%m-%d %H:%M:%S UTC").to_string();

    let class_imports: Vec<String> = java_clesses
        .iter()
        .map(|class| class.class_import.clone())
        .collect();
    let class_imports = class_imports.join("\n");

    template
        .replace("[[package_name]]", &project_info.java_package_name)
        .replace("[[class-imports]]", &class_imports)
        .replace("[[lib-name]]", &project_info.lib_name)
        .replace("[[lib-version]]", &project_info.lib_version)
        .replace("[[java-class-name]]", &project_info.get_java_class_name())
        .replace("[[lib-release-date]]", &release_date)
        .replace(
            "[[java-bind-methods]]",
            &ffi.get_methods()
                .into_iter()
                .map(|m| format!("\t{};", m.sig))
                .collect::<Vec<String>>()
                .join("\n"),
        )
}

fn copy_binary(
    resources_dir: &Path,
    binnary: Option<&PathBuf>,
    lib_name: &str,
) -> color_eyre::Result<()> {
    if let Some(binary_source) = binnary {
        let file_extension = binary_source
            .extension()
            .expect("Expected file with extensiton")
            .to_string_lossy();

        let binar_dist = resources_dir.join(format!("{lib_name}.{}", file_extension));
        fs::copy(binary_source, binar_dist).wrap_err("Failed to copy")?;
    }

    Ok(())
}

pub fn setup_java_project(
    project_dir: &Path,
    java_dir: &Path,
    project_info: &ProjectInfo,
) -> color_eyre::Result<()> {
    // Create directory
    let java_dir = create_or_get_dir(java_dir)?;
    let ffi_store = FFIStore::open_read_only(&consts::ffi_definitions_path(project_dir));
    let java_classes = produce_java_classes(project_info, &ffi_store);

    // Create pom
    create_file(
        &java_dir,
        "pom.xml",
        &process_template(POM_TEMPLATE, project_info, &ffi_store, &java_classes),
    )?;

    let src = create_or_get_dir(&java_dir.join("src"))?;
    let src_main = create_or_get_dir(&src.join("main"))?;
    let java_dir = create_or_get_dir(&src_main.join("java"))?;

    // Create packages structure
    let mut lib_java_class_directory = java_dir.clone();
    for dir_name in project_info.get_packages_path() {
        lib_java_class_directory = lib_java_class_directory.join(dir_name);
        create_or_get_dir(&lib_java_class_directory)?;
    }

    // Create java.class
    create_file(
        &lib_java_class_directory,
        &format!("{}.java", project_info.get_java_class_name()),
        &process_template(JAVA_LIB_TEMPLATE, project_info, &ffi_store, &java_classes),
    )?;

    // Create classes
    for class in java_classes.into_iter() {
        create_file(
            &lib_java_class_directory,
            &class.file_name,
            &class.file_content,
        )?;
    }

    Ok(())
}

pub fn build_jar(
    java_dir: &Path,
    project_info: &ProjectInfo,
    binary: &RustBinaryInfo,
) -> color_eyre::Result<()> {
    let src = create_or_get_dir(&java_dir.join("src"))?;
    let src_main = create_or_get_dir(&src.join("main"))?;
    let resources_dir = create_or_get_dir(&src_main.join("resources"))?;

    // Movie binaries to Java resource
    copy_binary(
        &resources_dir,
        binary.linux_binary_path.as_ref(),
        &project_info.lib_name,
    )?;
    copy_binary(
        &resources_dir,
        binary.mac_binary_path.as_ref(),
        &project_info.lib_name,
    )?;
    copy_binary(
        &resources_dir,
        binary.windows_binary_path.as_ref(),
        &project_info.lib_name,
    )?;

    // Build Jar

    // cli_utils::exec_command(&java_dir, "mvn clean install -U", "Clean cache")?;
    cli_utils::exec_command(java_dir, "mvn clean install compile assembly:single -U", "Jar")
}

fn get_file_if_exist(file: &Path) -> Option<PathBuf> {
    if file.is_dir() {
        return None;
    }

    if !file.exists() {
        return None;
    }

    Some(file.to_owned())
}

fn find_binary_file(dir: &Path, lib_name: &str, info: &mut RustBinaryInfo) {
    if info.linux_binary_path.is_none() {
        let linux_binary = dir.join(format!("lib{lib_name}.so"));
        info.linux_binary_path = get_file_if_exist(&linux_binary);
    }

    if info.windows_binary_path.is_none() {
        let win_binary = dir.join(format!("{lib_name}.dll"));
        let win_binary_2 = dir.join(format!("{}.dll", lib_name.replace("-", "_")));
        info.windows_binary_path = get_file_if_exist(&win_binary).or( get_file_if_exist(&win_binary_2));
    }

    if info.mac_binary_path.is_none() {
        let mac_binary = dir.join(format!("lib{lib_name}.dylib"));
        info.mac_binary_path = get_file_if_exist(&mac_binary);
    }
}

pub fn look_for_binary(target_dir: &Path, lib_name: &str, result: &mut RustBinaryInfo) {
    if !target_dir.exists() {
        return;
    }

    let release_dir = target_dir.join("release");
    let debug_dir = target_dir.join("debug");

    if let Ok(path) = release_dir.canonicalize() {
        find_binary_file(&path, lib_name, result);
    } else if let Ok(path) = debug_dir.canonicalize() {
        find_binary_file(&path, lib_name, result);
    };
}

pub fn find_native_lib(lib_name: &str, rust_project_path: &Path) -> RustBinaryInfo {
    let mut result = RustBinaryInfo::default();
    

    // x86 64

    let linux = rust_project_path.join("target").join("x86_64-unknown-linux-gnu");
    look_for_binary(&linux, lib_name, &mut result);

    let windows = rust_project_path.join("target").join("x86_64-pc-windows-gnu");
    look_for_binary(&windows, lib_name, &mut result);    
    
    let mac_os = rust_project_path.join("target").join("x86_64-apple-darwin");
    look_for_binary(&mac_os, lib_name, &mut result);    

    // ARM 64

    let linux = rust_project_path.join("target").join("aarch64-unknown-linux-gnu");
    look_for_binary(&linux, lib_name, &mut result);

    let mac_os = rust_project_path.join("target").join("aarch64-apple-darwin");
    look_for_binary(&mac_os, lib_name, &mut result);

    // System
    let system = rust_project_path.join("target");
    look_for_binary(&system, lib_name, &mut result);

    result
}

pub fn copy_jar_to(
    from_dir: &Path,
    to_dir: &Path,
    project_info: &ProjectInfo,
) -> color_eyre::Result<()> {
    let result_jar = from_dir
        .join("target")
        .join(project_info.jar_asymbly_name());

    if result_jar.exists() && result_jar.is_file() {
        let dist_jar = to_dir.join(project_info.jar_final_name());
        fs::remove_file(&dist_jar).ok();
        fs::copy(&result_jar, &dist_jar).ok();
    }

    Ok(())
}

#[cfg(test)]
pub mod test {
    use java_bindgen_core::project_info::ProjectInfo;
    use std::path::Path;

    #[test]
    fn should_setup_directory() {
        let project = ProjectInfo {
            java_package_name: "com.test".to_string(),
            lib_name: "myLib".to_string(),
            lib_version: "1.0.0".to_string(),
        };
        super::setup_java_project(
            &Path::new("."),
            &Path::new(".").join("target").join("setup-project-test"),
            &project,
        )
        .unwrap();
    }
}
