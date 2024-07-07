use java_bindgen_core::{
    consts::ffi_definitions_path,
    ffi_store::{FFIStore, JavaFFI},
    project_info::ProjectInfo,
};
use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens, TokenStreamExt};
use syn::{
    __private::TokenStream2, punctuated::Punctuated, spanned::Spanned, FnArg, Pat, PathArguments,
    ReturnType, Token, Type,
};

use crate::util::{self, parse_attr_to_map, CompileErrors};

// determine java type based on Rust type
pub fn rewrite_type(type_token: &TokenStream2, errors: &mut CompileErrors) -> Option<String> {
    let rust_type = type_token.to_string();
    if rust_type.contains("JNIEnv") {
        return None;
    };
    if rust_type.contains("JClass") {
        return None;
    };
    if rust_type.contains("JString") {
        return Some("String".to_string());
    };
    if rust_type.contains("JObject") {
        return Some("Object".to_string());
    };
    if rust_type.contains("JByteArray") {
        return Some("byte[]".to_string());
    };

    errors.add_spaned(
        type_token.span(),
        format!("unsupported Java type - use jni types"),
    );
    None
}

// determine java type based on Rust type
pub fn to_java_type(rust_type: &Type, errors: &mut CompileErrors) -> Option<String> {
    let mut add_error = |msg: &str| {
        errors.add_spaned(rust_type.span(), format!("'{msg}' is not supported"));
        None
    };

    match rust_type {
        Type::Array(_) => add_error("array"),
        Type::BareFn(_) => add_error("bare function"),
        Type::Group(_) => add_error("group"),
        Type::ImplTrait(_) => add_error("impl"),
        Type::Infer(_) => None,
        Type::Macro(_) => None,
        Type::Never(_) => add_error("never type"),
        Type::Paren(_) => add_error("parenthesized type"),
        Type::Path(t) => rewrite_type(&t.to_token_stream(), errors),
        Type::Ptr(_) => add_error("*"),
        Type::Reference(_) => add_error("&"),
        Type::Slice(_) => add_error("slice"),
        Type::TraitObject(_) => add_error("trait object"),
        Type::Tuple(_) => add_error("tuple"),
        Type::Verbatim(_) => add_error("unknown token"),
        _ => add_error("unknown type"),
    }
}

// Java arguments list for FFI interface (Java side arguments list) 
pub fn produce_java_args(
    inputs: &Punctuated<FnArg, Token![,]>,
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

// Rust fn arguments for inner function call
pub fn produce_rust_args_names(
    inputs: &Punctuated<FnArg, Token![,]>,
    errors: &mut CompileErrors,
) -> TokenStream2 {
    let mut args = quote! {};
    for ele in inputs.iter() {
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
                if let Pat::Ident(ref patpath) = *typed.pat {
                    let name = patpath.ident.to_token_stream();
                    if args.is_empty() {
                        args.append_all(quote! { #name });
                    } else {
                        args.append_all(quote! {, #name });
                    }
                }
            }
        }
    }
    args
}

pub fn produce_rust_result_type(r_type: &ReturnType, errors: &mut CompileErrors) -> TokenStream2 {
    if let ReturnType::Type(_, r_type) = r_type {
        if let Type::Path(ref path) = **r_type {
            for segment in path.path.segments.iter() {
                if !segment.ident.to_string().contains("JResult") {
                    errors.add_spaned(
                        r_type.span(),
                        format!(
                            "Expected java_bindgen::JResult<{}>",
                            r_type.to_token_stream().to_string().replace(" ", "")
                        ),
                    );

                    return quote! { -> #r_type };
                }

                if let PathArguments::AngleBracketed(ref arg) = segment.arguments {
                    let inner_type = arg.args.to_token_stream();
                    return quote! { -> #inner_type};
                }
            }
        }
    }
    quote! {}
}

pub fn produce_java_return(return_type: &ReturnType, errors: &mut CompileErrors) -> String {
    match return_type {
        ReturnType::Default => Some("void".to_string()),
        ReturnType::Type(_, t) => rewrite_type(&t.to_token_stream(), errors),
    }
    .unwrap_or("void".to_string())
}

struct JavaBindgenAttr {
    pub package: String,
}

impl JavaBindgenAttr {
    pub fn parse_attr(attr: TokenStream) -> JavaBindgenAttr {
        let map = parse_attr_to_map(attr);
        JavaBindgenAttr {
            package: map.get("package").unwrap_or(&"".to_string()).clone(),
        }
    }
}

pub fn main(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut errors = CompileErrors::default();

    if let Ok(java_fn) = syn::parse::<syn::ItemFn>(item.clone()) {
        let source = TokenStream2::from(item.clone());
        let attribute = JavaBindgenAttr::parse_attr(attr.clone());
        let project_dir = std::path::Path::new(".");

        // Parse Cargo.toml file
        let cargo_toml = match util::parse_project_toml(&project_dir) {
            Ok(toml) => toml,
            Err(err) => {
                let error = util::error(java_fn.span(), err.to_string());
                return quote! {
                    #source
                    #error
                }
                .into();
            }
        };
        // Create project info
        let project_info = ProjectInfo::from(&cargo_toml).set_package_name(&attribute.package);
        let rust_fn_name = java_fn.sig.ident.to_string();
        let return_type = produce_rust_result_type(&java_fn.sig.output, &mut errors);

        if let Some(mut store) = FFIStore::read_from_file(&ffi_definitions_path(&project_dir)) {
            let return_type = syn::parse::<syn::ReturnType>(return_type.clone().into())
                .unwrap_or(ReturnType::Default);

            let args = produce_java_args(&java_fn.sig.inputs, &mut errors);
            let return_type = produce_java_return(&return_type, &mut errors);
            // TODO produce java FFI
            store.add_ffi(JavaFFI {
                id: rust_fn_name.clone(),
                sig: format!(
                    "public static native {} {}({})",
                    &return_type,
                    &rust_fn_name,
                    args.join(",")
                ),
            });
            store.save();
        }
        // Rewrite rust function
        let java_ffi_fn_name = format_ident!("{}", project_info.get_java_method_name(&rust_fn_name));
        let fn_body_inputs = java_fn.sig.inputs.to_token_stream();
        let fn_body_generics = java_fn.sig.generics.to_token_stream();
        let fn_name = java_fn.sig.ident.to_token_stream();
        let args_names = produce_rust_args_names(&java_fn.sig.inputs, &mut errors);

        let result = quote! {

            #errors

            #[allow(non_snake_case)]
            #source

            #[no_mangle]
            #[allow(unused_mut, non_snake_case)]
            pub extern "system" fn #java_ffi_fn_name #fn_body_generics(#fn_body_inputs) #return_type {

                match #fn_name(#args_names) {
                    Ok(ok) => ok,
                    Err(_) => Default::default(),
               }
            }
            
        };
        return result.into();
    }

    item
}
