use std::{
    fs::{self},
    path::{Path, PathBuf},
};

use color_eyre::eyre::Context;
use java_bindgen_core::{
    consts, ffi_store::FFIStore, project_info::ProjectInfo, utils::create_or_get_dir,
};

use crate::utils::{self};

#[derive(Debug, Default)]
pub struct RustBinaryInfo {
    pub linux_binary_path: Option<PathBuf>,
    pub mac_binary_path: Option<PathBuf>,
    pub windows_binary_path: Option<PathBuf>,
}

static JAVA_LIB_TEMPLATE: &str = include_str!("./java_templates/Lib.java.template");
static POM_TEMPLATE: &str = include_str!("./java_templates/pom.xml.template");

pub fn process_template(content: &str, project_info: &ProjectInfo, ffi: &FFIStore) -> String {
    let date = chrono::Local::now();
    let release_date = date.format("%Y-%m-%d %H:%M:%S").to_string();
    content
        .replace("[[package_name]]", &project_info.java_package_name)
        .replace("[[lib-name]]", &project_info.lib_name)
        .replace("[[lib-version]]", &project_info.lib_version)
        .replace("[[java-class-name]]", &project_info.get_java_class_name())
        .replace("[[lib-release-date]]", &release_date)
        .replace(
            "[[java-bind-methods]]",
            &ffi.get_all()
                .into_iter()
                .map(|m| format!("\t{};", m.sig))
                .collect::<Vec<String>>()
                .join("\n"),
        )
}

fn create_file(directory: &Path, file_name: &str, content: &str) -> color_eyre::Result<PathBuf> {
    let file_path = directory.join(file_name);
    fs::write(&file_path, content).wrap_err(format!("Failed to create {file_name} file "))?;
    Ok(file_path)
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
    let ffi_store = FFIStore::open_read_only(&consts::ffi_definitions_path(&project_dir));

    // Create pom
    create_file(
        &java_dir,
        "pom.xml",
        &process_template(POM_TEMPLATE, &project_info, &ffi_store),
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
        &process_template(JAVA_LIB_TEMPLATE, &project_info, &ffi_store),
    )?;

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

    utils::exec_command(&java_dir, "mvn compile assembly:single", "Jar")
}

fn get_file_if_exist(file: &Path) -> Option<PathBuf> {
    if file.is_dir() {
        return None;
    }

    if !file.exists() {
        return None;
    }

    return Some(file.to_owned());
}

fn find_lib(dir: &Path, lib_name: &str) -> RustBinaryInfo {
    let linux_binary = dir.join(format!("lib{lib_name}.so"));
    let windows_binary = dir.join(format!("lib{lib_name}.dll"));
    let mac_binary = dir.join(format!("lib{lib_name}.dylib"));

    RustBinaryInfo {
        linux_binary_path: get_file_if_exist(&&linux_binary),
        mac_binary_path: get_file_if_exist(&mac_binary),
        windows_binary_path: get_file_if_exist(&windows_binary),
    }
}

pub fn find_native_lib(lib_name: &str, rust_project_path: &Path) -> RustBinaryInfo {
    let target_dir = rust_project_path.join("target");
    if !target_dir.exists() {
        return RustBinaryInfo::default();
    }

    let release_dir = target_dir.join("release");
    let debug_dir = target_dir.join("debug");

    if let Ok(path) = release_dir.canonicalize() {
        return find_lib(&path, lib_name);
    };

    if let Ok(path) = debug_dir.canonicalize() {
        return find_lib(&path, lib_name);
    };

    RustBinaryInfo::default()
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
