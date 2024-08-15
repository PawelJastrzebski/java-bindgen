mod checks;
mod cli_utils;
mod commands;
mod java_build_project;
mod java_templates;
mod java_test_project;

use std::{fs, path::{Path, PathBuf}};

use clap::{
    arg,
    builder::{
        styling::{AnsiColor, Color, Style},
        Styles,
    },
    command, value_parser, ColorChoice, Command,
};
use cli_utils::header;

const DOC_URL: &str = "https://crates.io/crates/java-pack";

pub fn print_exec_info() {
    let mut iter = std::env::args().into_iter();
    let mut path =  iter.next().map(|v| v.to_string()).unwrap_or_default();
    let args: Vec<String> = iter.map(|a| a.to_string()).collect();
    let command = format!("java-pack {}", args.join(" "));

    if cfg!(target_os = "linux") {
        if path.trim().is_empty() || path == "java-pack" {
            let (c, package_path, _) = cli_utils::exec_command_silent(Path::new("."), "which java-pack");
            if c == 0 {
                path = package_path;
            }
        }
    }

    cli_utils::print_exec_command_info("Executing: ", &command, &path);
}

pub fn cli() -> color_eyre::Result<()> {
    // Print command
    print_exec_info();

    // Setup cli
    let cli_style = Styles::styled().header(
        Style::new()
            .bold()
            .fg_color(Some(Color::Ansi(AnsiColor::Green))),
    );

    let command = command!()
        .color(ColorChoice::Always)
        .styles(cli_style)
        .arg(arg!([path] "Path to cargo project [OPTIONAL]").value_parser(value_parser!(PathBuf)))
        .arg(arg!(
            -d --debug "Turn debugging information on"
        ))
        .arg(arg!(
            -r --release "Release mode"
        ))
        .subcommand(Command::new("info").alias("i").about("Check project setup"))
        .subcommand(Command::new("build").alias("b").about("Build jar"))
        .subcommand(Command::new("jar").alias("j").alias("run").about("Run jar"))
        .subcommand(Command::new("test").alias("t").about("Run tests"))
        .subcommand(Command::new("deploy-local").about("Deploy jar to local maven repository"))
        .subcommand(
            Command::new("clean")
                .alias("clear")
                .alias("c")
                .about("Remove temp files [target/**]"),
        )
        .subcommand(Command::new("new-cargo").about("New cargo project"))
        .subcommand(
            Command::new("new-test")
                .alias("t")
                .alias("test-new")
                .about("Create Java test project"),
        );

    // Print help uti
    let mut command_help = command.clone();
    let mut print_help = move || {
        println!("{}", header("Help"));
        command_help.print_help().ok();
    };

    // Read args
    let matches = command.get_matches();
    let project_path = if let Some(config_path) = matches.get_one::<PathBuf>("path") {
        config_path.clone()
    } else {
        Path::new(".").to_owned()
    };
    let project_path = fs::canonicalize(&project_path).unwrap_or(project_path);

    let _debug_mode = matches.get_flag("debug");
    let release_mode = matches.get_flag("release");

    // Select Action
    let check_result = checks::CheckResult::check(&project_path);

    if let Some(_args) = matches.subcommand_matches("new-cargo") {
        commands::init_cargo_project(&project_path)?
    }

    if let Some(_args) = matches.subcommand_matches("new-test") {
        commands::setup_test_project(&project_path, release_mode)?
    }

    // Project config guard
    if matches.subcommand().is_some() && !check_result.is_ready() {
        let project_path_str = cli_utils::path_to_str(&project_path);

        println!("{}", header("Invalid Configuration"));
        println!("For additional details, please refer to the documentation.\n");
        println!("Docs: {DOC_URL}");
        println!("Project: {project_path_str}\n");
        check_result.print_status();
        return Ok(());
    }

    if let Some(_args) = matches.subcommand_matches("info") {
        check_result.print_status()
    }
    if let Some(_args) = matches.subcommand_matches("build") {
        commands::build(&project_path, release_mode)?;
    }
    if let Some(_args) = matches.subcommand_matches("test") {
        commands::run_tests(&project_path, release_mode)?
    }
    if let Some(_args) = matches.subcommand_matches("clean") {
        commands::clear(&project_path)?
    }
    if let Some(_args) = matches.subcommand_matches("jar") {
        commands::run_jar(&project_path, release_mode)?
    }
    if let Some(_args) = matches.subcommand_matches("deploy-local") {
        commands::deploy_local(&project_path, release_mode)?
    }

    if matches.subcommand().is_none() {
        print_help();
    }
    Ok(())
}