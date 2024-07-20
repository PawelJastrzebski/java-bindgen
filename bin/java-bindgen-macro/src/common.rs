use quote::format_ident;
use quote::quote;
use quote::ToTokens;
use quote::TokenStreamExt;
use syn::FnArg;
use syn::Type;
use syn::__private::TokenStream2;
use syn::{spanned::Spanned, ReturnType};

use crate::{types_conversion::rewrite_rust_to_java, util::CompileErrors};

// Java arguments list for FFI interface (Java side arguments list) 
pub fn produce_java_args(
    inputs: &syn::punctuated::Punctuated<syn::FnArg, syn::Token![,]>,
    errors: &mut CompileErrors,
) -> Vec<String> {
    let mut args = vec![];
    for ele in inputs.iter() {
        match ele {
            FnArg::Receiver(_) => {}
            FnArg::Typed(typed) => {
                if let Some(java_type) = to_java_type(&typed.ty, errors) {
                    let name = typed.pat.to_token_stream().to_string();
                    args.push(format!("{java_type} {name}"))
                }
            }
        }
    }
    args
}

pub fn produce_java_return(return_type: &TokenStream2, errors: &mut CompileErrors) -> String {
    if let Some(new_type) = rewrite_rust_to_java(return_type, errors) {
        return new_type;
    }

    "void".to_string()
}

pub fn produce_rust_result_type(r_type: &ReturnType, errors: &mut CompileErrors) -> TokenStream2 {
    if let ReturnType::Type(_, r_type) = r_type {
        if let syn::Type::Path(ref path) = **r_type {
            for segment in path.path.segments.iter() {
                if !segment.ident.to_string().contains("JResult") {
                    errors.add_spaned(
                        r_type.span(),
                        format!(
                            "Expected java_bindgen::JResult<{}>",
                            r_type.to_token_stream().to_string().replace(' ', "")
                        ),
                    );

                    return quote! { #r_type };
                }

                if let syn::PathArguments::AngleBracketed(ref arg) = segment.arguments {
                    let inner_type = arg.args.to_token_stream();
                    return quote! { #inner_type};
                }
            }
        }
    }
    quote! {}
}


// Rust fn arguments for inner function call
pub fn produce_rust_args_names(
    inputs: &syn::punctuated::Punctuated<FnArg, syn::Token![,]>,
    errors: &mut CompileErrors,
) -> TokenStream2 {
    let mut args = quote! {};
    for (i, ele) in inputs.iter().enumerate() {
        match ele {
            FnArg::Receiver(r) => {
                errors.add_spaned(
                    r.span(),
                    "'self' parameter is not supported. Use functions for defining java bindings."
                        .into(),
                );
                args.append_all(quote! { self });
            }
            FnArg::Typed(typed) => {
                let type_string = typed.ty.to_token_stream().to_string().replace(' ', "");
                let mut is_mute = quote! {};
                if type_string.contains("&mut") {
                    is_mute = quote! { &mut }
                }

                let name = if let syn::Pat::Ident(ref patpath) = *typed.pat {
                    patpath.ident.clone()
                } else {
                    format_ident!("arg{i}")
                };

                if args.is_empty() {
                    args.append_all(quote! { #is_mute #name });
                } else {
                    args.append_all(quote! {, #is_mute #name });
                }
            }
        }
    }
    args
}


// determine java type based on Rust type
pub fn to_java_type(rust_type: &Type, errors: &mut CompileErrors) -> Option<String> {
    let mut add_error = |msg: &str| {
        errors.add_spaned(rust_type.span(), format!("'{msg}' is not supported"));
        None
    };

    let rust_type_str = rust_type.to_token_stream().to_string().replace(' ', "");
    match rust_type {
        Type::Array(_) => add_error("array"),
        Type::BareFn(_) => add_error("bare function"),
        Type::Group(_) => add_error("group"),
        Type::ImplTrait(_) => add_error("impl"),
        Type::Infer(_) => None,
        Type::Macro(_) => None,
        Type::Never(_) => add_error("never type"),
        Type::Paren(_) => add_error("parenthesized type"),
        Type::Path(t) => rewrite_rust_to_java(&t.to_token_stream(), errors),
        Type::Ptr(_) => add_error("*"),
        Type::Reference(_) => {
            if rust_type_str.contains("JNIEnv") {
                return None;
            }
            add_error("&")
        },
        Type::Slice(_) => add_error("slice"),
        Type::TraitObject(_) => add_error("trait object"),
        Type::Tuple(_) => add_error("tuple"),
        Type::Verbatim(_) => add_error("unknown token"),
        _ => add_error("unknown type"),
    }
}


pub fn get_struct_fileds(fields: &syn::Fields, errors: &mut CompileErrors) -> Vec<(syn::Ident, Type)> {
    let mut result = vec![];
    for field in fields.iter() {
        let Some(ref name) = field.ident else {
            errors.add_spaned(
                field.span(),
                "Fields with no names are not supported.".to_string(),
            );
            continue;
        };
        result.push((name.clone(), field.ty.clone()));
    }
    result
}

pub fn produce_java_class_ffi_types(
    rust_types: &Vec<(syn::Ident, Type)>,
    errors: &mut CompileErrors,
) -> Option<Vec<(String, String)>> {
    let mut java_types = vec![];
    for (name, ty) in rust_types {
        let Some(java_ty) = crate::types_conversion::rewrite_rust_to_java(&ty.to_token_stream(), errors) else {
            continue;
        };
        java_types.push((name.to_string(), java_ty));
    }

    Some(java_types)
}
