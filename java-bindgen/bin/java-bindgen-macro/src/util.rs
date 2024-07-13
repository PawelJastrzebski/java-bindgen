use proc_macro::TokenStream;
use quote::{quote, quote_spanned, ToTokens, TokenStreamExt};
use std::collections::HashMap;
use std::str::FromStr;
use syn::__private::TokenStream2;
use syn::{FnArg, ItemFn, Lifetime, PatType};

pub fn parse_attr_to_map(attr: TokenStream) -> HashMap<String, String> {
    let mut map: HashMap<String, String> = HashMap::new();
    for entry in attr.to_string().split(',') {
        let key_value = entry.split('=').collect::<Vec<&str>>();
        let key = key_value.get(0).unwrap_or(&"");
        let value = key_value.get(1).unwrap_or(&"");
        map.insert(
            key.trim().to_string(),
            value.trim().replace('\"', "").to_string(),
        );
    }
    map
}

pub fn error(span: syn::__private::Span, message: String) -> TokenStream2 {
    quote_spanned!( span => compile_error!(#message); )
}

pub struct CompileErrors {
    quote: TokenStream2,
}

impl Default for CompileErrors {
    fn default() -> Self {
        Self { quote: quote! {} }
    }
}

impl ToTokens for CompileErrors {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        tokens.append_all(self.quote.clone())
    }
}

#[allow(dead_code)]
impl CompileErrors {
    pub fn add(&mut self, message: String) {
        let message = format!("[java-bindgen]\n{message}\n");
        self.quote.append_all(quote!( compile_error!(#message); ))
    }
    pub fn add_spaned(&mut self, span: syn::__private::Span, message: String) {
        let message = format!("[java-bindgen]\n{message}\n");
        self.quote
            .append_all(quote_spanned!( span => compile_error!(#message); ))
    }
}

pub fn parse_fn_args(lazy_fn: &ItemFn) -> Vec<&PatType> {
    let mut fields = vec![];
    for t in lazy_fn.sig.inputs.iter() {
        if let FnArg::Typed(t) = t {
            fields.push(t)
        }
    }
    fields
}

pub fn comment(msg: &str) -> TokenStream2 {
    let comment = format!("const comment: &str = r###\"{msg}\"###;");
    TokenStream2::from_str(&comment).expect("comment")
}

pub fn ts2(tokens: &str) -> TokenStream2 {
    TokenStream2::from_str(&tokens).expect("valid TokenStream2")
}

pub fn parse_project_toml(
    project_dir: &std::path::Path,
) -> Result<java_bindgen_core::cargo_parser::CargoToml, String> {
    let toml_file_path = project_dir.join("Cargo.toml");
    match java_bindgen_core::cargo_parser::parse_toml(&toml_file_path) {
        Ok(info) => {
            let example =
                "\n\nExample:\n[package.java-bindgen.metadata]\npackage = \"com.java.package\"\n\n";
            if info.java_bindgen().is_none() {
                return Err(format!(
                    "Add java-bindgen metadata in your Cargo.toml file. {example}"
                ));
            }

            let config = info.java_bindgen().expect("To be checked");
            if config.package.is_none() {
                return Err(format!(
                    "Add: package = \"com.java.package\" in your Cargo.toml file.{example}"
                ));
            }

            Ok(info)
        }
        Err(err) => {
            return Err(err.to_string());
        }
    }
}


pub fn lifetime_from_type(ty: &syn::Type) -> Option<TokenStream2> {

    if let syn::Type::Path(syn::TypePath { ref path, .. }) = ty {

        for segment in &path.segments {
            if let syn::PathArguments::AngleBracketed(ref args) = &segment.arguments {
                for arg in &args.args {
                    if let syn::GenericArgument::Lifetime(lifetime) = arg {
                        return Some(quote! {<#lifetime>}.into());
                    }
                }
            }
        }
    }

    None
}