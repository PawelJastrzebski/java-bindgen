#![doc = include_str!("../../../README.md")]

extern crate proc_macro;
use proc_macro::TokenStream;

#[doc(hidden)]
mod util;
#[doc(hidden)]
mod common;
#[doc(hidden)]
mod derive_java_bindgen_raw;
mod derive_java_bindgen;
mod dervie_into_java;
mod derive_into_rust;
mod derive_jlogger;
mod derive_test_jvm;
mod types_conversion;

#[proc_macro_attribute]
pub fn java_bindgen(attr: TokenStream, item: TokenStream) -> TokenStream {
    derive_java_bindgen::main(attr, item)
}

#[proc_macro_attribute]
pub fn java_bindgen_raw(attr: TokenStream, item: TokenStream) -> TokenStream {
    derive_java_bindgen_raw::main(attr, item)
}

#[proc_macro_derive(IntoJava)]
pub fn java_bindgen_into_java(item: TokenStream) -> TokenStream {
    dervie_into_java::main(item)
}

#[proc_macro_derive(IntoRust)]
pub fn java_bindgen_into_rust(item: TokenStream) -> TokenStream {
    derive_into_rust::main(item)
}

#[proc_macro_derive(JLogger)]
pub fn java_bindgen_jlogger(item: TokenStream) -> TokenStream {
    derive_jlogger::main(item)
}

#[proc_macro_attribute]
pub fn test_jvm(attr: TokenStream, item: TokenStream) -> TokenStream {
    derive_test_jvm::main(attr, item)
}