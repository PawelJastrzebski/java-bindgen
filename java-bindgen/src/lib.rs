pub use jni;
pub mod jni_to_rust;
pub mod exception;
pub use exception::JResult;

/// Macro
pub extern crate java_bindgen_macro as derive;

pub mod prelude {
    pub use crate::derive::{java_bindgen, java_bindgen_raw, java_bindgen_raw2, IntoJava};
    pub use crate::jni_to_rust::*;
    pub use crate::signature_by_type;
    pub use jni;
    pub use jni::objects::{JByteArray, JClass, JObject, JString};
    pub use jni::sys::{jbyte, jchar, jboolean, jint, jshort, jlong, jfloat, jdouble};
    pub use jni::JNIEnv;

    pub type CastJByte<'a> = JObject<'a>;

    pub use crate::exception::*;
}
