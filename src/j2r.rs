use jni::objects::{JByteArray, JObject, JString};
use crate::prelude::*;

use super::*;
use crate::exception::*;

// Java To Rust
pub trait IntoRustType<'local, T> {
    fn into_rust(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<T>;
}

impl<'local, T: IntoRustType<'local, R>, R> IntoRustType<'local, Option<R>> for Option<T> {
    fn into_rust(self, env: &mut JNIEnv<'local>) -> JResult<Option<R>> {
        match self {
            None => { Ok(None) }
            Some(v) => {
                let t = v.into_rust(env)?;
                Ok(Some(t))
            }
        }
    }
}

impl<'local> IntoRustType<'local, bool> for jni::sys::jboolean {
    fn into_rust(self, _: &mut jni::JNIEnv<'local>) -> crate::JResult<bool> {
        Ok(self == jni::sys::JNI_TRUE)
    }
}

impl<'local> IntoRustType<'local, u8> for jni::sys::jbyte {
    fn into_rust(self, _: &mut jni::JNIEnv<'local>) -> crate::JResult<u8> {
        Ok(self as u8)
    }
}

impl<'local> IntoRustType<'local, i8> for jni::sys::jbyte {
    fn into_rust(self, _: &mut jni::JNIEnv<'local>) -> crate::JResult<i8> {
        Ok(self)
    }
}

impl<'local> IntoRustType<'local, char> for jni::sys::jchar {
    fn into_rust(self, _: &mut jni::JNIEnv<'local>) -> crate::JResult<char> {
        char::from_u32(self as u32).ok_or(JException::from_class_and_msg(
            JExceptionClass::ClassCastException,
            "u32 to char",
        ))
    }
}

impl<'local> IntoRustType<'local, u16> for jni::sys::jchar {
    fn into_rust(self, _: &mut jni::JNIEnv<'local>) -> crate::JResult<u16> {
        Ok(self)
    }
}

impl<'local> IntoRustType<'local, i16> for jni::sys::jshort {
    fn into_rust(self, _: &mut jni::JNIEnv<'local>) -> crate::JResult<i16> {
        Ok(self)
    }
}

impl<'local> IntoRustType<'local, i32> for jni::sys::jint {
    fn into_rust(self, _: &mut jni::JNIEnv<'local>) -> crate::JResult<i32> {
        Ok(self)
    }
}

impl<'local> IntoRustType<'local, i64> for jni::sys::jlong {
    fn into_rust(self, _: &mut jni::JNIEnv<'local>) -> crate::JResult<i64> {
        Ok(self)
    }
}

impl<'local> IntoRustType<'local, f32> for jni::sys::jfloat {
    fn into_rust(self, _: &mut jni::JNIEnv<'local>) -> crate::JResult<f32> {
        Ok(self)
    }
}

impl<'local> IntoRustType<'local, f64> for jni::sys::jdouble {
    fn into_rust(self, _: &mut jni::JNIEnv<'local>) -> crate::JResult<f64> {
        Ok(self)
    }
}

impl<'local> IntoRustType<'local, Vec<u8>> for JByteArray<'local> {
    fn into_rust(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<Vec<u8>> {
        env.convert_byte_array(self)
            .j_catch_ini(env, "Cast failed [JByteArray -> Vec<u8>]")
    }
}

impl<'local> IntoRustType<'local, String> for JString<'local> {
    fn into_rust(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<String> {
        env.get_string_owned(&self)
            .j_catch_ini(env, "Cast failed [JString -> String]")
    }
}

impl<'local> IntoRustType<'local, String> for JObject<'local> {
    fn into_rust(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<String> {
        let j_string = JString::from(self);
        env.get_string_owned(&j_string)
            .j_catch_ini(env, "Cast failed [JObject -> String]")
    }
}

// JValueGen to primitive (Option)

#[allow(non_snake_case)]
macro_rules! JValueGen_Option_impl {
    ($rust_type:ty) => {
        impl<'local> IntoRustType<'local, Option<$rust_type>> for jni::objects::JValueGen<JObject<'local>> {
            fn into_rust(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<Option<$rust_type>> {
                let v: $rust_type = self.into_rust(env)?;
                Ok(Some(v))
            }
        }
    };
}
JValueGen_Option_impl!(u8);
JValueGen_Option_impl!(i8);
JValueGen_Option_impl!(i16);
JValueGen_Option_impl!(i32);
JValueGen_Option_impl!(i64);
JValueGen_Option_impl!(f32);
JValueGen_Option_impl!(f64);
JValueGen_Option_impl!(bool);
JValueGen_Option_impl!(char);

