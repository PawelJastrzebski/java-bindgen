#![doc = include_str!("../README.md")]
#![forbid(unsafe_code, clippy::unwrap_used)]

#[doc(hidden)]
mod cli;

#[doc(hidden)]
pub fn main() -> color_eyre::Result<()> {
    cli::cli()
}
