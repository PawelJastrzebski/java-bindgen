use crate::exception::JavaCatchINI;

// Getters
pub trait JObjectGetters<'local> {
    fn call_getter<T>(&self, name: &str, env: &mut jni::JNIEnv<'local>) -> crate::JResult<T>
    where
        T: JTypeInfo,
        jni::objects::JValueGen<jni::objects::JObject<'local>>:
            j2r::IntoRustType<'local, T>;
}

impl<'l> JObjectGetters<'l> for jni::objects::JObject<'l> {
    fn call_getter<T>(&self, name: &str, env: &mut jni::JNIEnv<'l>) -> crate::JResult<T>
    where
        T: JTypeInfo,
        jni::objects::JValueGen<jni::objects::JObject<'l>>: IntoRustType<'l, T>,
    {
        let ty = T::j_type().to_string();
        let e = env
            .call_method(self, name, format!("(){ty}"), &[])
            .j_catch_ini(env, &format!("Call Java getter: {name}()"))?;

        e.into_rust(env)
    }
}

pub use j2r::*;
mod j2r {
    use super::*;
    use crate::exception::*;

    // Java To Rust
    pub trait IntoRustType<'local, T> {
        fn into_rust(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<T>;
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

    impl<'local> IntoRustType<'local, Vec<u8>> for jni::objects::JByteArray<'local> {
        fn into_rust(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<Vec<u8>> {
            env.convert_byte_array(self)
                .j_catch_ini(env, "Cast failed [JByteArray -> Vec<u8>]")
        }
    }

    impl<'local> IntoRustType<'local, String> for jni::objects::JString<'local> {
        fn into_rust(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<String> {
            env.get_string_owned(&self)
                .j_catch_ini(env, "Cast failed [JString -> String]")
        }
    }

    impl<'local> IntoRustType<'local, String> for jni::objects::JObject<'local> {
        fn into_rust(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<String> {
            let j_string = jni::objects::JString::from(self);
            env.get_string_owned(&j_string)
                .j_catch_ini(env, "Cast failed [JObject -> String]")
        }
    }

    impl<'local> IntoRustType<'local, String>
        for jni::objects::JValueGen<jni::objects::JObject<'local>>
    {
        fn into_rust(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<String> {
            let obj = self.l().j_catch_ini(env, "JObject -> String")?;
            obj.into_rust(env)
        }
    }

    impl<'local> IntoRustType<'local, i8> for jni::objects::JValueGen<jni::objects::JObject<'local>> {
        fn into_rust(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<i8> {
            self.b().j_catch_ini(env, "JObject -> i8")
        }
    }

    impl<'local> IntoRustType<'local, u8> for jni::objects::JValueGen<jni::objects::JObject<'local>> {
        fn into_rust(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<u8> {
            let b = self.b().j_catch_ini(env, "JObject -> u8")?;
            Ok(b as u8)
        }
    }

    impl<'local> IntoRustType<'local, i16> for jni::objects::JValueGen<jni::objects::JObject<'local>> {
        fn into_rust(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<i16> {
            self.s().j_catch_ini(env, "JObject -> i16")
        }
    }

    impl<'local> IntoRustType<'local, i32> for jni::objects::JValueGen<jni::objects::JObject<'local>> {
        fn into_rust(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<i32> {
            self.i().j_catch_ini(env, "JObject -> i32")
        }
    }

    impl<'local> IntoRustType<'local, i64> for jni::objects::JValueGen<jni::objects::JObject<'local>> {
        fn into_rust(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<i64> {
            self.j().j_catch_ini(env, "JObject -> i64")
        }
    }

    impl<'local> IntoRustType<'local, f32> for jni::objects::JValueGen<jni::objects::JObject<'local>> {
        fn into_rust(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<f32> {
            self.f().j_catch_ini(env, "JObject -> f32")
        }
    }

    impl<'local> IntoRustType<'local, f64> for jni::objects::JValueGen<jni::objects::JObject<'local>> {
        fn into_rust(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<f64> {
            self.d().j_catch_ini(env, "JObject -> f64")
        }
    }

    impl<'local> IntoRustType<'local, bool> for jni::objects::JValueGen<jni::objects::JObject<'local>> {
        fn into_rust(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<bool> {
            self.z().j_catch_ini(env, "JObject -> bool")
        }
    }

    impl<'local> IntoRustType<'local, char> for jni::objects::JValueGen<jni::objects::JObject<'local>> {
        fn into_rust(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<char> {
            let char = self.c().j_catch_ini(env, "JObject -> char")?;
            char.into_rust(env)
        }
    }

    impl<'local> IntoRustType<'local, Vec<u8>>
        for jni::objects::JValueGen<jni::objects::JObject<'local>>
    {
        fn into_rust(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<Vec<u8>> {
            let obj = self.l().j_catch_ini(env, "JObject -> Vec<u8>")?;
            jni::objects::JByteArray::from(obj).into_rust(env)
        }
    }

    impl<'local> IntoRustType<'local, JByte>
        for jni::objects::JValueGen<jni::objects::JObject<'local>>
    {
        fn into_rust(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<JByte> {
            let obj = self.l().j_catch_ini(env, "JObject -> JByte")?;
            let byte: u8 = obj.call_getter("byteValue", env)?;
            Ok(JByte(byte as i8))
        }
    }

    impl<'local> IntoRustType<'local, JShort>
        for jni::objects::JValueGen<jni::objects::JObject<'local>>
    {
        fn into_rust(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<JShort> {
            let obj = self.l().j_catch_ini(env, "JObject -> JShort")?;
            let short: i16 = obj.call_getter("shortValue", env)?;
            Ok(JShort(short))
        }
    }

    impl<'local> IntoRustType<'local, JInt> for jni::objects::JValueGen<jni::objects::JObject<'local>> {
        fn into_rust(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<JInt> {
            let obj = self.l().j_catch_ini(env, "JObject -> JInt")?;
            let int: i32 = obj.call_getter("intValue", env)?;
            Ok(JInt(int))
        }
    }

    impl<'local> IntoRustType<'local, JLong>
        for jni::objects::JValueGen<jni::objects::JObject<'local>>
    {
        fn into_rust(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<JLong> {
            let obj = self.l().j_catch_ini(env, "JObject -> JLong")?;
            let long: i64 = obj.call_getter("longValue", env)?;
            Ok(JLong(long))
        }
    }

    impl<'local> IntoRustType<'local, JFloat>
        for jni::objects::JValueGen<jni::objects::JObject<'local>>
    {
        fn into_rust(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<JFloat> {
            let obj = self.l().j_catch_ini(env, "JObject -> JFloat")?;
            let float: f32 = obj.call_getter("floatValue", env)?;
            Ok(JFloat(float))
        }
    }

    impl<'local> IntoRustType<'local, JDouble>
        for jni::objects::JValueGen<jni::objects::JObject<'local>>
    {
        fn into_rust(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<JDouble> {
            let obj = self.l().j_catch_ini(env, "JObject -> JDouble")?;
            let double: f64 = obj.call_getter("doubleValue", env)?;
            Ok(JDouble(double))
        }
    }

    impl<'local> IntoRustType<'local, JChar>
        for jni::objects::JValueGen<jni::objects::JObject<'local>>
    {
        fn into_rust(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<JChar> {
            let obj = self.l().j_catch_ini(env, "JObject -> JChar")?;
            let char: char = obj.call_getter("charValue", env)?;
            Ok(JChar(char))
        }
    }

    impl<'local> IntoRustType<'local, JBoolean>
        for jni::objects::JValueGen<jni::objects::JObject<'local>>
    {
        fn into_rust(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<JBoolean> {
            let obj = self.l().j_catch_ini(env, "JObject -> JBoolean")?;
            let bool: bool = obj.call_getter("booleanValue", env)?;
            Ok(JBoolean(bool))
        }
    }

    // Raw JNI types

    impl<'local> IntoRustType<'local, jni::objects::JString<'local>> for jni::objects::JString<'local> {
        fn into_rust(
            self,
            _: &mut jni::JNIEnv<'local>,
        ) -> crate::JResult<jni::objects::JString<'local>> {
            Ok(self)
        }
    }

    impl<'local> IntoRustType<'local, jni::objects::JByteArray<'local>>
        for jni::objects::JByteArray<'local>
    {
        fn into_rust(
            self,
            _: &mut jni::JNIEnv<'local>,
        ) -> crate::JResult<jni::objects::JByteArray<'local>> {
            Ok(self)
        }
    }

    impl<'local> IntoRustType<'local, jni::objects::JObject<'local>> for jni::objects::JObject<'local> {
        fn into_rust(
            self,
            _: &mut jni::JNIEnv<'local>,
        ) -> crate::JResult<jni::objects::JObject<'local>> {
            Ok(self)
        }
    }
}

pub use r2j::*;
mod r2j {
    use crate::exception::*;

    // Rust to Java
    pub trait IntoJavaType<'local>
    where
        Self::JType: Default,
    {
        type JType;
        fn into_java(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<Self::JType>;
    }

    // void/null ()

    impl<'local> IntoJavaType<'local> for () {
        type JType = jni::objects::JObject<'local>;

        fn into_java(self, _: &mut jni::JNIEnv<'local>) -> crate::JResult<Self::JType> {
            Ok(jni::objects::JObject::null())
        }
    }

    // Raw types (JNI)

    impl<'local> IntoJavaType<'local> for jni::objects::JObject<'local> {
        type JType = jni::objects::JObject<'local>;

        fn into_java(self, _: &mut jni::JNIEnv<'local>) -> crate::JResult<Self::JType> {
            Ok(self)
        }
    }

    impl<'local> IntoJavaType<'local> for jni::objects::JString<'local> {
        type JType = jni::objects::JString<'local>;

        fn into_java(self, _: &mut jni::JNIEnv<'local>) -> crate::JResult<Self::JType> {
            Ok(self)
        }
    }

    impl<'local> IntoJavaType<'local> for jni::objects::JByteArray<'local> {
        type JType = jni::objects::JByteArray<'local>;

        fn into_java(self, _: &mut jni::JNIEnv<'local>) -> crate::JResult<Self::JType> {
            Ok(self)
        }
    }

    // primitves

    impl<'local> IntoJavaType<'local> for i8 {
        type JType = jni::sys::jbyte;

        fn into_java(self, _: &mut jni::JNIEnv<'local>) -> crate::JResult<Self::JType> {
            Ok(self)
        }
    }
    impl<'local> IntoJavaType<'local> for u8 {
        type JType = jni::sys::jbyte;

        fn into_java(self, _: &mut jni::JNIEnv<'local>) -> crate::JResult<Self::JType> {
            Ok(self as i8)
        }
    }

    impl<'local> IntoJavaType<'local> for i32 {
        type JType = jni::sys::jint;

        fn into_java(self, _: &mut jni::JNIEnv<'local>) -> crate::JResult<Self::JType> {
            Ok(self)
        }
    }

    impl<'local> IntoJavaType<'local> for i64 {
        type JType = jni::sys::jlong;

        fn into_java(self, _: &mut jni::JNIEnv<'local>) -> crate::JResult<Self::JType> {
            Ok(self)
        }
    }

    impl<'local> IntoJavaType<'local> for bool {
        type JType = jni::sys::jboolean;

        fn into_java(self, _: &mut jni::JNIEnv<'local>) -> crate::JResult<Self::JType> {
            Ok(self as u8)
        }
    }

    impl<'local> IntoJavaType<'local> for char {
        type JType = jni::sys::jchar;

        fn into_java(self, _: &mut jni::JNIEnv<'local>) -> crate::JResult<Self::JType> {
            Ok(self as u16)
        }
    }

    impl<'local> IntoJavaType<'local> for i16 {
        type JType = jni::sys::jshort;

        fn into_java(self, _: &mut jni::JNIEnv<'local>) -> crate::JResult<Self::JType> {
            Ok(self)
        }
    }

    impl<'local> IntoJavaType<'local> for u16 {
        type JType = jni::sys::jchar;

        fn into_java(self, _: &mut jni::JNIEnv<'local>) -> crate::JResult<Self::JType> {
            Ok(self)
        }
    }

    impl<'local> IntoJavaType<'local> for f32 {
        type JType = jni::sys::jfloat;

        fn into_java(self, _: &mut jni::JNIEnv<'local>) -> crate::JResult<Self::JType> {
            Ok(self)
        }
    }

    impl<'local> IntoJavaType<'local> for f64 {
        type JType = jni::sys::jdouble;

        fn into_java(self, _: &mut jni::JNIEnv<'local>) -> crate::JResult<Self::JType> {
            Ok(self)
        }
    }

    // primitves class wrappers

    impl<'local> IntoJavaType<'local> for super::JByte {
        type JType = jni::objects::JObject<'local>;

        fn into_java(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<Self::JType> {
            let args = &[jni::objects::JValue::Byte(self.0)];
            let class = env.find_class("java/lang/Byte").j_catch_ini(env, "JByte -> JObject")?;
            let obj = env
                .call_static_method(class, "valueOf", "(B)Ljava/lang/Byte;", args)
                .j_catch_ini(env, "JByte -> JObject")?;

            obj.l().j_catch_ini(env, "JByte -> JObject")
        }
    }

    impl<'local> IntoJavaType<'local> for super::JShort {
        type JType = jni::objects::JObject<'local>;

        fn into_java(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<Self::JType> {
            let args = &[jni::objects::JValue::Short(self.0)];
            let class = env.find_class("java/lang/Short").j_catch_ini(env, "JShort -> JObject")?;
            let obj = env
                .call_static_method(class, "valueOf", "(S)Ljava/lang/Short;", args)
                .j_catch_ini(env, "JShort -> JObject")?;

            obj.l().j_catch_ini(env, "JShort -> JObject")
        }
    }

    impl<'local> IntoJavaType<'local> for super::JInt {
        type JType = jni::objects::JObject<'local>;

        fn into_java(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<Self::JType> {
            let args = &[jni::objects::JValue::Int(self.0)];
            let class = env.find_class("java/lang/Integer").j_catch_ini(env, "JInt -> JObject")?;
            let obj = env
                .call_static_method(class, "valueOf", "(I)Ljava/lang/Integer;", args)
                .j_catch_ini(env, "JInt -> JObject")?;

            obj.l().j_catch_ini(env, "JInt -> JObject")
        }
    }

    impl<'local> IntoJavaType<'local> for super::JLong {
        type JType = jni::objects::JObject<'local>;

        fn into_java(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<Self::JType> {
            let args = &[jni::objects::JValue::Long(self.0)];
            let class = env.find_class("java/lang/Long").j_catch_ini(env, "JLong -> JObject")?;
            let obj = env
                .call_static_method(class, "valueOf", "(J)Ljava/lang/Long;", args)
                .j_catch_ini(env, "JLong -> JObject")?;

            obj.l().j_catch_ini(env, "JLong -> JObject")
        }
    }

    impl<'local> IntoJavaType<'local> for super::JFloat {
        type JType = jni::objects::JObject<'local>;

        fn into_java(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<Self::JType> {
            let args = &[jni::objects::JValue::Float(self.0)];
            let class = env.find_class("java/lang/Float").j_catch_ini(env, "JFloat -> JObject")?;
            let obj = env
                .call_static_method(class, "valueOf", "(F)Ljava/lang/Float;", args)
                .j_catch_ini(env, "JFloat -> JObject")?;

            obj.l().j_catch_ini(env, "JFloat -> JObject")
        }
    }

    impl<'local> IntoJavaType<'local> for super::JDouble {
        type JType = jni::objects::JObject<'local>;

        fn into_java(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<Self::JType> {
            let args = &[jni::objects::JValue::Double(self.0)];
            let class = env.find_class("java/lang/Double").j_catch_ini(env, "JDouble -> JObject")?;
            let obj = env
                .call_static_method(class, "valueOf", "(D)Ljava/lang/Double;", args)
                .j_catch_ini(env, "JDouble -> JObject")?;

            obj.l().j_catch_ini(env, "JDouble -> JObject")
        }
    }

    impl<'local> IntoJavaType<'local> for super::JBoolean {
        type JType = jni::objects::JObject<'local>;

        fn into_java(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<Self::JType> {
            let args = &[jni::objects::JValue::Bool(self.0 as u8)];
            let class = env.find_class("java/lang/Boolean").j_catch_ini(env, "JBoolean -> JObject")?;
            let obj = env
                .call_static_method(class, "valueOf", "(Z)Ljava/lang/Boolean;", args)
                .j_catch_ini(env, "JBoolean -> JObject")?;

            obj.l().j_catch_ini(env, "JBoolean -> JObject")
        }
    }

    impl<'local> IntoJavaType<'local> for super::JChar {
        type JType = jni::objects::JObject<'local>;

        fn into_java(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<Self::JType> {
            let args = &[jni::objects::JValue::Char(self.0 as u16)];
            let class = env.find_class("java/lang/Character").j_catch_ini(env, "JChar -> JObject")?;
            let obj = env
                .call_static_method(class, "valueOf", "(C)Ljava/lang/Character;", args)
                .j_catch_ini(env, "JChar -> JObject")?;

            obj.l().j_catch_ini(env, "JChar -> JObject")
        }
    }

    // byte array

    impl<'local> IntoJavaType<'local> for Vec<u8> {
        type JType = jni::objects::JByteArray<'local>;

        fn into_java(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<Self::JType> {
            env.byte_array_from_slice(&self).j_catch_ini(env, "Vec<u8> -> JByteArray")
        }
    }

    impl<'local> IntoJavaType<'local> for &[u8] {
        type JType = jni::objects::JByteArray<'local>;

        fn into_java(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<Self::JType> {
            env.byte_array_from_slice(self).j_catch_ini(env, "&[u8] -> JByteArray")
        }
    }

    // String

    impl<'local> IntoJavaType<'local> for String {
        type JType = jni::objects::JString<'local>;

        fn into_java(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<Self::JType> {
            env.new_string(self).j_catch_ini(env, "String -> JString")
        }
    }

    impl<'local> IntoJavaType<'local> for &str {
        type JType = jni::objects::JString<'local>;

        fn into_java(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<Self::JType> {
            env.new_string(self).j_catch_ini(env, "&str -> JString")
        }
    }
}

pub use jtypes::*;

mod jtypes {
    use jni::{
        objects::{JObject, JValue, JValueOwned},
        signature::{JavaType, ReturnType},
    };

    use crate::{prelude::JavaCatchINI, JResult};

    // Signature Builder
    #[doc(hidden)]
    pub struct TypeSignatureBuilder {
        args: Vec<jni::signature::JavaType>,
    }

    #[allow(dead_code)]
    impl TypeSignatureBuilder {
        pub fn new1<V: JTypeInfo>(_: &V) -> Self {
            Self {
                args: vec![V::j_type()],
            }
        }

        pub fn new_noargs1<R: JTypeInfo>(_: &R) -> jni::signature::TypeSignature {
            jni::signature::TypeSignature {
                args: vec![],
                ret: R::j_return_type(),
            }
        }

        pub fn arg1<V: JTypeInfo>(mut self, _: &V) -> Self {
            self.args.push(V::j_type());
            self
        }

        pub fn ret1<R: JTypeInfo>(self, _: &R) -> jni::signature::TypeSignature {
            jni::signature::TypeSignature {
                args: self.args,
                ret: R::j_return_type(),
            }
        }

        pub fn new<V: JTypeInfo>() -> Self {
            Self {
                args: vec![V::j_type()],
            }
        }

        pub fn new_noargs<R: JTypeInfo>() -> jni::signature::TypeSignature {
            jni::signature::TypeSignature {
                args: vec![],
                ret: R::j_return_type(),
            }
        }

        pub fn arg<V: JTypeInfo>(mut self) -> Self {
            self.args.push(V::j_type());
            self
        }

        pub fn ret<R: JTypeInfo>(self) -> jni::signature::TypeSignature {
            jni::signature::TypeSignature {
                args: self.args,
                ret: R::j_return_type(),
            }
        }
    }

    #[doc(hidden)]
    pub fn arg1<V: JTypeInfo>(v: &V) -> TypeSignatureBuilder {
        TypeSignatureBuilder::new1(v)
    }

    #[doc(hidden)]
    pub fn arg<V: JTypeInfo>() -> TypeSignatureBuilder {
        TypeSignatureBuilder::new::<V>()
    }

    #[macro_export]
    macro_rules! signature_by_type {
        ( => ) => { signature_by_type! { => JVoid } };
        ( => $_return:tt) => { TypeSignatureBuilder::new_noargs::<$_return>() };
        ( $a1:tt => $_return:tt) => { TypeSignatureBuilder::new::<$a1>().ret::<$_return>() };
        ( $a1:tt => ) => { signature_by_type!( $a1 => JVoid) };

        ($a1:tt , $($arg:ty),* => $_return:ty) => {
            TypeSignatureBuilder::new::<$a1>()
            $(.arg::<$arg>())
            *
            .ret::<$_return>()
        };
        ($a1:tt , $($arg:tt),* => ) => { signature_by_type!( $a1 , $($arg)* => JVoid)  };
    }

    #[macro_export]
    macro_rules! signature_by_value {
        ( => ) => { signature_by_value! { => JVoid() } };
        ( => $_return:expr) => { TypeSignatureBuilder::new_noargs1(&$_return) };
        ( $a1:expr => $_return:expr) => { TypeSignatureBuilder::new1(&$a1).ret1(&$_return) };
        ( $a1:expr => ) => { signature_by_value!( $a1 => JVoid()) };

        ($a1:expr , $($arg:expr),* => $_return:expr) => {
            TypeSignatureBuilder::new1(&$a1)
            $(.arg1(&$arg))
            *
            .ret1(&$_return)
        };
        ($a1:expr , $($arg:expr),* => ) => { signature_by_value!( $a1 , $($arg)* => JVoid())  };
    }

    // Java primary types wrappers

    #[repr(transparent)]
    #[derive(Default)]
    pub struct JByte(pub i8);
    #[repr(transparent)]
    #[derive(Default)]
    pub struct JShort(pub i16);
    #[repr(transparent)]
    #[derive(Default)]
    pub struct JInt(pub i32);
    #[repr(transparent)]
    #[derive(Default)]
    pub struct JLong(pub i64);
    #[repr(transparent)]
    #[derive(Default)]
    pub struct JFloat(pub f32);
    #[repr(transparent)]
    #[derive(Default)]
    pub struct JDouble(pub f64);
    #[repr(transparent)]
    #[derive(Default)]
    pub struct JBoolean(pub bool);
    #[repr(transparent)]
    #[derive(Default)]
    pub struct JChar(pub char);

    #[repr(transparent)]
    #[derive(Default)]
    pub struct JVoid();

    pub struct JType<T> {
        sig: jni::signature::JavaType,
        value: T,
    }

    impl<T> JType<T> {
        pub fn into_value(self) -> T {
            self.value
        }

        pub fn signature(&self) -> &jni::signature::JavaType {
            &self.sig
        }
    }

    // Rust to Java Type
    pub trait JTypeInfo
    where
        Self: Sized,
    {
        fn j_return_type() -> jni::signature::ReturnType;
        fn j_type() -> jni::signature::JavaType;
        fn j_value_type(&self) -> jni::signature::JavaType {
            Self::j_type()
        }
        fn into_j_value<'local>(self, _: &mut jni::JNIEnv<'local>) -> JResult<JValueOwned<'local>>;
    }

    impl JTypeInfo for JVoid {
        fn j_type() -> jni::signature::JavaType {
            JavaType::Primitive(jni::signature::Primitive::Void)
        }

        fn j_return_type() -> jni::signature::ReturnType {
            ReturnType::Primitive(jni::signature::Primitive::Void)
        }

        fn into_j_value<'local>(self, _: &mut jni::JNIEnv<'local>) -> JResult<JValueOwned<'local>> {
            Ok(JValueOwned::Void)
        }
    }

    impl JTypeInfo for u8 {
        fn j_type() -> jni::signature::JavaType {
            JavaType::Primitive(jni::signature::Primitive::Byte)
        }

        fn j_return_type() -> jni::signature::ReturnType {
            ReturnType::Primitive(jni::signature::Primitive::Byte)
        }

        fn into_j_value<'local>(self, _: &mut jni::JNIEnv<'local>) -> JResult<JValueOwned<'local>> {
            Ok(JValueOwned::Byte(self as i8))
        }
    }

    impl JTypeInfo for i16 {
        fn j_type() -> jni::signature::JavaType {
            JavaType::Primitive(jni::signature::Primitive::Short)
        }

        fn j_return_type() -> jni::signature::ReturnType {
            ReturnType::Primitive(jni::signature::Primitive::Short)
        }

        fn into_j_value<'local>(self, _: &mut jni::JNIEnv<'local>) -> JResult<JValueOwned<'local>> {
            Ok(JValueOwned::Short(self))
        }
    }

    impl JTypeInfo for i32 {
        fn j_type() -> jni::signature::JavaType {
            JavaType::Primitive(jni::signature::Primitive::Int)
        }

        fn j_return_type() -> jni::signature::ReturnType {
            ReturnType::Primitive(jni::signature::Primitive::Int)
        }

        fn into_j_value<'local>(self, _: &mut jni::JNIEnv<'local>) -> JResult<JValueOwned<'local>> {
            Ok(JValueOwned::Int(self))
        }
    }

    impl JTypeInfo for i64 {
        fn j_type() -> jni::signature::JavaType {
            JavaType::Primitive(jni::signature::Primitive::Long)
        }

        fn j_return_type() -> jni::signature::ReturnType {
            ReturnType::Primitive(jni::signature::Primitive::Long)
        }

        fn into_j_value<'local>(self, _: &mut jni::JNIEnv<'local>) -> JResult<JValueOwned<'local>> {
            Ok(JValueOwned::Long(self))
        }
    }

    impl JTypeInfo for f32 {
        fn j_type() -> jni::signature::JavaType {
            JavaType::Primitive(jni::signature::Primitive::Float)
        }

        fn j_return_type() -> jni::signature::ReturnType {
            ReturnType::Primitive(jni::signature::Primitive::Float)
        }

        fn into_j_value<'local>(self, _: &mut jni::JNIEnv<'local>) -> JResult<JValueOwned<'local>> {
            Ok(JValueOwned::Float(self))
        }
    }

    impl JTypeInfo for f64 {
        fn j_type() -> jni::signature::JavaType {
            JavaType::Primitive(jni::signature::Primitive::Double)
        }

        fn j_return_type() -> jni::signature::ReturnType {
            ReturnType::Primitive(jni::signature::Primitive::Double)
        }

        fn into_j_value<'local>(self, _: &mut jni::JNIEnv<'local>) -> JResult<JValueOwned<'local>> {
            Ok(JValueOwned::Double(self))
        }
    }

    impl JTypeInfo for bool {
        fn j_type() -> jni::signature::JavaType {
            JavaType::Primitive(jni::signature::Primitive::Boolean)
        }

        fn j_return_type() -> jni::signature::ReturnType {
            ReturnType::Primitive(jni::signature::Primitive::Boolean)
        }

        fn into_j_value<'local>(self, _: &mut jni::JNIEnv<'local>) -> JResult<JValueOwned<'local>> {
            Ok(JValueOwned::Bool(self as u8))
        }
    }

