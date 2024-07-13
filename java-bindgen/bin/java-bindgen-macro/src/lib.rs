#![doc = include_str!("../README.md")]

extern crate proc_macro;
use proc_macro::TokenStream;

mod util;
mod common;
mod derive_java_bindgen;
mod derive_java_bindgen_raw;
mod derive_java_bindgen_raw2;
mod dervie_into_java;
mod types_conversion;

#[proc_macro_attribute]
pub fn java_bindgen(attr: TokenStream, item: TokenStream) -> TokenStream {
    derive_java_bindgen::main(attr, item)
}

#[proc_macro_attribute]
pub fn java_bindgen_raw(attr: TokenStream, item: TokenStream) -> TokenStream {
    derive_java_bindgen_raw::main(attr, item)
}

#[proc_macro_attribute]
pub fn java_bindgen_raw2(attr: TokenStream, item: TokenStream) -> TokenStream {
    derive_java_bindgen_raw2::main(attr, item)
}

#[proc_macro_derive(IntoJava)]
pub fn java_bindgen_into_java(item: TokenStream) -> TokenStream {
    dervie_into_java::main(item)
}