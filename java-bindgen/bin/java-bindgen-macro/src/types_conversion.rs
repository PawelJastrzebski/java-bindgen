use crate::util::CompileErrors;
use syn::{__private::TokenStream2, spanned::Spanned};
use quote::quote;

// rewrite [JNI Rust] to [Java Type]
pub fn rewrite_jni_to_java(ty: &TokenStream2, errors: &mut CompileErrors) -> Option<String> {
    let rust_type = ty.to_string();
    // jni primitives
    if rust_type.contains("jbyte") {
        return Some("byte".to_string());
    };
    if rust_type.contains("jchar") {
        return Some("char".to_string());
    };
    if rust_type.contains("jboolean") {
        return Some("boolean".to_string());
    };
    if rust_type.contains("jint") {
        return Some("int".to_string());
    };
    if rust_type.contains("jshort") {
        return Some("short".to_string());
    };
    if rust_type.contains("jlong") {
        return Some("long".to_string());
    };
    if rust_type.contains("jfloat") {
        return Some("float".to_string());
    };
    if rust_type.contains("double") {
        return Some("double".to_string());
    };

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

    // jni class primitives
    if rust_type.contains("JByte") {
        return Some("Byte".to_string());
    };
    if rust_type.contains("String") {
        return Some("String".to_string());
    };

    errors.add_spaned(ty.span(), format!("unsupported Java type. (use jni types)"));
    None
}

// rewrite [Rust Type] to [Java Type]
pub fn rewrite_rust_to_java(ty: &TokenStream2, errors: &mut CompileErrors) -> Option<String> {
    let rust_type = ty.to_string().replace(" ", "");
    // primitives
    if rust_type == "u8" {
        return Some("byte".to_string());
    };
    if rust_type == "i16" {
        return Some("short".to_string());
    };
    if rust_type == "i32" {
        return Some("int".to_string());
    };
    if rust_type == "i64" {
        return Some("long".to_string());
    };
    if rust_type == "f32" {
        return Some("float".to_string());
    };
    if rust_type == "f64" {
        return Some("double".to_string());
    };

    // class primitive wrappers
    if rust_type.contains("JByte") {
        return Some("Byte".to_string());
    };
    if rust_type.contains("JShort") {
        return Some("Short".to_string());
    };
    if rust_type.contains("JInt") {
        return Some("Integer".to_string());
    };
    if rust_type.contains("JLong") {
        return Some("Long".to_string());
    };
    if rust_type.contains("JFloat") {
        return Some("Float".to_string());
    };
    if rust_type.contains("JDouble") {
        return Some("Double".to_string());
    };
    if rust_type.contains("JBoolean") {
        return Some("Boolean".to_string());
    };
    if rust_type.contains("JChar") {
        return Some("Character".to_string());
    };

    // string
    if rust_type.contains("String") {
        return Some("String".to_string());
    };

    // arrays
    if rust_type.contains("Vec<u8>") {
        return Some("byte[]".to_string());
    };

    errors.add_spaned(ty.span(), format!("unsupported type. {}", rust_type));
    None
}

// Revirte [Rust Type] to [JNI Rust]
pub fn rewrite_rust_type_to_jni(
    ty: &TokenStream2,
    lifetime: &TokenStream2,
    errors: &mut CompileErrors,
) -> Option<TokenStream2> {
    let rust_type = ty.to_string().replace(" ", "");
    // primitives
    if rust_type == "String" {
        return Some(quote! { jni::objects::JString #lifetime })
    };
    None
}