    impl JTypeInfo for char {
        fn j_type() -> jni::signature::JavaType {
            JavaType::Primitive(jni::signature::Primitive::Char)
        }

        fn j_return_type() -> jni::signature::ReturnType {
            ReturnType::Primitive(jni::signature::Primitive::Char)
        }

        fn into_j_value<'local>(self, _: &mut jni::JNIEnv<'local>) -> JResult<JValueOwned<'local>> {
            Ok(JValueOwned::Char(self as u16))
        }
    }

    // Implementing JTypeInfo for wrapper classes

    impl JTypeInfo for JByte {
        fn j_type() -> jni::signature::JavaType {
            JavaType::Object("java/lang/Byte".to_string())
        }

        fn j_return_type() -> jni::signature::ReturnType {
            ReturnType::Object
        }

        fn into_j_value<'local>(
            self,
            env: &mut jni::JNIEnv<'local>,
        ) -> JResult<JValueOwned<'local>> {
            let class = env.find_class("java/lang/Byte").j_catch_ini(env, "i8 -> JByte")?;
            let args = &[JValue::Byte(self.0)];
            env.call_static_method(class, "valueOf", "(B)Ljava/lang/Byte;", args)
                .j_catch_ini(env, "i8 -> JByte")
        }
    }

    impl JTypeInfo for JShort {
        fn j_type() -> jni::signature::JavaType {
            JavaType::Object("java/lang/Short".to_string())
        }

        fn j_return_type() -> jni::signature::ReturnType {
            ReturnType::Object
        }

        fn into_j_value<'local>(
            self,
            env: &mut jni::JNIEnv<'local>,
        ) -> JResult<JValueOwned<'local>> {
            let class = env.find_class("java/lang/Short").j_catch_ini(env, "i16 -> JShort")?;
            let args = &[JValue::Short(self.0)];
            env.call_static_method(class, "valueOf", "(S)Ljava/lang/Short;", args)
                .j_catch_ini(env, "i16 -> JShort")
        }
    }

    impl JTypeInfo for JInt {
        fn j_type() -> jni::signature::JavaType {
            JavaType::Object("java/lang/Integer".to_string())
        }

        fn j_return_type() -> jni::signature::ReturnType {
            ReturnType::Object
        }

        fn into_j_value<'local>(
            self,
            env: &mut jni::JNIEnv<'local>,
        ) -> JResult<JValueOwned<'local>> {
            let class = env.find_class("java/lang/Integer").j_catch_ini(env, "i32 -> JInt")?;
            let args = &[JValue::Int(self.0)];
            env.call_static_method(class, "valueOf", "(I)Ljava/lang/Integer;", args)
                .j_catch_ini(env, "i32 -> JInt")
        }
    }

    impl JTypeInfo for JLong {
        fn j_type() -> jni::signature::JavaType {
            JavaType::Object("java/lang/Long".to_string())
        }

        fn j_return_type() -> jni::signature::ReturnType {
            ReturnType::Object
        }

        fn into_j_value<'local>(
            self,
            env: &mut jni::JNIEnv<'local>,
        ) -> JResult<JValueOwned<'local>> {
            let class = env.find_class("java/lang/Long").j_catch_ini(env, "i64 -> JLong")?;
            let args = &[JValue::Long(self.0)];
            env.call_static_method(class, "valueOf", "(J)Ljava/lang/Long;", args)
                .j_catch_ini(env, "i64 -> JLong")
        }
    }

    impl JTypeInfo for JFloat {
        fn j_type() -> jni::signature::JavaType {
            JavaType::Object("java/lang/Float".to_string())
        }

        fn j_return_type() -> jni::signature::ReturnType {
            ReturnType::Object
        }

        fn into_j_value<'local>(
            self,
            env: &mut jni::JNIEnv<'local>,
        ) -> JResult<JValueOwned<'local>> {
            let class = env.find_class("java/lang/Float").j_catch_ini(env, "f32 -> JFloat")?;
            let args = &[JValue::Float(self.0)];
            env.call_static_method(class, "valueOf", "(F)Ljava/lang/Float;", args)
                .j_catch_ini(env, "f32 -> JFloat")
        }
    }

    impl JTypeInfo for JDouble {
        fn j_type() -> jni::signature::JavaType {
            JavaType::Object("java/lang/Double".to_string())
        }

        fn j_return_type() -> jni::signature::ReturnType {
            ReturnType::Object
        }

        fn into_j_value<'local>(
            self,
            env: &mut jni::JNIEnv<'local>,
        ) -> JResult<JValueOwned<'local>> {
            let class = env.find_class("java/lang/Double").j_catch_ini(env, "f64 -> JDouble")?;
            let args = &[JValue::Double(self.0)];
            env.call_static_method(class, "valueOf", "(D)Ljava/lang/Double;", args)
                .j_catch_ini(env, "f64 -> JDouble")
        }
    }

    impl JTypeInfo for JBoolean {
        fn j_type() -> jni::signature::JavaType {
            JavaType::Object("java/lang/Boolean".to_string())
        }

        fn j_return_type() -> jni::signature::ReturnType {
            ReturnType::Object
        }

        fn into_j_value<'local>(
            self,
            env: &mut jni::JNIEnv<'local>,
        ) -> JResult<JValueOwned<'local>> {
            let class = env.find_class("java/lang/Boolean").j_catch_ini(env, "bool -> JBoolean")?;
            let args = &[JValue::Bool(self.0 as u8)];
            env.call_static_method(class, "valueOf", "(Z)Ljava/lang/Boolean;", args)
                .j_catch_ini(env, "bool -> JBoolean")
        }
    }

    impl JTypeInfo for JChar {
        fn j_type() -> jni::signature::JavaType {
            JavaType::Object("java/lang/Character".to_string())
        }

        fn j_return_type() -> jni::signature::ReturnType {
            ReturnType::Object
        }

        fn into_j_value<'local>(
            self,
            env: &mut jni::JNIEnv<'local>,
        ) -> JResult<JValueOwned<'local>> {
            let class = env.find_class("java/lang/Character").j_catch_ini(env, "char -> JChar")?;
            let args = &[JValue::Char(self.0 as u16)];
            env.call_static_method(class, "valueOf", "(C)Ljava/lang/Character;", args)
                .j_catch_ini(env, "char -> JChar")
        }
    }

    // String

    impl JTypeInfo for String {
        fn j_type() -> jni::signature::JavaType {
            JavaType::Object("java/lang/String".to_string())
        }

        fn j_return_type() -> jni::signature::ReturnType {
            ReturnType::Object
        }

        fn into_j_value<'local>(
            self,
            env: &mut jni::JNIEnv<'local>,
        ) -> JResult<JValueOwned<'local>> {
            let obj = env.new_string(&self).j_catch_ini(env, "String -> JString")?;
            Ok(JValueOwned::Object(JObject::from(obj)))
        }
    }

    impl JTypeInfo for Vec<u8> {
        fn j_type() -> jni::signature::JavaType {
            JavaType::Array(Box::new(JavaType::Primitive(
                jni::signature::Primitive::Byte,
            )))
        }

        fn j_return_type() -> jni::signature::ReturnType {
            ReturnType::Object
        }

        fn into_j_value<'local>(
            self,
            env: &mut jni::JNIEnv<'local>,
        ) -> JResult<JValueOwned<'local>> {
            let array = env.byte_array_from_slice(&self).j_catch_ini(env, "Vec<u8> -> JByteArray")?;
            Ok(JValueOwned::Object(JObject::from(array)))
        }
    }

    #[cfg(test)]
    pub mod tests {
        use jni::signature::{Primitive, TypeSignature};

        use super::*;

        #[test]
        pub fn test_primitive_types() {
            assert_eq!(100u8.j_value_type(), JavaType::Primitive(Primitive::Byte));

            assert_eq!(100i16.j_value_type(), JavaType::Primitive(Primitive::Short));

            assert_eq!(100i32.j_value_type(), JavaType::Primitive(Primitive::Int));

            assert_eq!(100i64.j_value_type(), JavaType::Primitive(Primitive::Long));

            assert_eq!(
                100.0f32.j_value_type(),
                JavaType::Primitive(Primitive::Float)
            );

            assert_eq!(
                100.0f64.j_value_type(),
                JavaType::Primitive(Primitive::Double)
            );

            assert_eq!(true.j_value_type(), JavaType::Primitive(Primitive::Boolean));

            assert_eq!('a'.j_value_type(), JavaType::Primitive(Primitive::Char));
        }

        #[test]
        pub fn test_wrapper_types() {
            assert_eq!(
                JByte(100).j_value_type(),
                JavaType::Object("java/lang/Byte".to_string())
            );

            assert_eq!(
                JShort(100).j_value_type(),
                JavaType::Object("java/lang/Short".to_string())
            );

            assert_eq!(
                JInt(100).j_value_type(),
                JavaType::Object("java/lang/Integer".to_string())
            );

            assert_eq!(
                JLong(100).j_value_type(),
                JavaType::Object("java/lang/Long".to_string())
            );

            assert_eq!(
                JFloat(100.0).j_value_type(),
                JavaType::Object("java/lang/Float".to_string())
            );

            assert_eq!(
                JDouble(100.0).j_value_type(),
                JavaType::Object("java/lang/Double".to_string())
            );

            assert_eq!(
                JBoolean(true).j_value_type(),
                JavaType::Object("java/lang/Boolean".to_string())
            );

            assert_eq!(
                JChar('a').j_value_type(),
                JavaType::Object("java/lang/Character".to_string())
            );
        }

        #[test]
        pub fn test_class_types() {
            assert_eq!(
                "string".to_string().j_value_type(),
                JavaType::Object("java/lang/String".to_string())
            );
        }

        #[test]
        pub fn test_signature_builder() {
            // builder
            assert_eq!(
                TypeSignature::from_str("(ILjava/lang/String;)F").unwrap(),
                arg::<i32>().arg::<String>().ret::<f32>()
            );

            assert_eq!(
                TypeSignature::from_str("(ILjava/lang/String;)F").unwrap(),
                arg1(&10).arg1(&"test".to_string()).ret1(&10_f32)
            );
        }

        #[test]
        pub fn test_signature_macro_by_type() {
            assert_eq!(
                TypeSignature::from_str("(Ljava/lang/String;I)V").unwrap(),
                signature_by_type!( String , i32 => )
            );

            println!("{}", signature_by_type!( => ));
            println!("{}", signature_by_type!( => String));
            println!("{}", signature_by_type!( => i32));
            println!("{}", signature_by_type!( String  => i32));
            println!("{}", signature_by_type!( String  => ));
            println!("{}", signature_by_type!( String , i32 => i32));
            println!("{}", signature_by_type!( String , i32 => i32));
            println!("{}", signature_by_type!( String , i32 => ));
        }

        #[test]
        pub fn test_signature_macro_by_value() {
            let string = "ok".to_string();

            assert_eq!(
                TypeSignature::from_str("(Ljava/lang/String;I)I").unwrap(),
                signature_by_value!( string, 10_i32 => 10_i32)
            );

            println!("{}", signature_by_value!( => ));
            println!("{}", signature_by_value!( => string));
            println!("{}", signature_by_value!( => 10_i32));
            println!("{}", signature_by_value!( string  => 10_i32));
            println!("{}", signature_by_value!( string  => ));
            println!("{}", signature_by_value!( string , 10_i32 => 10_i32));
            println!("{}", signature_by_value!( string , 10_i32 => 10_i32));
            println!("{}", signature_by_value!( string , 10_i32 => ));
        }
    }
}
