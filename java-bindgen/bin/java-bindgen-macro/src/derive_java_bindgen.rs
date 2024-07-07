#![allow(dead_code, unused_imports)]
use java_bindgen_core::project_info::ProjectInfo;
use proc_macro::TokenStream;
use std::{path::Path, str::FromStr};
use quote::{format_ident, quote, ToTokens};
use syn::{Ident, ItemFn, ItemStruct, ReturnType, Visibility, __private::TokenStream2, spanned::Spanned};

use crate::util::{self, comment, parse_attr_to_map, parse_fn_args};

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
                }.into();
            },
        };
        // Create project info
        let project_info = ProjectInfo::from(&cargo_toml).set_package_name(&attribute.package);
        let _attr = parse_fn_args(&java_fn);
        let java_fn_name = format_ident!("{}", project_info.get_java_method_name(&java_fn.sig.ident.to_string()));
        let java_fn_args = quote! {};
        let java_fn_body = quote! {};
        let java_impl = quote! { 
            #[no_mangle]
            pub extern "system" fn #java_fn_name<'local>(
                mut env: jni::JNIEnv<'local>,
                _class: jni::objects::JClass<'local>,
                #java_fn_args
            ) {
                #java_fn_body
            } 
        };

        let res = quote! {
            #source
            #[automatically_derived]
            #java_impl
        };
        return res.into();
    
    }

    item
}