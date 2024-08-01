use color_eyre::eyre::bail;
use java_bindgen_core::{
    cargo_parser::parse_toml, consts, project_info::ProjectInfo, utils::create_or_get_dir,
};
use std::path::{Path, PathBuf};
use crate::cli::cli_utils::exit;
use super::{
    cli_utils::{self, header},
    java_build_project, java_test_project,
};

pub fn get_jar_path(
    project_dir: &Path,
    project_info: &ProjectInfo,
) -> (String, Option<PathBuf>) {
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

pub fn build(project_dir: &Path, release_mode: bool) -> color_eyre::Result<()> {
    let toml_path = consts::cargo_toml_path(project_dir);
    let target_java_build = consts::java_build_dir(project_dir);
    let target_path = create_or_get_dir(&project_dir.join("target"))?;

    let toml = parse_toml(&toml_path)?.toml_parsed;
    let project_info = ProjectInfo::from(&toml);

    if release_mode {
        cli_utils::exec_command(project_dir, "cargo build --color always --release", "Binary (release)")?;
    } else {
        cli_utils::exec_command(project_dir, "cargo build --color always", "Binary")?;
    }
    cli_utils::sleep(100);

    java_build_project::setup_java_project(project_dir, &target_java_build, &project_info)?;
    cli_utils::sleep(100);

    let binary =
        java_build_project::find_native_lib(&project_info.get_native_lib_name(), project_dir);
    java_build_project::build_jar(&target_java_build, &project_info, &binary)?;
    cli_utils::sleep(100);

    java_build_project::copy_jar_to(&target_java_build, &target_path, &project_info)
}

pub(crate) fn clear(project_dir: &Path) -> color_eyre::Result<()> {
    cli_utils::exec_command(project_dir, "cargo clean --color always", "Clean")
}

pub(crate) fn run_jar(project_dir: &Path, release_mode: bool) -> color_eyre::Result<()> {
    let target_path = project_dir.join("target");
    let toml_path = consts::cargo_toml_path(project_dir);

    let toml = parse_toml(&toml_path)?.toml_parsed;
    let project_info = ProjectInfo::from(&toml);

    let (_, jar) = get_jar_path(project_dir, &project_info);
    if jar.is_none() {
        println!("{}", header("Jar Not Exist"));
        println!("Building new jar..\n");
        build(project_dir, release_mode)?;
    }

    let (jar_name, jar) = get_jar_path(project_dir, &project_info);
    if jar.is_none() {
        bail!("Jar Not Exist")
    }

    let command = format!("java -jar ./{jar_name}");
    cli_utils::exec_command(&target_path, &command, "Run jar")
}

pub(crate) fn init_cargo_project(_project_dir: &Path) -> color_eyre::Result<()> {
    // let command = format!("cargo init --bin --color always");
    // utils::exec_command(&project_dir, &command, "Init Cargo Project")

    // todo
    Ok(())
}

pub(crate) fn setup_test_project(project_dir: &Path, release_mode: bool) -> color_eyre::Result<()> {
    let toml_path = consts::cargo_toml_path(project_dir);
    let toml = parse_toml(&toml_path)?.toml_parsed;
    let project_info = ProjectInfo::from(&toml);

    // Setup Java Project
    java_test_project::setup_tests_java_project(project_dir, &project_info)?;
    // Run tests
    run_tests(project_dir, release_mode)
}

pub(crate) fn run_tests(project_dir: &Path, release_mode: bool) -> color_eyre::Result<()> {
    let toml_path = consts::cargo_toml_path(project_dir);
    let toml = parse_toml(&toml_path)?.toml_parsed;
    let project_info = ProjectInfo::from(&toml);

    // Install Jar
    let (_, jar) = get_jar_path(project_dir, &project_info);
    if jar.is_none() {
        println!("{}", header("Jar Not Exist"));
        println!("Building new jar..\n");
        build(project_dir, release_mode)?;
    }

    let (_, jar) = get_jar_path(project_dir, &project_info);
    let Some(jar) = jar else {
        bail!("Jar Not Exist")
    };

    let test_dir = project_dir.join(project_info.tests_java_dir_name());
    if !test_dir.exists() {
        return Ok(());
    }

    java_test_project::install_jar(&test_dir, &jar, &project_info, "./local-maven-repo/")?;
    java_test_project::runt_tests(project_dir, &project_info)?;
    Ok(())
}

pub fn deploy_local(project_dir: &Path, release_mode: bool) -> color_eyre::Result<()> {
    let toml_path = consts::cargo_toml_path(project_dir);
    let toml = parse_toml(&toml_path)?.toml_parsed;
    let project_info = ProjectInfo::from(&toml);

    let Some(local_mvn_repo_dir) = toml.java_bindgen().unwrap_or_default().local_mvn_repository else {
        println!("{}", header("Invalid Configuration"));
        println!("Setup `local_mvn_repository` in [package.metadata.java-bindgen].");
        println!();
        println!("Example:");
        println!("[package.metadata.java-bindgen]");
        println!(r#"package = "{}""#, project_info.java_package_name);
        println!(r#"local_mvn_repository = "../local-maven-repo/" "#, );
        println!();
        return exit();
    };

    // Build prod
    build(project_dir, release_mode)?;

    let (_, jar) = get_jar_path(project_dir, &project_info);
    let Some(jar) = jar else {
        bail!("Jar not exist")
    };

    let local_mvn_repo_dir = create_or_get_dir(Path::new(&local_mvn_repo_dir))?;

    // Deploy jar to local repository
    java_test_project::install_jar(project_dir, &jar, &project_info, &local_mvn_repo_dir.to_string_lossy())?;

    Ok(())
}

