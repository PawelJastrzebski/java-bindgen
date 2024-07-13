use java_bindgen_core::{
    consts::ffi_definitions_path,
    ffi_store::{FFIStore, JavaFFIMethod},
    project_info::ProjectInfo,
};
use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens, TokenStreamExt};
use syn::{
    __private::TokenStream2, punctuated::Punctuated, spanned::Spanned, FnArg, ReturnType, Token, Type,
};

use crate::{
    common::{produce_java_args, produce_java_return, produce_rust_args_names, produce_rust_result_type}, types_conversion::rewrite_rust_type_to_jni, util::{self, parse_attr_to_map, ts2, CompileErrors}
};


// Extract lifetime form args
pub fn extract_jni_env_lifetime(
    inputs: &Punctuated<FnArg, Token![,]>,
    errors: &mut CompileErrors,
) -> Option<TokenStream2> {
    for ele in inputs.iter() {
        if let FnArg::Typed(typed) = ele {
            let ty_str = typed.ty.to_token_stream().to_string().replace(" ", "");
            if !ty_str.contains("JNIEnv") {
                continue;
            }

            // errors.add_spaned(typed.span(), format!("YEs {}\n {}", ty_str, typed.ty.to_token_stream().to_string()));

            match *typed.ty.clone() {
                Type::Array(_) => {
                    errors.add_spaned(
                        typed.span(),
                        format!(
                            "YEs Array {}\n {}",
                            ty_str,
                            typed.ty.to_token_stream().to_string()
                        ),
                    );
                }
                Type::BareFn(_) => {
                    errors.add_spaned(
                        typed.span(),
                        format!(
                            "YEs Fn {}\n {}",
                            ty_str,
                            typed.ty.to_token_stream().to_string()
                        ),
                    );
                }
                Type::Group(_) => {
                    errors.add_spaned(
                        typed.span(),
                        format!(
                            "YEs Group {}\n {}",
                            ty_str,
                            typed.ty.to_token_stream().to_string()
                        ),
                    );
                }
                Type::ImplTrait(_) => {
                    errors.add_spaned(
                        typed.span(),
                        format!(
                            "YEs Trait {}\n {}",
                            ty_str,
                            typed.ty.to_token_stream().to_string()
                        ),
                    );
                }
                Type::Infer(_) => {
                    errors.add_spaned(
                        typed.span(),
                        format!(
                            "YEs Infer {}\n {}",
                            ty_str,
                            typed.ty.to_token_stream().to_string()
                        ),
                    );
                }
                Type::Macro(_) => {
                    errors.add_spaned(
                        typed.span(),
                        format!(
                            "YEs Macro {}\n {}",
                            ty_str,
                            typed.ty.to_token_stream().to_string()
                        ),
                    );
                }
                Type::Never(_) => {
                    errors.add_spaned(
                        typed.span(),
                        format!(
                            "YEs Never {}\n {}",
                            ty_str,
                            typed.ty.to_token_stream().to_string()
                        ),
                    );
                }
                Type::Paren(_) => {
                    errors.add_spaned(
                        typed.span(),
                        format!(
                            "YEs Paren {}\n {}",
                            ty_str,
                            typed.ty.to_token_stream().to_string()
                        ),
                    );
                }
                Type::Path(path) => {
                    errors.add_spaned(
                        typed.span(),
                        format!(
                            "YEs Path {}\n {}",
                            ty_str,
                            typed.ty.to_token_stream().to_string()
                        ),
                    );
                }
                Type::Ptr(_) => {
                    errors.add_spaned(
                        typed.span(),
                        format!(
                            "YEs Ptr {}\n {}",
                            ty_str,
                            typed.ty.to_token_stream().to_string()
                        ),
                    );
                }
                Type::Reference(refer) => {
                    if let Some(l) = util::lifetime_from_type(&refer.elem) {
                        return Some(l);
                    }

                    errors.add_spaned(
                        typed.span(),
                        format!(
                            "YEs Reference {}\n {}",
                            ty_str,
                            typed.ty.to_token_stream().to_string()
                        ),
                    );
                }
                Type::Slice(_) => {
                    errors.add_spaned(
                        typed.span(),
                        format!(
                            "YEs Slice {}\n {}",
                            ty_str,
                            typed.ty.to_token_stream().to_string()
                        ),
                    );
                }
                Type::TraitObject(_) => {
                    errors.add_spaned(
                        typed.span(),
                        format!(
                            "YEs Object {}\n {}",
                            ty_str,
                            typed.ty.to_token_stream().to_string()
                        ),
                    );
                }
                Type::Tuple(_) => {
                    errors.add_spaned(
                        typed.span(),
                        format!(
                            "YEs Tuple {}\n {}",
                            ty_str,
                            typed.ty.to_token_stream().to_string()
                        ),
                    );
                }
                Type::Verbatim(_) => {
                    errors.add_spaned(
                        typed.span(),
                        format!(
                            "YEs Verbatim {}\n {}",
                            ty_str,
                            typed.ty.to_token_stream().to_string()
                        ),
                    );
                }
                _ => {
                    errors.add_spaned(
                        typed.span(),
                        format!(
                            "YEs -- {}\n {}",
                            ty_str,
                            typed.ty.to_token_stream().to_string()
                        ),
                    );
                }
            };
        }
    }
    None
}

