#![doc = include_str!("../README.md")]

extern crate proc_macro;
use proc_macro::TokenStream;

mod util;
mod derive_java_bindgen;
mod derive_java_bindgen_raw;

#[proc_macro_attribute]
pub fn java_bindgen(attr: TokenStream, item: TokenStream) -> TokenStream {
    derive_java_bindgen::main(attr, item)
}

#[proc_macro_attribute]
pub fn java_bindgen_raw(attr: TokenStream, item: TokenStream) -> TokenStream {
    derive_java_bindgen_raw::main(attr, item)
}