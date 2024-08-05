use crate::prelude::*;
use jni::objects::JObject;

// Rust to Java
pub trait IntoJavaType<'local, T> {
    fn into_java(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<T>;
}

// Option<T>
impl<'local, T, R> IntoJavaType<'local, R> for Option<T>
where
    R: Default,
    T: IntoJavaType<'local, R>
{
    fn into_java(self, env: &mut JNIEnv<'local>) -> JResult<R> {
        match self {
            None => Ok(R::default()),
            Some(v) => v.into_java(env)
        }
    }
}

// Java Void

impl<'local> IntoJavaType<'local, JObject<'local>> for () {
    fn into_java(self, _: &mut jni::JNIEnv<'local>) -> crate::JResult<JObject<'local>> {
        Ok(JObject::null())
    }
}

// Raw types (JNI) (Allows return raw type)

impl<'local> IntoJavaType<'local, JObject<'local>> for JObject<'local> {
    fn into_java(self, _: &mut jni::JNIEnv<'local>) -> crate::JResult<JObject<'local>> {
        Ok(self)
    }
}

impl<'local> IntoJavaType<'local, jni::objects::JString<'local>> for jni::objects::JString<'local> {
    fn into_java(
        self,
        _: &mut jni::JNIEnv<'local>,
    ) -> crate::JResult<jni::objects::JString<'local>> {
        Ok(self)
    }
}

impl<'local> IntoJavaType<'local, jni::objects::JByteArray<'local>>
for jni::objects::JByteArray<'local>
{
    fn into_java(
        self,
        _: &mut jni::JNIEnv<'local>,
    ) -> crate::JResult<jni::objects::JByteArray<'local>> {
        Ok(self)
    }
}

// Class primitives (Auto cast from primitives )

impl<'local> IntoJavaType<'local, JObject<'local>> for u8 {
    fn into_java(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<JObject<'local>> {
        JByte(self as i8).into_java(env)
    }
}

impl<'local> IntoJavaType<'local, JObject<'local>> for i8 {
    fn into_java(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<JObject<'local>> {
        let class = env
            .find_class("java/lang/Byte")
            .j_catch_ini(env, "i8 -> JByte")?;
        let args = &[jni::objects::JValue::Byte(self)];
        let new_obj = env
            .call_static_method(class, "valueOf", "(B)Ljava/lang/Byte;", args)
            .j_catch_ini(env, "i8 -> JByte")?;

        new_obj.l().j_catch_ini(env, "i8 -> JByte")
    }
}

impl<'local> IntoJavaType<'local, JObject<'local>> for i16 {
    fn into_java(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<JObject<'local>> {
        let class = env
            .find_class("java/lang/Short")
            .j_catch_ini(env, "i64 -> JShort")?;
        let args = &[jni::objects::JValue::Short(self)];
        let new_obj = env
            .call_static_method(class, "valueOf", "(S)Ljava/lang/Short;", args)
            .j_catch_ini(env, "i64 -> JShort")?;

        new_obj.l().j_catch_ini(env, "i64 -> JShort")
    }
}

impl<'local> IntoJavaType<'local, JObject<'local>> for i32 {
    fn into_java(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<JObject<'local>> {
        let args = &[jni::objects::JValue::Int(self)];
        let class = env
            .find_class("java/lang/Integer")
            .j_catch_ini(env, "i32 -> JInt")?;
        let obj = env
            .call_static_method(class, "valueOf", "(I)Ljava/lang/Integer;", args)
            .j_catch_ini(env, "i32 -> JInt")?;

        obj.l().j_catch_ini(env, "i32 -> JInt")
    }
}

impl<'local> IntoJavaType<'local, JObject<'local>> for i64 {
    fn into_java(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<JObject<'local>> {
        let class = env
            .find_class("java/lang/Long")
            .j_catch_ini(env, "i64 -> JLong")?;
        let args = &[jni::objects::JValue::Long(self)];
        let new_obj = env
            .call_static_method(class, "valueOf", "(J)Ljava/lang/Long;", args)
            .j_catch_ini(env, "i64 -> JLong")?;

        new_obj.l().j_catch_ini(env, "i64 -> JLong")
    }
}

