#![doc = include_str!("../../../README.md")]

#[doc(hidden)]
mod cli;

#[doc(hidden)]
pub fn main() -> color_eyre::Result<()> {
    cli::cli()
}
