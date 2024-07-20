use std::borrow::BorrowMut;
use crate::{exception::JavaCatchINI, j2r::IntoRustType, r2j::IntoJavaType};

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

// Getters
pub trait JObjectGetters<'l> {
    fn call_getter<T>(&self, name: &str, env: &mut jni::JNIEnv<'l>) -> crate::JResult<T>
    where
        T: JTypeInfo,
        JValueGen<jni::objects::JObject<'l>>: crate::j2r::IntoRustType<'l, T>;
}

impl<'local> JObjectGetters<'local> for jni::objects::JObject<'local> {
    fn call_getter<T>(&self, name: &str, env: &mut jni::JNIEnv<'local>) -> crate::JResult<T>
    where
        T: JTypeInfo,
        JValueGen<jni::objects::JObject<'local>>: crate::j2r::IntoRustType<'local, T>,
    {
        let ty = T::j_type().to_string();
        let e = env
            .call_method(self, name, format!("(){ty}"), &[])
            .j_catch_ini(env, &format!("Call Java getter: {name}()"))?;

        e.into_rust(env)
    }
}


use jni::objects::{JValueGen, JValueOwned};

pub use jtypes::*;
mod jtypes {
    use crate::prelude::*;
    // use crate::{prelude::JavaCatchINI, JResult};
    use jni::{
        objects::{JObject, JValueOwned},
        signature::{JavaType, ReturnType},
    };

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

    // Rust to JavaType Info
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
            let obj = self.into_java(env)?;
            Ok(JValueOwned::Object(obj))
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
            let obj = self.into_java(env)?;
            Ok(JValueOwned::Object(obj))
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
            let obj = self.into_java(env)?;
            Ok(JValueOwned::Object(obj))
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
            let obj = self.0.into_java(env)?;
            Ok(JValueOwned::Object(obj))
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
            let obj = self.into_java(env)?;
            Ok(JValueOwned::Object(obj))
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
            let obj = self.into_java(env)?;
            Ok(JValueOwned::Object(obj))
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
            let obj = self.into_java(env)?;
            Ok(JValueOwned::Object(obj))
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
            let obj = self.into_java(env)?;
            Ok(JValueOwned::Object(obj))
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
            let obj = env
                .new_string(&self)
                .j_catch_ini(env, "String -> JString")?;
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
            let array = env
                .byte_array_from_slice(&self)
                .j_catch_ini(env, "Vec<u8> -> JByteArray")?;
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


// Java List<T> Support

#[repr(transparent)]
#[derive(Default)]
pub struct JList<T>(pub Vec<T>);

impl<'local, T> IntoRustType<'local, JList<T>> for jni::objects::JObject<'local>
where
    jni::objects::JObject<'local>: IntoRustType<'local, i32> + IntoRustType<'local, T>,
{
    fn into_rust(mut self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<JList<T>> {
        let mut items = vec![];

        let size: i32 = self.borrow_mut().call_getter("size", env)?;
        for i in 0..size {
            let index = i.into_j_value(env)?;
            let e = env
                .call_method(
                    &mut self,
                    "get",
                    format!("(I)Ljava/lang/Object;"),
                    &[index.borrow()],
                )
                .j_catch_ini(env, &format!("List.get(int) failed"))?;

            let obj = e.l()?;
            let item: T = obj.into_rust(env)?;
            items.push(item)
        }

        Ok(JList(items))
    }
}

impl<'local, T> IntoJavaType<'local, jni::objects::JObject<'local>> for JList<T>
where
    T: IntoJavaType<'local, jni::objects::JObject<'local>>,
{
    fn into_java(
        self,
        env: &mut jni::JNIEnv<'local>,
    ) -> crate::JResult<jni::objects::JObject<'local>> {
        let class = env
            .find_class("java/util/ArrayList")
            .j_catch_ini(env, "ArrayList class not found")?;
        let mut array_list = env
            .new_object(class, "()V", &[])
            .j_catch_ini(env, "Failed to create ArrayList")?;

        for item in self.0.into_iter() {
            let obj = item.into_java(env)?;
            let index = JValueOwned::Object(obj);
            env.call_method(
                &mut array_list,
                "add",
                format!("(Ljava/lang/Object;)Z"),
                &[index.borrow()],
            )
            .j_catch_ini(env, &format!("ArrayList.add(T) failed"))?;
        }

        Ok(array_list)
    }
}