impl<'local> IntoRustType<'local, Option<String>> for jni::objects::JValueGen<JObject<'local>> {
    fn into_rust(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<Option<String>> {
        let obj = self.l().j_catch_ini(env, "Cast failed [JObject -> String]")?;
        if obj.is_null() {
            return Ok(None)
        }
        let v: String = obj.into_rust(env)?;
        Ok(Some(v))
    }
}

impl<'local> IntoRustType<'local, Option<Vec<u8>>> for jni::objects::JValueGen<JObject<'local>> {
    fn into_rust(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<Option<Vec<u8>>> {
        let obj = self.l().j_catch_ini(env, "Cast failed [JObject -> byte[]]")?;
        if obj.is_null() {
            return Ok(None)
        }
        let v: Vec<u8> = obj.into_rust(env)?;
        Ok(Some(v))
    }
}

// JValueGen to primitive

impl<'local> IntoRustType<'local, u8> for jni::objects::JValueGen<JObject<'local>> {
    fn into_rust(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<u8> {
        let b = self.b().j_catch_ini(env, "Cast failed [JObject -> u8]")?;
        Ok(b as u8)
    }
}

impl<'local> IntoRustType<'local, i8> for jni::objects::JValueGen<JObject<'local>> {
    fn into_rust(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<i8> {
        self.b().j_catch_ini(env, "Cast failed [JObject -> i8]")
    }
}

impl<'local> IntoRustType<'local, i16> for jni::objects::JValueGen<JObject<'local>> {
    fn into_rust(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<i16> {
        self.s().j_catch_ini(env, "Cast failed [JObject -> i16]")
    }
}

impl<'local> IntoRustType<'local, i32> for jni::objects::JValueGen<JObject<'local>> {
    fn into_rust(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<i32> {
        self.i().j_catch_ini(env, "Cast failed [JObject -> i32]")
    }
}

impl<'local> IntoRustType<'local, i64> for jni::objects::JValueGen<JObject<'local>> {
    fn into_rust(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<i64> {
        self.j().j_catch_ini(env, "Cast failed [JObject -> i64]")
    }
}

impl<'local> IntoRustType<'local, f32> for jni::objects::JValueGen<JObject<'local>> {
    fn into_rust(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<f32> {
        self.f().j_catch_ini(env, "Cast failed [JObject -> f32]")
    }
}

impl<'local> IntoRustType<'local, f64> for jni::objects::JValueGen<JObject<'local>> {
    fn into_rust(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<f64> {
        self.d().j_catch_ini(env, "Cast failed [JObject -> f64]")
    }
}

impl<'local> IntoRustType<'local, bool> for jni::objects::JValueGen<JObject<'local>> {
    fn into_rust(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<bool> {
        self.z().j_catch_ini(env, "Cast failed [JObject -> bool]")
    }
}

impl<'local> IntoRustType<'local, char> for jni::objects::JValueGen<JObject<'local>> {
    fn into_rust(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<char> {
        let char = self.c().j_catch_ini(env, "Cast failed [JObject -> char]")?;
        char.into_rust(env)
    }
}

impl<'local> IntoRustType<'local, String> for jni::objects::JValueGen<JObject<'local>> {
    fn into_rust(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<String> {
        let obj = self.l().j_catch_ini(env, "Cast failed [JObject -> String]")?;
        obj.into_rust(env)
    }
}

impl<'local> IntoRustType<'local, Vec<u8>> for jni::objects::JValueGen<JObject<'local>> {
    fn into_rust(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<Vec<u8>> {
        let obj = self.l().j_catch_ini(env, "Cast failed [JObject -> byte[]]")?;
        obj.into_rust(env)
    }
}

