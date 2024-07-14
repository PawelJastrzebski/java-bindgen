#![allow(dead_code, unused_imports)]
use std::{str::FromStr, vec};

use java_bindgen_core::{
    consts::ffi_definitions_path,
    ffi_store::{FFIStore, JavaFFIClass},
    project_info::ProjectInfo,
};
use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{__private::TokenStream2, spanned::Spanned, Data, DeriveInput, Fields, Ident, Type};

use crate::{types_conversion::rewrite_rust_to_java, util::{self, CompileErrors}};

pub fn get_struct_fileds(fields: &Fields, errors: &mut CompileErrors) -> Vec<(Ident, Type)> {
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

pub fn produce_java_class_ffi_types(rust_types: &Vec<(Ident, Type)>, errors: &mut CompileErrors) -> Option<Vec<(String, String)>> {
    let mut java_types = vec![];
    for (name, ty) in rust_types {
        let Some(java_ty) = rewrite_rust_to_java(&ty.to_token_stream(), errors) else {
            return None;
        };
        java_types.push((name.to_string(), java_ty));
    }

    Some(java_types)
}

pub fn main(item: TokenStream) -> TokenStream {
    if let Ok(input) = syn::parse::<DeriveInput>(item.clone()) {
        let project_dir = std::path::Path::new(".");
        let mut errors = CompileErrors::default();
        let name = &input.ident;

        // Struct Guard
        let Data::Struct(struct_info) = input.data else {
            errors.add("Only struct is alowed.".to_string());
            return quote! { #errors }.into();
        };


        // Parse Cargo.toml file
        let cargo_toml = match util::parse_project_toml(&project_dir) {
            Ok(toml) => toml,
            Err(err) => {
                let error = util::error(input.ident.span(), err.to_string());
                return quote! { #error }.into();
            }
        };

        // Create project info
        let project_info = ProjectInfo::from(&cargo_toml);
        let fields = get_struct_fileds(&struct_info.fields, &mut errors);
        let Some(java_fields) = produce_java_class_ffi_types(&fields, &mut errors) else {
            return  quote! {
                #errors
            }.into()
        };

        if let Some(mut store) = FFIStore::read_from_file(&ffi_definitions_path(&project_dir)) {
            store.add_ffi_class(JavaFFIClass {
                id: name.to_string(),
                fields: java_fields,
            });
            store.save();
        }

        let (_, ty_generics, where_clause) = input.generics.split_for_impl();

        // rust to java type covertion
        let mut type_signature = quote! {};
        let mut args_conversion = quote! {};
        for (name, ty) in fields.into_iter() {
            args_conversion = quote! {
                #args_conversion
                self. #name .into_j_value(env)?.as_jni(),
            };

            if type_signature.is_empty() {
                type_signature = quote! { #ty };
            } else {
                type_signature = quote! { #type_signature, #ty };
            }
        }

        let mut class_path = project_info.get_packages_path();
        class_path.push(name.to_string());
        let class_path = class_path.join("/");
        let class_path = TokenStream2::from_str(&format!("\"{class_path}\"")).unwrap_or(quote! {});

        let result = quote! {

            #errors

            impl <'local> IntoJavaType<'local> for #name #ty_generics #where_clause {
                type JType = JObject<'local>;

                fn into_java(self, env: &mut jni::JNIEnv<'local>) -> JResult<Self::JType> {
                    let sig = signature_by_type!(#type_signature => JVoid);

                    let args = &[
                        #args_conversion
                    ];

                    if args.len() != sig.args.len() {
                        let e = JExceptionClass::ClassCastException;
                        env.j_throw_msg(&e, "Invalid implementation");
                        return Err(e.into());
                    }

                    let class = env.find_class(#class_path).j_catch(env)?;
                    let m_id = env.get_method_id(&class, "<init>", sig.to_string())?;
                    unsafe { env.new_object_unchecked(class, m_id, args).j_catch(env) }
                }
            }
        };

        return result.into();
    }

    quote! {}.into()
}