impl<'local> IntoJavaType<'local, JObject<'local>> for f32 {
    fn into_java(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<JObject<'local>> {
        let args = &[jni::objects::JValue::Float(self)];
        let class = env
            .find_class("java/lang/Float")
            .j_catch_ini(env, "f32 -> JFloat")?;
        let obj = env
            .call_static_method(class, "valueOf", "(F)Ljava/lang/Float;", args)
            .j_catch_ini(env, "f32 -> JFloat")?;

        obj.l().j_catch_ini(env, "f32 -> JFloat")
    }
}

impl<'local> IntoJavaType<'local, JObject<'local>> for f64 {
    fn into_java(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<JObject<'local>> {
        let args = &[jni::objects::JValue::Double(self)];
        let class = env
            .find_class("java/lang/Double")
            .j_catch_ini(env, "f64 -> JDouble")?;
        let obj = env
            .call_static_method(class, "valueOf", "(D)Ljava/lang/Double;", args)
            .j_catch_ini(env, "f64 -> JDouble")?;

        obj.l().j_catch_ini(env, "f64 -> JDouble")
    }
}

impl<'local> IntoJavaType<'local, JObject<'local>> for bool {
    fn into_java(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<JObject<'local>> {
        let args = &[jni::objects::JValue::Bool(self as u8)];
        let class = env
            .find_class("java/lang/Boolean")
            .j_catch_ini(env, "bool -> JBoolean")?;
        let obj = env
            .call_static_method(class, "valueOf", "(Z)Ljava/lang/Boolean;", args)
            .j_catch_ini(env, "bool -> JBoolean")?;

        obj.l().j_catch_ini(env, "bool -> JBoolean")
    }
}

impl<'local> IntoJavaType<'local, JObject<'local>> for char {
    fn into_java(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<JObject<'local>> {
        let args = &[jni::objects::JValue::Char(self as u16)];
        let class = env
            .find_class("java/lang/Character")
            .j_catch_ini(env, "char -> JChar")?;
        let obj = env
            .call_static_method(class, "valueOf", "(C)Ljava/lang/Character;", args)
            .j_catch_ini(env, "char -> JChar")?;

        obj.l().j_catch_ini(env, "char -> JChar")
    }
}

// primitves

impl<'local> IntoJavaType<'local, jni::sys::jbyte> for i8 {
    fn into_java(self, _: &mut jni::JNIEnv<'local>) -> crate::JResult<jni::sys::jbyte> {
        Ok(self)
    }
}

impl<'local> IntoJavaType<'local, jni::sys::jbyte> for u8 {
    fn into_java(self, _: &mut jni::JNIEnv<'local>) -> crate::JResult<jni::sys::jbyte> {
        Ok(self as i8)
    }
}

impl<'local> IntoJavaType<'local, jni::sys::jint> for i32 {
    fn into_java(self, _: &mut jni::JNIEnv<'local>) -> crate::JResult<jni::sys::jint> {
        Ok(self)
    }
}

impl<'local> IntoJavaType<'local, jni::sys::jlong> for i64 {
    fn into_java(self, _: &mut jni::JNIEnv<'local>) -> crate::JResult<jni::sys::jlong> {
        Ok(self)
    }
}

impl<'local> IntoJavaType<'local, jni::sys::jboolean> for bool {
    fn into_java(self, _: &mut jni::JNIEnv<'local>) -> crate::JResult<jni::sys::jboolean> {
        Ok(self as u8)
    }
}

impl<'local> IntoJavaType<'local, jni::sys::jchar> for char {
    fn into_java(self, _: &mut jni::JNIEnv<'local>) -> crate::JResult<jni::sys::jchar> {
        Ok(self as u16)
    }
}

impl<'local> IntoJavaType<'local, jni::sys::jshort> for i16 {
    fn into_java(self, _: &mut jni::JNIEnv<'local>) -> crate::JResult<jni::sys::jshort> {
        Ok(self)
    }
}

impl<'local> IntoJavaType<'local, jni::sys::jchar> for u16 {
    fn into_java(self, _: &mut jni::JNIEnv<'local>) -> crate::JResult<jni::sys::jchar> {
        Ok(self)
    }
}

impl<'local> IntoJavaType<'local, jni::sys::jfloat> for f32 {
    fn into_java(self, _: &mut jni::JNIEnv<'local>) -> crate::JResult<jni::sys::jfloat> {
        Ok(self)
    }
}

impl<'local> IntoJavaType<'local, jni::sys::jdouble> for f64 {
    fn into_java(self, _: &mut jni::JNIEnv<'local>) -> crate::JResult<jni::sys::jdouble> {
        Ok(self)
    }
}

// primitves class wrappers

