#![doc = include_str!("../README.md")]

pub use jni;
pub mod exception;
pub mod interop;
pub mod logger;
pub use exception::JResult;
pub mod test_utils;

/// Macro
pub extern crate java_bindgen_macro as derive;

pub mod prelude {
    pub use crate::derive::{java_bindgen, java_bindgen_raw, test_jvm, IntoJava, JLogger};
    pub use crate::interop::*;
    pub use crate::signature_by_type;
    pub use jni;
    pub use jni::objects::{JByteArray, JClass, JObject, JString};
    pub use jni::sys::{jboolean, jbyte, jchar, jdouble, jfloat, jint, jlong, jshort};
    pub use jni::JNIEnv;

    pub use crate::exception::*;
}

