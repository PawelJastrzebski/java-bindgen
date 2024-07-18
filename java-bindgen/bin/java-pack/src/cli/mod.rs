mod checks;
mod cli_utils;
mod commands;
mod java_build_project;
mod java_templates;
mod java_test_project;

use std::path::{Path, PathBuf};

use clap::{
    arg,
    builder::{
        styling::{AnsiColor, Color, Style},
        Styles,
    },
    command, value_parser, ColorChoice, Command,
};
use cli_utils::header;

pub fn cli() -> color_eyre::Result<()> {
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
        .subcommand(Command::new("info").alias("i").about("Check project setup"))
        .subcommand(Command::new("build").alias("b").about("Build jar"))
        .subcommand(Command::new("jar").alias("j").about("Run jar"))
        .subcommand(Command::new("test").alias("t").about("Run tests"))
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
                .about("Create Java test project"),
        );

    // Print help uti
    let mut comand_help = command.clone();
    let mut print_help = move || {
        println!("{}", header("Help"));
        comand_help.print_help().ok();
    };

    // Read args
    let matches = command.get_matches();
    let project_path = if let Some(config_path) = matches.get_one::<PathBuf>("path") {
        config_path.clone()
    } else {
        Path::new(".").to_owned()
    };

    let _debug_mode = matches.get_flag("debug");

    // Select Action
    let check_result = checks::CheckResult::check(&project_path);

    if let Some(_args) = matches.subcommand_matches("new-cargo") {
        commands::init_cargo_project(&project_path)?
    }

    if let Some(_args) = matches.subcommand_matches("new-test") {
        commands::setup_test_project(&project_path)?
    }

    // Project config guard
    if matches.subcommand().is_some() && !check_result.is_ready() {
        println!("{}", header("Project not ready"));
        println!("Go to the documentation for more information.\n");
        check_result.print_status();

        cli_utils::sleep(1000);
        print_help();
        return Ok(());
    }

    if let Some(_args) = matches.subcommand_matches("info") {
        check_result.print_status()
    }
    if let Some(_args) = matches.subcommand_matches("build") {
        commands::build(&project_path)?;
    }
    if let Some(_args) = matches.subcommand_matches("test") {
        commands::run_tests(&project_path)?
    }
    if let Some(_args) = matches.subcommand_matches("clean") {
        commands::clear(&project_path)?
    }
    if let Some(_args) = matches.subcommand_matches("jar") {
        commands::run_jar(&project_path)?
    }

    if matches.subcommand().is_none() {
        print_help();
    }

    Ok(())
}