#![doc = include_str!("../README.md")]
#![forbid(unsafe_code, clippy::unwrap_used)]

#[doc(hidden)]
pub mod ffi_store;
#[doc(hidden)]
pub mod project_info;
#[doc(hidden)]
pub mod cargo_parser;
#[doc(hidden)]
pub mod utils;
#[doc(hidden)]
pub mod consts;