struct JavaFnSig {
    args: TokenStream2,
    env_indent: TokenStream2,
    into_rust_ident: Vec<TokenStream2>,
    jni_env_lifetime: TokenStream2,
}


// Rust fn arguments for inner function call
fn produce_fn_java_args_signature(
    inputs: &Punctuated<FnArg, Token![,]>,
    errors: &mut CompileErrors,
) -> JavaFnSig {
    let jni_env_lifetime = extract_jni_env_lifetime(&inputs, errors).unwrap_or(quote! { <'l> });
    let mut into_rust_ident = vec![];
    let mut env_indent = quote! { env };
    let mut jni_env = quote! { mut env: JNIEnv #jni_env_lifetime };
    let mut jni_class = quote! { _classs: JClass #jni_env_lifetime };
    let mut args = quote! {};

    // TODO !! clean code

    for (i, ele) in inputs.iter().enumerate() {
        match ele {
            FnArg::Receiver(r) => {
                errors.add_spaned(
                    r.span(),
                    "'self' parameter is not supported. Use functions for defining java bindings."
                        .into(),
                );
                args.append_all(quote! { self, #args });
            }
            FnArg::Typed(typed) => {
                // Rewrite [Rust] to [RustJNI]
                let ty = if let Some(ty) =
                    rewrite_rust_type_to_jni(&typed.ty.to_token_stream(), &jni_env_lifetime, errors)
                {
                    into_rust_ident.push(typed.pat.to_token_stream());
                    ty
                } else {
                    typed.ty.to_token_stream()
                };
                let type_string = ty.to_token_stream().to_string().replace(" ", "");

                let pat = &typed.pat;
                let pat_string = pat.to_token_stream().to_string().replace(" ", "");

                if type_string.contains("JNIEnv") {
                    if !type_string.contains("&mut") {
                        errors.add_spaned(
                            typed.span(),
                            "'JNIEnv' must be declared as mutable reference: &mut JNIEnv<'a>"
                                .into(),
                        );
                    }
                    if pat_string.trim() == "_" {
                        env_indent = format_ident!("arg{i}").to_token_stream()
                    }

                    let jni_env_type = ts2(&type_string.replace("&mut", ""));
                    jni_env = quote! { mut #env_indent : #jni_env_type };
                    continue;
                }

                if type_string.contains("JClass") {
                    jni_class = quote! { #pat : #ty };
                    continue;
                }
                args.append_all(quote! { , #pat : #ty  });
            }
        }
    }

    JavaFnSig {
        args: quote! { #jni_env, #jni_class #args },
        env_indent,
        jni_env_lifetime,
        into_rust_ident,
    }
}

struct JavaBindgenAttr {
    pub package: String,
    pub returns: Option<String>,
}

impl JavaBindgenAttr {
    pub fn parse_attr(attr: TokenStream) -> JavaBindgenAttr {
        let map = parse_attr_to_map(attr);
        JavaBindgenAttr {
            package: map.get("package").unwrap_or(&"".to_string()).clone(),
            returns: map
                .get("return")
                .cloned()
                .or_else(|| map.get("returns").cloned()),
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
            let return_type = attribute
                .returns
                .unwrap_or_else(|| produce_java_return(&return_type, &mut errors));
            store.add_ffi_method(JavaFFIMethod {
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
        let j_ffi_fn_name = format_ident!("{}", project_info.get_java_method_name(&rust_fn_name));
        let fn_name = java_fn.sig.ident.to_token_stream();
        let args_names = produce_rust_args_names(&java_fn.sig.inputs, &mut errors);

        let JavaFnSig {
            args,
            env_indent,
            jni_env_lifetime,
            into_rust_ident,
        } = produce_fn_java_args_signature(&java_fn.sig.inputs, &mut errors);

        // Types conversion
        let mut rewrites = quote! {};
        for indent in into_rust_ident {
            rewrites.append_all(quote! {

                let Ok(#indent) = #indent.into_rust(&mut env) else {
                    return JObject::null().as_raw()
                };

            });
        }

        return quote! {

            #errors

            #[allow(non_snake_case)]
            #source

            #[no_mangle]
            #[allow(unused_mut, non_snake_case)]
            pub extern "system" fn #j_ffi_fn_name #jni_env_lifetime(#args) -> jni::sys::jobject {

                #rewrites

                let r = #fn_name(#args_names);
                j_result_handler(r, &mut #env_indent).as_raw()
            }

        }.into()
    }

    item
}