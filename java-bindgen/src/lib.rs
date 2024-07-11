pub use jni;
pub mod jni_to_rust;
pub mod exception;
pub use exception::JResult;

/// Macro
pub extern crate java_bindgen_macro as derive;

pub mod prelude {
    pub use crate::derive::{java_bindgen, java_bindgen_raw, IntoJava};
    pub use crate::jni_to_rust::*;
    pub use crate::signature_by_type;
    pub use jni;
    pub use jni::objects::{JByteArray, JClass, JObject, JString, JValue};
    pub use jni::JNIEnv;

    pub use crate::exception::*;
}
