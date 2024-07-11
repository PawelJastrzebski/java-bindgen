use std::path::{Path, PathBuf};

use clap::{
    arg,
    builder::{
        styling::{AnsiColor, Color, Style},
        Styles,
    },
    command, value_parser, ColorChoice, Command,
};

use crate::utils::header;

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
        .arg(arg!([path] "path to cargo project (optional)").value_parser(value_parser!(PathBuf)))
        .arg(arg!(
            -d --debug "Turn debugging information on"
        ))
        .subcommand(Command::new("new-project").about("New cargo project"))
        .subcommand(
            Command::new("info")
                .alias("i")
                .about("Check project setup"),
        )
        .subcommand(Command::new("build").alias("b").about("Build jar"))
        .subcommand(Command::new("jar").alias("j").about("Run jar"))
        .subcommand(
            Command::new("clean")
                .alias("clear")
                .alias("c")
                .about("Delete build temp files [./target/**]"),
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
    let check_result = crate::checks::CheckResult::check(&project_path);

    if let Some(_args) = matches.subcommand_matches("new-project") {
        crate::commands::init_cargo_project(&project_path)?
    }

    // Project config guard
    if matches.subcommand().is_some() && !check_result.is_ready() {
        println!("{}", header("Project not ready"));
        println!("Go to the documentation for more information.\n");
        check_result.print_status();

        crate::utils::sleep(1000);
        print_help();
        return Ok(());
    }

    if let Some(_args) = matches.subcommand_matches("info") {
        check_result.print_status()
    }
    if let Some(_args) = matches.subcommand_matches("build") {
        crate::commands::build(&project_path)?;
    }
    if let Some(_args) = matches.subcommand_matches("clean") {
        crate::commands::clear(&project_path)?
    }
    if let Some(_args) = matches.subcommand_matches("jar") {
        crate::commands::run_jar(&project_path)?
    }

    if matches.subcommand().is_none() {
        print_help();
    }

    Ok(())
}
