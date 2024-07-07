use color_eyre::eyre::bail;
use java_bindgen_core::{cargo_parser::parse_toml, consts, project_info::ProjectInfo, utils::create_or_get_dir};
use std::path::{Path, PathBuf};

use crate::{java_project_stup, utils::{self, header}};

pub fn build(project_dir: &Path) -> color_eyre::Result<()> {
    let toml_path = consts::cargo_toml_path(&project_dir);
    let target_java_build = consts::java_build_dir(&project_dir);
    let target_path = create_or_get_dir(&project_dir.join("target"))?;

    let parse_toml = parse_toml(&toml_path);
    let toml = parse_toml?;
    let project_info = ProjectInfo::from(&toml);
    
    utils::exec_command(project_dir, "cargo build --color always", "Binary")?;
    utils::sleep(100);

    java_project_stup::setup_java_project(&project_dir, &target_java_build, &project_info)?;
    utils::sleep(100);

    let binary = java_project_stup::find_native_lib(&toml.package.name, &project_dir);
    java_project_stup::build_jar(&target_java_build, &project_info, &binary)?;
    utils::sleep(100);

    java_project_stup::copy_jar_to(&target_java_build, &target_path, &project_info)
}

pub(crate) fn clear(project_dir: &PathBuf) -> color_eyre::Result<()> {
    utils::exec_command(&project_dir, "cargo clean --color always", "Clean")
}

pub(crate) fn run_jar(project_dir: &PathBuf) -> color_eyre::Result<()> {
    let target_path = project_dir.join("target");
    let toml_path = consts::cargo_toml_path(&project_dir);

    let toml = parse_toml(&toml_path)?;
    let project_info = ProjectInfo::from(&toml);

    let jar_name = project_info.jar_final_name();
    let jar_name_path = target_path.join(&jar_name);
    if !jar_name_path.exists() {
        println!("{}", header("Jar not extist"));
        println!("Building new jar..\n");
        build(project_dir)?;
    }

    if !jar_name_path.exists() {
        bail!("Jar not exist")
    }

    let command = format!("java -jar ./{jar_name}");
    utils::exec_command(&target_path, &command, "Run jar")
}

pub(crate) fn init_cargo_project(_project_dir: &PathBuf) -> color_eyre::Result<()> {
    // let command = format!("cargo init --bin --color always");
    // utils::exec_command(&project_dir, &command, "Init Cargo Project")

    // todo
    Ok(())
}