// JValueGen to class primitive
#[allow(non_snake_case)]
macro_rules! JValueGen_obj_to_class_primitive_impl {
    ($rust_type:tt) => {
        impl<'local> IntoRustType<'local, $rust_type>
            for jni::objects::JValueGen<JObject<'local>>
        {
            fn into_rust(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<$rust_type> {
                let obj = self.l()?;
                let byte = obj.into_rust(env)?;
                Ok($rust_type(byte))
            }
        }
    };
}

JValueGen_obj_to_class_primitive_impl!(JByte);
JValueGen_obj_to_class_primitive_impl!(JShort);
JValueGen_obj_to_class_primitive_impl!(JInt);
JValueGen_obj_to_class_primitive_impl!(JLong);
JValueGen_obj_to_class_primitive_impl!(JFloat);
JValueGen_obj_to_class_primitive_impl!(JDouble);
JValueGen_obj_to_class_primitive_impl!(JBoolean);
JValueGen_obj_to_class_primitive_impl!(JChar);

#[allow(non_snake_case)]
macro_rules! JValueGen_Option_obj_to_class_primitive_impl {
    ($rust_type:tt) => {
        impl<'local> IntoRustType<'local, Option<$rust_type>>
            for jni::objects::JValueGen<JObject<'local>>
        {
            fn into_rust(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<Option<$rust_type>> {
                let obj = self.l()?;
                if obj.is_null() {
                    return Ok(None)
                }

                let byte = obj.into_rust(env)?;
                Ok(Some($rust_type(byte)))
            }
        }
    };
}

JValueGen_Option_obj_to_class_primitive_impl!(JByte);
JValueGen_Option_obj_to_class_primitive_impl!(JShort);
JValueGen_Option_obj_to_class_primitive_impl!(JInt);
JValueGen_Option_obj_to_class_primitive_impl!(JLong);
JValueGen_Option_obj_to_class_primitive_impl!(JFloat);
JValueGen_Option_obj_to_class_primitive_impl!(JDouble);
JValueGen_Option_obj_to_class_primitive_impl!(JBoolean);
JValueGen_Option_obj_to_class_primitive_impl!(JChar);

// JObject to class primitive

macro_rules! obj_to_class_primitive_impl {
    ($rust_type:tt) => {
        impl<'local> IntoRustType<'local, $rust_type> for jni::objects::JObject<'local> {
            fn into_rust(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<$rust_type> {
                let value = self.into_rust(env)?;
                Ok($rust_type(value))
            }
        }
    };
}

obj_to_class_primitive_impl!(JByte);
obj_to_class_primitive_impl!(JShort);
obj_to_class_primitive_impl!(JInt);
obj_to_class_primitive_impl!(JLong);
obj_to_class_primitive_impl!(JFloat);
obj_to_class_primitive_impl!(JDouble);
obj_to_class_primitive_impl!(JBoolean);
obj_to_class_primitive_impl!(JChar);

// impl<'local> IntoRustType<'local, Option<JInt>> for jni::objects::JObject<'local> {
//     fn into_rust(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<Option<JInt>> {
//         if self.is_null() {
//             return Ok(None)
//         }
//
//         let value = self.into_rust(env)?;
//         Ok(Some(JInt(value)))
//     }
// }

// JObject to primitive

impl<'local> IntoRustType<'local, u8> for JObject<'local> {
    fn into_rust(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<u8> {
        let value: u8 = self.call_getter("byteValue", env)?;
        Ok(value)
    }
}

impl<'local> IntoRustType<'local, i8> for JObject<'local> {
    fn into_rust(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<i8> {
        let value: u8 = self.call_getter("byteValue", env)?;
        Ok(value as i8)
    }
}

impl<'local> IntoRustType<'local, i16> for JObject<'local> {
    fn into_rust(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<i16> {
        let value: i16 = self.call_getter("shortValue", env)?;
        Ok(value)
    }
}

impl<'local> IntoRustType<'local, i32> for JObject<'local> {
    fn into_rust(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<i32> {
        let value = self.call_getter("intValue", env)?;
        Ok(value)
    }
}

impl<'local> IntoRustType<'local, i64> for JObject<'local> {
    fn into_rust(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<i64> {
        let value = self.call_getter("longValue", env)?;
        Ok(value)
    }
}

impl<'local> IntoRustType<'local, f32> for JObject<'local> {
    fn into_rust(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<f32> {
        let value = self.call_getter("floatValue", env)?;
        Ok(value)
    }
}

impl<'local> IntoRustType<'local, f64> for JObject<'local> {
    fn into_rust(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<f64> {
        let value = self.call_getter("doubleValue", env)?;
        Ok(value)
    }
}

impl<'local> IntoRustType<'local, char> for JObject<'local> {
    fn into_rust(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<char> {
        let value = self.call_getter("charValue", env)?;
        Ok(value)
    }
}

impl<'local> IntoRustType<'local, bool> for JObject<'local> {
    fn into_rust(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<bool> {
        let value = self.call_getter("booleanValue", env)?;
        Ok(value)
    }
}

impl<'local> IntoRustType<'local, Vec<u8>> for JObject<'local> {
    fn into_rust(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<Vec<u8>> {
        JByteArray::from(self).into_rust(env)
    }
}

// Raw JNI types (Allows pass raw type as fn argument)

impl<'local> IntoRustType<'local, JString<'local>> for JString<'local> {
    fn into_rust(self, _: &mut jni::JNIEnv<'local>) -> crate::JResult<JString<'local>> {
        Ok(self)
    }
}

impl<'local> IntoRustType<'local, JByteArray<'local>> for JByteArray<'local> {
    fn into_rust(self, _: &mut jni::JNIEnv<'local>) -> crate::JResult<JByteArray<'local>> {
        Ok(self)
    }
}

impl<'local> IntoRustType<'local, JObject<'local>> for JObject<'local> {
    fn into_rust(self, _: &mut jni::JNIEnv<'local>) -> crate::JResult<JObject<'local>> {
        Ok(self)
    }
}