pub use java_to_rust::*;
mod java_to_rust {
    use jni::objects::JByteArray;

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
            char::from_u32(self as u32).ok_or(JExceptionClass::ArithmeticException.into())
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
            env.convert_byte_array(&self).j_catch(env)
        }
    }

    impl<'local> IntoRustType<'local, String> for jni::objects::JString<'local> {
        fn into_rust(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<String> {
            let string = env.get_string(&self).j_catch(env)?;
            string.to_str().map(|s| s.to_string()).j_catch(env)
        }
    }

    impl<'local> IntoRustType<'local, String> for jni::objects::JObject<'local> {
        fn into_rust(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<String> {
            jni::objects::JString::from(self).into_rust(env)
        }
    }

    impl<'local> IntoRustType<'local, String>
        for jni::objects::JValueGen<jni::objects::JObject<'local>>
    {
        fn into_rust(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<String> {
            let obj = self.l()?;
            obj.into_rust(env)
        }
    }

    impl<'local> IntoRustType<'local, i8> for jni::objects::JValueGen<jni::objects::JObject<'local>> {
        fn into_rust(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<i8> {
            self.b().j_catch(env)
        }
    }

    impl<'local> IntoRustType<'local, u8> for jni::objects::JValueGen<jni::objects::JObject<'local>> {
        fn into_rust(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<u8> {
            let b = self.b().j_catch(env)?;
            Ok(b as u8)
        }
    }

    impl<'local> IntoRustType<'local, i16> for jni::objects::JValueGen<jni::objects::JObject<'local>> {
        fn into_rust(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<i16> {
            self.s().j_catch(env)
        }
    }

    impl<'local> IntoRustType<'local, i32> for jni::objects::JValueGen<jni::objects::JObject<'local>> {
        fn into_rust(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<i32> {
            self.i().j_catch(env)
        }
    }

    impl<'local> IntoRustType<'local, i64> for jni::objects::JValueGen<jni::objects::JObject<'local>> {
        fn into_rust(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<i64> {
            self.j().j_catch(env)
        }
    }

    impl<'local> IntoRustType<'local, f32> for jni::objects::JValueGen<jni::objects::JObject<'local>> {
        fn into_rust(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<f32> {
            self.f().j_catch(env)
        }
    }

    impl<'local> IntoRustType<'local, f64> for jni::objects::JValueGen<jni::objects::JObject<'local>> {
        fn into_rust(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<f64> {
            self.d().j_catch(env)
        }
    }

    impl<'local> IntoRustType<'local, bool> for jni::objects::JValueGen<jni::objects::JObject<'local>> {
        fn into_rust(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<bool> {
            self.z().j_catch(env)
        }
    }

    impl<'local> IntoRustType<'local, char> for jni::objects::JValueGen<jni::objects::JObject<'local>> {
        fn into_rust(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<char> {
            let char = self.c().j_catch(env)?;
            char.into_rust(env)
        }
    }

    impl<'local> IntoRustType<'local, Vec<u8>>
        for jni::objects::JValueGen<jni::objects::JObject<'local>>
    {
        fn into_rust(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<Vec<u8>> {
            let obj = self.l().j_catch(env)?;
            JByteArray::from(obj).into_rust(env)
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
}

pub use java_getters::*;
mod java_getters {
    use super::*;

    // Getters
    pub trait JObjectGetters<'local> {
        fn call_getter<T>(&self, name: &str, env: &mut jni::JNIEnv<'local>) -> crate::JResult<T>
        where
            T: JTypeInfo,
            jni::objects::JValueGen<jni::objects::JObject<'local>>:
                java_to_rust::IntoRustType<'local, T>;
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
                .j_catch(env)?;

            e.into_rust(env)
        }
    }
}

pub use rust_to_java::*;
mod rust_to_java {
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

    // jni types (self impl)

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
            let args = &[jni::objects::JValue::Byte(self.0 as i8)];
            let class = env.find_class("java/lang/Byte").j_catch(env)?;
            let obj = env
                .call_static_method(class, "valueOf", "(B)Ljava/lang/Byte;", args)
                .j_catch(env)?;

            obj.l().j_catch(env)
        }
    }

    impl<'local> IntoJavaType<'local> for super::JShort {
        type JType = jni::objects::JObject<'local>;

        fn into_java(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<Self::JType> {
            let args = &[jni::objects::JValue::Short(self.0)];
            let class = env.find_class("java/lang/Short").j_catch(env)?;
            let obj = env
                .call_static_method(class, "valueOf", "(S)Ljava/lang/Short;", args)
                .j_catch(env)?;

            obj.l().j_catch(env)
        }
    }

    impl<'local> IntoJavaType<'local> for super::JInt {
        type JType = jni::objects::JObject<'local>;

        fn into_java(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<Self::JType> {
            let args = &[jni::objects::JValue::Int(self.0)];
            let class = env.find_class("java/lang/Integer").j_catch(env)?;
            let obj = env
                .call_static_method(class, "valueOf", "(I)Ljava/lang/Integer;", args)
                .j_catch(env)?;

            obj.l().j_catch(env)
        }
    }

    impl<'local> IntoJavaType<'local> for super::JLong {
        type JType = jni::objects::JObject<'local>;

        fn into_java(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<Self::JType> {
            let args = &[jni::objects::JValue::Long(self.0)];
            let class = env.find_class("java/lang/Long").j_catch(env)?;
            let obj = env
                .call_static_method(class, "valueOf", "(J)Ljava/lang/Long;", args)
                .j_catch(env)?;

            obj.l().j_catch(env)
        }
    }

    impl<'local> IntoJavaType<'local> for super::JFloat {
        type JType = jni::objects::JObject<'local>;

        fn into_java(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<Self::JType> {
            let args = &[jni::objects::JValue::Float(self.0)];
            let class = env.find_class("java/lang/Float").j_catch(env)?;
            let obj = env
                .call_static_method(class, "valueOf", "(F)Ljava/lang/Float;", args)
                .j_catch(env)?;

            obj.l().j_catch(env)
        }
    }

    impl<'local> IntoJavaType<'local> for super::JDouble {
        type JType = jni::objects::JObject<'local>;

        fn into_java(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<Self::JType> {
            let args = &[jni::objects::JValue::Double(self.0)];
            let class = env.find_class("java/lang/Double").j_catch(env)?;
            let obj = env
                .call_static_method(class, "valueOf", "(D)Ljava/lang/Double;", args)
                .j_catch(env)?;

            obj.l().j_catch(env)
        }
    }

    impl<'local> IntoJavaType<'local> for super::JBoolean {
        type JType = jni::objects::JObject<'local>;

        fn into_java(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<Self::JType> {
            let args = &[jni::objects::JValue::Bool(self.0 as u8)];
            let class = env.find_class("java/lang/Boolean").j_catch(env)?;
            let obj = env
                .call_static_method(class, "valueOf", "(Z)Ljava/lang/Boolean;", args)
                .j_catch(env)?;

            obj.l().j_catch(env)
        }
    }

    impl<'local> IntoJavaType<'local> for super::JChar {
        type JType = jni::objects::JObject<'local>;

        fn into_java(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<Self::JType> {
            let args = &[jni::objects::JValue::Char(self.0 as u16)];
            let class = env.find_class("java/lang/Character").j_catch(env)?;
            let obj = env
                .call_static_method(class, "valueOf", "(C)Ljava/lang/Character;", args)
                .j_catch(env)?;

            obj.l().j_catch(env)
        }
    }

    // byte array

    impl<'local> IntoJavaType<'local> for Vec<u8> {
        type JType = jni::objects::JByteArray<'local>;

        fn into_java(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<Self::JType> {
            env.byte_array_from_slice(&self).j_catch(env)
        }
    }

    impl<'local> IntoJavaType<'local> for &[u8] {
        type JType = jni::objects::JByteArray<'local>;

        fn into_java(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<Self::JType> {
            env.byte_array_from_slice(&self).j_catch(env)
        }
    }

    // String

    impl<'local> IntoJavaType<'local> for String {
        type JType = jni::objects::JString<'local>;

        fn into_java(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<Self::JType> {
            env.new_string(self).j_catch(env)
        }
    }

    impl<'local> IntoJavaType<'local> for &str {
        type JType = jni::objects::JString<'local>;

        fn into_java(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<Self::JType> {
            env.new_string(self).j_catch(env)
        }
    }
}

pub use java_types::*;

use crate::prelude::JavaCatch;
mod java_types {
    use jni::{
        objects::{JObject, JValue, JValueOwned},
        signature::{JavaType, ReturnType},
    };

    use crate::{prelude::JavaCatch, JResult};

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
            let class = env.find_class("java/lang/Byte").j_catch(env)?;
            let args = &[JValue::Byte(self.0 as i8)];
            env.call_static_method(class, "valueOf", "(B)Ljava/lang/Byte;", args)
                .j_catch(env)
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
            let class = env.find_class("java/lang/Short").j_catch(env)?;
            let args = &[JValue::Short(self.0 as i16)];
            env.call_static_method(class, "valueOf", "(S)Ljava/lang/Short;", args)
                .j_catch(env)
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
            let class = env.find_class("java/lang/Integer").j_catch(env)?;
            let args = &[JValue::Int(self.0 as i32)];
            env.call_static_method(class, "valueOf", "(I)Ljava/lang/Integer;", args)
                .j_catch(env)
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
            let class = env.find_class("java/lang/Long").j_catch(env)?;
            let args = &[JValue::Long(self.0 as i64)];
            env.call_static_method(class, "valueOf", "(J)Ljava/lang/Long;", args)
                .j_catch(env)
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
            let class = env.find_class("java/lang/Float").j_catch(env)?;
            let args = &[JValue::Float(self.0)];
            env.call_static_method(class, "valueOf", "(F)Ljava/lang/Float;", args)
                .j_catch(env)
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
            let class = env.find_class("java/lang/Double").j_catch(env)?;
            let args = &[JValue::Double(self.0)];
            env.call_static_method(class, "valueOf", "(D)Ljava/lang/Double;", args)
                .j_catch(env)
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
            let class = env.find_class("java/lang/Boolean").j_catch(env)?;
            let args = &[JValue::Bool(self.0 as u8)];
            env.call_static_method(class, "valueOf", "(Z)Ljava/lang/Boolean;", args)
                .j_catch(env)
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
            let class = env.find_class("java/lang/Character").j_catch(env)?;
            let args = &[JValue::Char(self.0 as u16)];
            env.call_static_method(class, "valueOf", "(C)Ljava/lang/Character;", args)
                .j_catch(env)
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
            let obj = env.new_string(&self).j_catch(env)?;
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
            let array = env.byte_array_from_slice(&self).j_catch(env)?;
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
