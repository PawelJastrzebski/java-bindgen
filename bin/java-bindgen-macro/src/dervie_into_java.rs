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

use crate::{
    types_conversion::rewrite_rust_to_java,
    util::{self, CompileErrors},
};

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
        let cargo_toml = match util::parse_project_toml(project_dir) {
            Ok(toml) => toml,
            Err(err) => {
                let error = util::error(input.ident.span(), err.to_string());
                return quote! { #error }.into();
            }
        };

        // Create project info
        let project_info = ProjectInfo::from(&cargo_toml);
        let fields = crate::common::get_struct_fileds(&struct_info.fields, &mut errors);
        let Some(java_fields) = crate::common::produce_java_class_ffi_types(&fields, &mut errors) else {
            return quote! {
                #errors
            }
            .into();
        };

        if let Some(mut store) = FFIStore::read_from_file(&ffi_definitions_path(project_dir)) {
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
        let mut args_list = quote! {};
        for (i, (name, ty)) in fields.into_iter().enumerate() {
            let arg_name = format_ident!("a{i}");
            args_conversion = quote! {
                #args_conversion
                let #arg_name = self.#name.into_j_value(env)?;
                let #arg_name = #arg_name.borrow();
            };

            if type_signature.is_empty() {
                type_signature = quote! { #ty };
            } else {
                type_signature = quote! { #type_signature, #ty };
            }

            if args_list.is_empty() {
                args_list = quote! { #arg_name };
            } else {
                args_list = quote! { #args_list, #arg_name };
            }
        }

        let mut class_path = project_info.get_packages_path();
        class_path.push(name.to_string());
        let class_path = class_path.join("/");
        let class_path = TokenStream2::from_str(&format!("\"{class_path}\"")).unwrap_or(quote! {});

        return quote! {

            #errors

            impl <'local> IntoJavaType<'local, JObject<'local>> for #name #ty_generics #where_clause {
                fn into_java(self, env: &mut jni::JNIEnv<'local>) -> JResult<JObject<'local>> {
                    let sig = signature_by_type!(#type_signature => JVoid);

                    #args_conversion

                    let class = env.find_class(#class_path).j_catch(env)?;
                    env.new_object(class, sig.to_string(), &[#args_list]).j_catch(env)
                }
            }
        }.into()
    }

    quote! {}.into()
}
