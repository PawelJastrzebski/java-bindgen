use java_bindgen_core::{
    consts::ffi_definitions_path,
    ffi_store::{FFIStore, JavaFFIMethod},
    project_info::ProjectInfo,
};
use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{
    __private::TokenStream2, spanned::Spanned, ReturnType,
};

use crate::{common::{produce_java_args, produce_java_return, produce_rust_args_names, produce_rust_result_type}, util::{self, parse_attr_to_map, CompileErrors}};


struct JavaBindgenAttr {
    pub package: String,
    pub returns: Option<String>,
}

impl JavaBindgenAttr {
    pub fn parse_attr(attr: TokenStream) -> JavaBindgenAttr {
        let map = parse_attr_to_map(attr);
        JavaBindgenAttr {
            package: map.get("package").unwrap_or(&"".to_string()).clone(),
            returns: map.get("return").cloned().or_else(|| map.get("returns").cloned())
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
            let return_type = attribute.returns.unwrap_or_else(|| produce_java_return(&return_type, &mut errors));
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
