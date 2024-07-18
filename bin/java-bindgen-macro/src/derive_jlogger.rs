use java_bindgen_core::project_info::ProjectInfo;
use proc_macro::TokenStream;
use quote::quote;
use crate::util::{self, CompileErrors};


pub fn main(item: TokenStream) -> TokenStream {
    if let Ok(input) = syn::parse::<syn::DeriveInput>(item.clone()) {
        let project_dir = std::path::Path::new(".");
        let mut errors = CompileErrors::default();

        // Struct Guard
        let syn::Data::Struct(_) = input.data else {
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
        let struct_name = &input.ident;

        let package_path = project_info.get_packages_path().join("/");
        let class_path = format!("\"{package_path}/{}\"", project_info.get_java_class_name());
        let class_path = util::ts2(&class_path);

        return quote! {
            impl #struct_name {
                fn init<'local>(env: &mut jni::JNIEnv<'local>) -> java_bindgen::logger::JLoggerCore<'local> {
                    java_bindgen::logger::JLoggerCore::new(env, #class_path).unwrap_or_default()
                }
            }
        }.into()

    }
    item
}
