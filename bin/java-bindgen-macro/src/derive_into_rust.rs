use java_bindgen_core::{
    consts::ffi_definitions_path,
    ffi_store::{FFIStore, JavaFFIClass},
};
use proc_macro::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{Data, DeriveInput};

use crate::util::{ts2, CompileErrors};

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

        // Create project info
        let fields = crate::common::get_struct_fileds(&struct_info.fields, &mut errors);
        let Some(java_fields) = crate::common::produce_java_class_ffi_types(&fields, &mut errors)
        else {
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

        let mut fields_getters = quote! {};
        for (name, ty) in fields.into_iter() {
            let getter_fn_name = name.to_string();
            let ty_string = ty.to_token_stream().to_string();

            let prefix = if ty_string == "bool" {
                "is"
            } else {
                "get"
            };

            let getter_name = format!(
                "\"{prefix}{}{}\"",
                &getter_fn_name[0..1].to_uppercase(),
                &getter_fn_name[1..]
            );
            let getter_name = ts2(&getter_name);
            fields_getters.append_all(quote! {
                #name: self.call_getter(#getter_name, env)?,
            })
        }

        return quote! {

            #errors

            impl<'local> java_bindgen::prelude::IntoRustType<'local, #name> for jni::objects::JObject<'local> {
                fn into_rust(self, env: &mut jni::JNIEnv<'local>) -> JResult<#name> {
                    Ok(#name {
                        #fields_getters
                    })
                }
            }

            impl<'local> java_bindgen::prelude::IntoRustType<'local, #name> for jni::objects::JValueGen<jni::objects::JObject<'local>> {
                fn into_rust(self, env: &mut jni::JNIEnv<'local>) -> JResult<#name> {
                    let obj = self.l()?;
                    obj.into_rust(env)
                }
            }

        }
        .into();
    }

    item
}
