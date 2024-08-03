use crate::util::{ts2, CompileErrors};
use quote::quote;
use syn::__private::TokenStream2;

// Extract T from JList<T>
pub fn to_java_list(rust_type: String, errors: &mut CompileErrors) -> String {
    let default = "List<Object>".to_string();
    let Some(split_index) = rust_type.find('<') else {
        return default;
    };
    let (_, right) = rust_type.split_at(split_index + 1);

    let Some(split_index) = right.rfind('>') else {
        return default;
    };
    let (ty, _) = right.split_at(split_index);

    if ty == "u8" || ty == "i8" {
        return "List<Byte>".to_string();
    }
    if ty == "i16" {
        return "List<Short>".to_string();
    }
    if ty == "i32" {
        return "List<Integer>".to_string();
    }
    if ty == "i64" {
        return "List<Long>".to_string();
    }
    if ty == "f32" {
        return "List<Float>".to_string();
    }
    if ty == "f64" {
        return "List<Double>".to_string();
    }
    if ty == "char" {
        return "List<Character>".to_string();
    }
    if ty == "bool" {
        return "List<Boolean>".to_string();
    }

    let obj = rewrite_rust_to_java(&ts2(ty), errors).unwrap_or("Object".to_string());
    format!("List<{obj}>")
}

// rewrite [Rust] to [Java Type]
pub fn rewrite_rust_to_java(ty: &TokenStream2, errors: &mut CompileErrors) -> Option<String> {
    let rust_type = ty.to_string().replace(' ', "");

    // ignored types
    if rust_type.contains("JNIEnv") {
        return None;
    };
    if rust_type.contains("JClass") {
        return None;
    };

    if rust_type.contains("JList<") {
        return Some(to_java_list(rust_type, errors));
    };

    // void
    if rust_type.contains("()") || rust_type == "" {
        return Some("void".to_string());
    };

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
    if rust_type.contains("JString") {
        return Some("String".to_string());
    };
    if rust_type.contains("JObject") {
        return Some("Object".to_string());
    };
    if rust_type.contains("JByteArray") {
        return Some("byte[]".to_string());
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

    // rust primitves
    if rust_type == "u8" || rust_type == "i8" {
        return Some("byte".to_string());
    }
    if rust_type == "i16" {
        return Some("short".to_string());
    }
    if rust_type == "i32" {
        return Some("int".to_string());
    }
    if rust_type == "i64" {
        return Some("long".to_string());
    }
    if rust_type == "f32" {
        return Some("float".to_string());
    }
    if rust_type == "f64" {
        return Some("double".to_string());
    }
    if rust_type == "bool" {
        return Some("boolean".to_string());
    }
    if rust_type == "char" {
        return Some("char".to_string());
    } 
    if rust_type == "Vec<u8>" {
        return Some("byte[]".to_string());
    }

    // objects

    if rust_type.contains("String") {
        return Some("String".to_string());
    };

    if rust_type.contains("Vec<u8>") {
        return Some("byte[]".to_string());
    };

    Some(rust_type)

    // debug
    // errors.add_spaned(
    //     ty.span(),
    //     format!("unsupported Type: {rust_type}."),
    // );
    // None
}

const OBJECT_TYPES: &[&str] = &[
    "()", "JByte", "JShort", "JInt", "JLong", "JFloat", "JDouble", "JBoolean", "JChar",
];

// Rewrite [Rust Type] to [JNI Rust]
pub fn rewrite_rust_type_to_jni(
    ty: &TokenStream2,
    lifetime: &TokenStream2,
    _errors: &mut CompileErrors,
) -> Option<TokenStream2> {
    let rust_type = ty.to_string().replace(' ', "");

    // Ignored types
    if rust_type.contains("JNIEnv<") {
        return None;
    };
    if rust_type.contains("JClass<") {
        return None;
    };

    // JNI Types

    if rust_type.contains("JString<") {
        return Some(quote! { jni::objects::JString #lifetime });
    };
    if rust_type.contains("JByteArray<") {
        return Some(quote! { jni::objects::JByteArray #lifetime });
    };

    if OBJECT_TYPES.contains(&rust_type.as_str()) {
        return Some(quote! { jni::objects::JObject #lifetime });
    };

    // primitives

    if rust_type == "jbyte" || rust_type == "u8" || rust_type == "i8" {
        return Some(quote! { jni::sys::jbyte });
    };
    if rust_type == "jboolean" || rust_type == "bool" {
        return Some(quote! { jni::sys::jboolean });
    };
    if rust_type == "jshort" || rust_type == "i16" {
        return Some(quote! { jni::sys::jshort });
    };
    if rust_type == "jint" || rust_type == "i32" {
        return Some(quote! { jni::sys::jint });
    };
    if rust_type == "jlong" || rust_type == "i64" {
        return Some(quote! { jni::sys::jlong });
    };
    if rust_type == "jfloat" || rust_type == "f32" {
        return Some(quote! { jni::sys::jfloat });
    };
    if rust_type == "jdouble" || rust_type == "f64" {
        return Some(quote! { jni::sys::jdouble });
    };
    if rust_type == "jchar" || rust_type == "char" {
        return Some(quote! { jni::sys::jchar });
    };

    // Typed Objects

    if rust_type == "String" {
        return Some(quote! { jni::objects::JString #lifetime });
    };
    if rust_type == "Vec<u8>" {
        return Some(quote! { jni::objects::JByteArray #lifetime  });
    };

    Some(quote! { jni::objects::JObject #lifetime })
}
