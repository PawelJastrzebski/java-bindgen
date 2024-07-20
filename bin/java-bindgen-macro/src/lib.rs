#![doc = include_str!("../../../README.md")]
#![forbid(unsafe_code, clippy::unwrap_used)]

extern crate proc_macro;
use proc_macro::TokenStream;

#[doc(hidden)]
mod util;
#[doc(hidden)]
mod common;
mod derive_java_bindgen;
mod dervie_into_java;
mod derive_into_rust;
mod dervie_java_type;
mod derive_jlogger;
mod derive_test_jvm;
mod types_conversion;

#[proc_macro_attribute]
pub fn java_bindgen(attr: TokenStream, item: TokenStream) -> TokenStream {
    derive_java_bindgen::main(attr, item)
}

#[proc_macro_derive(IntoJava)]
pub fn java_bindgen_into_java(item: TokenStream) -> TokenStream {
    dervie_into_java::main(item)
}

#[proc_macro_derive(IntoRust)]
pub fn java_bindgen_into_rust(item: TokenStream) -> TokenStream {
    derive_into_rust::main(item)
}

#[proc_macro_derive(JavaType)]
pub fn java_bindgen_java_type(item: TokenStream) -> TokenStream {
    dervie_java_type::main(item)
}

#[proc_macro_derive(JLogger)]
pub fn java_bindgen_jlogger(item: TokenStream) -> TokenStream {
    derive_jlogger::main(item)
}

#[proc_macro_attribute]
pub fn test_jvm(attr: TokenStream, item: TokenStream) -> TokenStream {
    derive_test_jvm::main(attr, item)
}