impl<'local> IntoJavaType<'local, JObject<'local>> for JByte {
    fn into_java(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<JObject<'local>> {
        self.0.into_java(env)
    }
}

impl<'local> IntoJavaType<'local, JObject<'local>> for JShort {
    fn into_java(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<JObject<'local>> {
        self.0.into_java(env)
    }
}

impl<'local> IntoJavaType<'local, JObject<'local>> for JInt {
    fn into_java(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<JObject<'local>> {
        self.0.into_java(env)
    }
}

impl<'local> IntoJavaType<'local, JObject<'local>> for JLong {
    fn into_java(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<JObject<'local>> {
        self.0.into_java(env)
    }
}

impl<'local> IntoJavaType<'local, JObject<'local>> for JFloat {
    fn into_java(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<JObject<'local>> {
        self.0.into_java(env)
    }
}

impl<'local> IntoJavaType<'local, JObject<'local>> for JDouble {
    fn into_java(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<JObject<'local>> {
        self.0.into_java(env)
    }
}

impl<'local> IntoJavaType<'local, JObject<'local>> for JBoolean {
    fn into_java(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<JObject<'local>> {
        self.0.into_java(env)
    }
}

impl<'local> IntoJavaType<'local, JObject<'local>> for JChar {
    fn into_java(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<JObject<'local>> {
        self.0.into_java(env)
    }
}

// byte array

impl<'local> IntoJavaType<'local, jni::objects::JByteArray<'local>> for Vec<u8> {
    fn into_java(
        self,
        env: &mut jni::JNIEnv<'local>,
    ) -> crate::JResult<jni::objects::JByteArray<'local>> {
        env.byte_array_from_slice(&self)
            .j_catch_ini(env, "Vec<u8> -> JByteArray")
    }
}

impl<'local> IntoJavaType<'local, jni::objects::JByteArray<'local>> for &[u8] {
    fn into_java(
        self,
        env: &mut jni::JNIEnv<'local>,
    ) -> crate::JResult<jni::objects::JByteArray<'local>> {
        env.byte_array_from_slice(self)
            .j_catch_ini(env, "&[u8] -> JByteArray")
    }
}

// String

impl<'local> IntoJavaType<'local, jni::objects::JString<'local>> for String {
    fn into_java(
        self,
        env: &mut jni::JNIEnv<'local>,
    ) -> crate::JResult<jni::objects::JString<'local>> {
        env.new_string(self).j_catch_ini(env, "String -> JString")
    }
}

impl<'local> IntoJavaType<'local, jni::objects::JString<'local>> for &str {
    fn into_java(
        self,
        env: &mut jni::JNIEnv<'local>,
    ) -> crate::JResult<jni::objects::JString<'local>> {
        env.new_string(self).j_catch_ini(env, "&str -> JString")
    }
}

// byte array (JObject)

impl<'local> IntoJavaType<'local, jni::objects::JObject<'local>> for Vec<u8> {
    fn into_java(
        self,
        env: &mut jni::JNIEnv<'local>,
    ) -> crate::JResult<jni::objects::JObject<'local>> {
        let array = env
            .byte_array_from_slice(&self)
            .j_catch_ini(env, "Vec<u8> -> JByteArray")?;

        Ok(JObject::from(array))
    }
}

impl<'local> IntoJavaType<'local, jni::objects::JObject<'local>> for &[u8] {
    fn into_java(
        self,
        env: &mut jni::JNIEnv<'local>,
    ) -> crate::JResult<jni::objects::JObject<'local>> {
        let array = env
            .byte_array_from_slice(&self)
            .j_catch_ini(env, "Vec<u8> -> JByteArray")?;

        Ok(JObject::from(array))
    }
}

// String (JObject)

impl<'local> IntoJavaType<'local, jni::objects::JObject<'local>> for String {
    fn into_java(
        self,
        env: &mut jni::JNIEnv<'local>,
    ) -> crate::JResult<jni::objects::JObject<'local>> {
        let string = env.new_string(self).j_catch_ini(env, "String -> JString")?;
        Ok(JObject::from(string))
    }
}

impl<'local> IntoJavaType<'local, jni::objects::JObject<'local>> for &str {
    fn into_java(
        self,
        env: &mut jni::JNIEnv<'local>,
    ) -> crate::JResult<jni::objects::JObject<'local>> {
        let string = env.new_string(self).j_catch_ini(env, "String -> JString")?;
        Ok(JObject::from(string))
    }
}
