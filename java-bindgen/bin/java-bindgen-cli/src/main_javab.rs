mod commands;
mod checks;
mod cli;
mod java_project_stup;
mod utils;

pub fn main() -> color_eyre::Result<()> {
    cli::cli()
}
