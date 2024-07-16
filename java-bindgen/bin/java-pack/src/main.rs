#![doc = include_str!("../../../README.md")]
mod cli;

pub fn main() -> color_eyre::Result<()> {
    cli::cli()
}
