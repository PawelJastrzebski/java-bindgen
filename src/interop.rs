use crate::{exception::JavaCatchINI, j2r::IntoRustType, r2j::IntoJavaType};
use std::borrow::BorrowMut;

// Java primary types wrappers
#[repr(transparent)]
#[derive(Default, Debug)]
pub struct JByte(pub i8);
#[repr(transparent)]
#[derive(Default, Debug)]
pub struct JShort(pub i16);
#[repr(transparent)]
#[derive(Default, Debug)]
pub struct JInt(pub i32);
#[repr(transparent)]
#[derive(Default, Debug)]
pub struct JLong(pub i64);
#[repr(transparent)]
#[derive(Default, Debug)]
pub struct JFloat(pub f32);
#[repr(transparent)]
#[derive(Default, Debug)]
pub struct JDouble(pub f64);
#[repr(transparent)]
#[derive(Default, Debug)]
pub struct JBoolean(pub bool);
#[repr(transparent)]
#[derive(Default, Debug)]
pub struct JChar(pub char);

#[repr(transparent)]
#[derive(Default, Debug)]
pub struct JVoid();

// Getters
pub trait JObjectGetters<'l> {
    fn call_getter<T>(&self, name: &str, env: &mut jni::JNIEnv<'l>) -> crate::JResult<T>
    where
        T: JTypeInfo<'l>,
        JValueGen<jni::objects::JObject<'l>>: crate::j2r::IntoRustType<'l, T>;
}

impl<'local> JObjectGetters<'local> for jni::objects::JObject<'local> {
    fn call_getter<T>(&self, name: &str, env: &mut jni::JNIEnv<'local>) -> crate::JResult<T>
    where
        T: JTypeInfo<'local>,
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
    impl<'l> TypeSignatureBuilder {
        pub fn new1<V: JTypeInfo<'l>>(_: &V) -> Self {
            Self {
                args: vec![V::j_type()],
            }
        }

        pub fn new_noargs1<R: JTypeInfo<'l>>(_: &R) -> jni::signature::TypeSignature {
            jni::signature::TypeSignature {
                args: vec![],
                ret: R::j_return_type(),
            }
        }

        pub fn arg1<V: JTypeInfo<'l>>(mut self, _: &V) -> Self {
            self.args.push(V::j_type());
            self
        }

        pub fn ret1<R: JTypeInfo<'l>>(self, _: &R) -> jni::signature::TypeSignature {
            jni::signature::TypeSignature {
                args: self.args,
                ret: R::j_return_type(),
            }
        }

        pub fn new<V: JTypeInfo<'l>>() -> Self {
            Self {
                args: vec![V::j_type()],
            }
        }

        pub fn new_noargs<R: JTypeInfo<'l>>() -> jni::signature::TypeSignature {
            jni::signature::TypeSignature {
                args: vec![],
                ret: R::j_return_type(),
            }
        }

        pub fn arg<V: JTypeInfo<'l>>(mut self) -> Self {
            self.args.push(V::j_type());
            self
        }

        pub fn ret<R: JTypeInfo<'l>>(self) -> jni::signature::TypeSignature {
            jni::signature::TypeSignature {
                args: self.args,
                ret: R::j_return_type(),
            }
        }
    }

    #[doc(hidden)]
    pub fn arg1<V: for<'l> JTypeInfo<'l>>(v: &V) -> TypeSignatureBuilder {
        TypeSignatureBuilder::new1(v)
    }

    pub fn arg<V: for<'l> JTypeInfo<'l>>() -> TypeSignatureBuilder {
        TypeSignatureBuilder::new::<V>()
    }

    #[macro_export]
    macro_rules! signature_by_type {
       ($a1:ty , $($arg:ty),* => $_return:ty) => {
            TypeSignatureBuilder::new::<$a1>()
            $(.arg::<$arg>())
            *
            .ret::<$_return>()
        };
        ( => $_return:ty) => { TypeSignatureBuilder::new_noargs::<$_return>() };
        ( $a1:ty => $_return:ty) => { TypeSignatureBuilder::new::<$a1>().ret::<$_return>() };

        ( => ) => { signature_by_type! {=> JVoid} };
        ( $a1:ty => ) => { signature_by_type!($a1 => JVoid) };
        ($a1:ty , $($arg:ty),* => ) => { signature_by_type!( $a1 , $($arg)* => JVoid) };
    }

    #[macro_export]
    macro_rules! signature_by_value {
         ($a1:expr , $($arg:expr),* => $_return:expr) => {
            TypeSignatureBuilder::new1(&$a1)
            $(.arg1(&$arg))
            *
            .ret1(&$_return)
        };
        ( => $_return:expr) => { TypeSignatureBuilder::new_noargs1(&$_return) };
        ( $a1:expr => $_return:expr) => { TypeSignatureBuilder::new1(&$a1).ret1(&$_return) };

        ( => ) => { signature_by_value! { => JVoid() } };
        ( $a1:expr => ) => { signature_by_value!( $a1 => JVoid()) };
        ($a1:expr , $($arg:expr),* => ) => { signature_by_value!( $a1 , $($arg)* => JVoid())  };
    }

    // Rust to JavaType Info
    pub trait JTypeInfo<'local>
    where
        Self: Sized,
    {
        fn j_return_type() -> jni::signature::ReturnType;
        fn j_type() -> jni::signature::JavaType;
        fn j_value_type(&self) -> jni::signature::JavaType {
            Self::j_type()
        }
        fn into_j_value(self, _: &mut jni::JNIEnv<'local>) -> JResult<JValueOwned<'local>>;
    }

    impl<'local, T: JTypeInfo<'local> + Default> JTypeInfo<'local> for Option<T> {
        fn j_return_type() -> ReturnType {
            T::j_return_type()
        }

        fn j_type() -> JavaType {
            T::j_type()
        }

        fn into_j_value(self, env: &mut JNIEnv<'local>) -> JResult<JValueOwned<'local>> {
            match self {
                None => {
                    match T::j_type() {
                        JavaType::Primitive(_) => T::default().into_j_value(env),
                        _ => Ok(JValueOwned::Object(JObject::null()))
                    }
                }
                Some(v) => v.into_j_value(env)
            }
        }
    }

    impl<'local> JTypeInfo<'local> for JVoid {
        fn j_return_type() -> jni::signature::ReturnType {
            ReturnType::Primitive(jni::signature::Primitive::Void)
        }

        fn j_type() -> jni::signature::JavaType {
            JavaType::Primitive(jni::signature::Primitive::Void)
        }

        fn into_j_value(self, _: &mut jni::JNIEnv<'local>) -> JResult<JValueOwned<'local>> {
            Ok(JValueOwned::Void)
        }
    }

    impl<'local> JTypeInfo<'local> for u8 {
        fn j_return_type() -> jni::signature::ReturnType {
            ReturnType::Primitive(jni::signature::Primitive::Byte)
        }

        fn j_type() -> jni::signature::JavaType {
            JavaType::Primitive(jni::signature::Primitive::Byte)
        }

        fn into_j_value(self, _: &mut jni::JNIEnv<'local>) -> JResult<JValueOwned<'local>> {
            Ok(JValueOwned::Byte(self as i8))
        }
    }

    impl<'local> JTypeInfo<'local> for i16 {
        fn j_return_type() -> jni::signature::ReturnType {
            ReturnType::Primitive(jni::signature::Primitive::Short)
        }

        fn j_type() -> jni::signature::JavaType {
            JavaType::Primitive(jni::signature::Primitive::Short)
        }

        fn into_j_value(self, _: &mut jni::JNIEnv<'local>) -> JResult<JValueOwned<'local>> {
            Ok(JValueOwned::Short(self))
        }
    }

    impl<'local> JTypeInfo<'local> for i32 {
        fn j_return_type() -> jni::signature::ReturnType {
            ReturnType::Primitive(jni::signature::Primitive::Int)
        }

        fn j_type() -> jni::signature::JavaType {
            JavaType::Primitive(jni::signature::Primitive::Int)
        }

        fn into_j_value(self, _: &mut jni::JNIEnv<'local>) -> JResult<JValueOwned<'local>> {
            Ok(JValueOwned::Int(self))
        }
    }

    impl<'local> JTypeInfo<'local> for i64 {
        fn j_return_type() -> jni::signature::ReturnType {
            ReturnType::Primitive(jni::signature::Primitive::Long)
        }

        fn j_type() -> jni::signature::JavaType {
            JavaType::Primitive(jni::signature::Primitive::Long)
        }

        fn into_j_value(self, _: &mut jni::JNIEnv<'local>) -> JResult<JValueOwned<'local>> {
            Ok(JValueOwned::Long(self))
        }
    }

    impl<'local> JTypeInfo<'local> for f32 {
        fn j_return_type() -> jni::signature::ReturnType {
            ReturnType::Primitive(jni::signature::Primitive::Float)
        }

        fn j_type() -> jni::signature::JavaType {
            JavaType::Primitive(jni::signature::Primitive::Float)
        }

        fn into_j_value(self, _: &mut jni::JNIEnv<'local>) -> JResult<JValueOwned<'local>> {
            Ok(JValueOwned::Float(self))
        }
    }

    impl<'local> JTypeInfo<'local> for f64 {
        fn j_return_type() -> jni::signature::ReturnType {
            ReturnType::Primitive(jni::signature::Primitive::Double)
        }

        fn j_type() -> jni::signature::JavaType {
            JavaType::Primitive(jni::signature::Primitive::Double)
        }

        fn into_j_value(self, _: &mut jni::JNIEnv<'local>) -> JResult<JValueOwned<'local>> {
            Ok(JValueOwned::Double(self))
        }
    }

    impl<'local> JTypeInfo<'local> for bool {
        fn j_return_type() -> jni::signature::ReturnType {
            ReturnType::Primitive(jni::signature::Primitive::Boolean)
        }

        fn j_type() -> jni::signature::JavaType {
            JavaType::Primitive(jni::signature::Primitive::Boolean)
        }

        fn into_j_value(self, _: &mut jni::JNIEnv<'local>) -> JResult<JValueOwned<'local>> {
            Ok(JValueOwned::Bool(self as u8))
        }
    }

    impl<'local> JTypeInfo<'local> for char {
        fn j_return_type() -> jni::signature::ReturnType {
            ReturnType::Primitive(jni::signature::Primitive::Char)
        }

        fn j_type() -> jni::signature::JavaType {
            JavaType::Primitive(jni::signature::Primitive::Char)
        }

        fn into_j_value(self, _: &mut jni::JNIEnv<'local>) -> JResult<JValueOwned<'local>> {
            Ok(JValueOwned::Char(self as u16))
        }
    }

    // Implementing JTypeInfo for wrapper classes

    impl<'local> JTypeInfo<'local> for JByte {
        fn j_return_type() -> jni::signature::ReturnType {
            ReturnType::Object
        }

        fn j_type() -> jni::signature::JavaType {
            JavaType::Object("java/lang/Byte".to_string())
        }

        fn into_j_value(self, env: &mut jni::JNIEnv<'local>) -> JResult<JValueOwned<'local>> {
            let obj = self.into_java(env)?;
            Ok(JValueOwned::Object(obj))
        }
    }

    impl<'local> JTypeInfo<'local> for JShort {
        fn j_return_type() -> jni::signature::ReturnType {
            ReturnType::Object
        }

        fn j_type() -> jni::signature::JavaType {
            JavaType::Object("java/lang/Short".to_string())
        }

        fn into_j_value(self, env: &mut jni::JNIEnv<'local>) -> JResult<JValueOwned<'local>> {
            let obj = self.into_java(env)?;
            Ok(JValueOwned::Object(obj))
        }
    }

    impl<'local> JTypeInfo<'local> for JInt {
        fn j_return_type() -> jni::signature::ReturnType {
            ReturnType::Object
        }

        fn j_type() -> jni::signature::JavaType {
            JavaType::Object("java/lang/Integer".to_string())
        }

        fn into_j_value(self, env: &mut jni::JNIEnv<'local>) -> JResult<JValueOwned<'local>> {
            let obj = self.into_java(env)?;
            Ok(JValueOwned::Object(obj))
        }
    }

    impl<'local> JTypeInfo<'local> for JLong {
        fn j_return_type() -> jni::signature::ReturnType {
            ReturnType::Object
        }

        fn j_type() -> jni::signature::JavaType {
            JavaType::Object("java/lang/Long".to_string())
        }

        fn into_j_value(self, env: &mut jni::JNIEnv<'local>) -> JResult<JValueOwned<'local>> {
            let obj = self.0.into_java(env)?;
            Ok(JValueOwned::Object(obj))
        }
    }

    impl<'local> JTypeInfo<'local> for JFloat {
        fn j_return_type() -> jni::signature::ReturnType {
            ReturnType::Object
        }

        fn j_type() -> jni::signature::JavaType {
            JavaType::Object("java/lang/Float".to_string())
        }

        fn into_j_value(self, env: &mut jni::JNIEnv<'local>) -> JResult<JValueOwned<'local>> {
            let obj = self.into_java(env)?;
            Ok(JValueOwned::Object(obj))
        }
    }

    impl<'local> JTypeInfo<'local> for JDouble {
        fn j_return_type() -> jni::signature::ReturnType {
            ReturnType::Object
        }

        fn j_type() -> jni::signature::JavaType {
            JavaType::Object("java/lang/Double".to_string())
        }

        fn into_j_value(self, env: &mut jni::JNIEnv<'local>) -> JResult<JValueOwned<'local>> {
            let obj = self.into_java(env)?;
            Ok(JValueOwned::Object(obj))
        }
    }

    impl<'local> JTypeInfo<'local> for JBoolean {
        fn j_return_type() -> jni::signature::ReturnType {
            ReturnType::Object
        }

        fn j_type() -> jni::signature::JavaType {
            JavaType::Object("java/lang/Boolean".to_string())
        }

        fn into_j_value(self, env: &mut jni::JNIEnv<'local>) -> JResult<JValueOwned<'local>> {
            let obj = self.into_java(env)?;
            Ok(JValueOwned::Object(obj))
        }
    }

    impl<'local> JTypeInfo<'local> for JChar {
        fn j_return_type() -> jni::signature::ReturnType {
            ReturnType::Object
        }

        fn j_type() -> jni::signature::JavaType {
            JavaType::Object("java/lang/Character".to_string())
        }

        fn into_j_value(self, env: &mut jni::JNIEnv<'local>) -> JResult<JValueOwned<'local>> {
            let obj = self.into_java(env)?;
            Ok(JValueOwned::Object(obj))
        }
    }

    // String

    impl<'local> JTypeInfo<'local> for String {
        fn j_return_type() -> jni::signature::ReturnType {
            ReturnType::Object
        }

        fn j_type() -> jni::signature::JavaType {
            JavaType::Object("java/lang/String".to_string())
        }

        fn into_j_value(self, env: &mut jni::JNIEnv<'local>) -> JResult<JValueOwned<'local>> {
            let obj = env
                .new_string(&self)
                .j_catch_ini(env, "String -> JString")?;
            Ok(JValueOwned::Object(JObject::from(obj)))
        }
    }

    impl<'local> JTypeInfo<'local> for Vec<u8> {
        fn j_return_type() -> jni::signature::ReturnType {
            ReturnType::Object
        }

        fn j_type() -> jni::signature::JavaType {
            JavaType::Array(Box::new(JavaType::Primitive(
                jni::signature::Primitive::Byte,
            )))
        }

        fn into_j_value(self, env: &mut jni::JNIEnv<'local>) -> JResult<JValueOwned<'local>> {
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
            assert_eq!(
                TypeSignature::from_str("(Ljava/lang/String;[B)V").unwrap(),
                signature_by_type!( String , Vec<u8> => JVoid )
            );
            assert_eq!(
                TypeSignature::from_str("(Ljava/lang/String;[B)I").unwrap(),
                signature_by_type!( Option<String> , Option<Vec<u8>> => Option<i32> )
            );

            println!("{}", signature_by_type!( => ));
            println!("{}", signature_by_type!( => String));
            println!("{}", signature_by_type!( => i32));
            println!("{}", signature_by_type!( String  => i32));
            println!("{}", signature_by_type!( String  => ));
            println!("{}", signature_by_type!( String , i32 => i32));
            println!("{}", signature_by_type!( String , i32 => i32));
            println!("{}", signature_by_type!( String , i32 => ));
            println!("{}", signature_by_type!( String , Vec<u8> => JVoid ));
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

#[derive(Default)]
pub struct JList<T>(pub Vec<T>);

impl<T> JList<T> {
    pub fn add(&mut self, element: T) {
        self.0.push(element)
    }

    pub fn add_at(&mut self, index: usize, element: T) {
        self.0.insert(index, element)
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.0.get(index)
    }
}

impl<'local, T> JList<T>
where
    jni::objects::JObject<'local>: IntoRustType<'local, i32> + IntoRustType<'local, T>,
{
    fn from_j_object(
        mut obj: jni::objects::JObject<'local>,
        env: &mut jni::JNIEnv<'local>,
    ) -> crate::JResult<Self> {
        let mut items = vec![];

        let size: i32 = obj.borrow_mut().call_getter("size", env)?;
        for i in 0..size {
            let index = i.into_j_value(env)?;
            let e = env
                .call_method(
                    &mut obj,
                    "get",
                    "(I)Ljava/lang/Object;".to_string(),
                    &[index.borrow()],
                )
                .j_catch_ini(env, &"List.get(int) failed".to_string())?;

            let obj = e.l()?;
            let item: T = obj.into_rust(env)?;
            items.push(item)
        }

        Ok(JList(items))
    }
}

impl<'local, T> JList<T>
where
    T: IntoJavaType<'local, jni::objects::JObject<'local>>,
{
    fn into_j_object(
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
                "(Ljava/lang/Object;)Z".to_string(),
                &[index.borrow()],
            )
                .j_catch_ini(env, &"ArrayList.add(T) failed".to_string())?;
        }

        Ok(array_list)
    }
}

impl<'local, T> IntoRustType<'local, JList<T>> for jni::objects::JObject<'local>
where
    jni::objects::JObject<'local>: IntoRustType<'local, i32> + IntoRustType<'local, T>,
{
    fn into_rust(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<JList<T>> {
        JList::from_j_object(self, env)
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
        self.into_j_object(env)
    }
}

impl<'local, T> IntoRustType<'local, JList<T>>
for jni::objects::JValueGen<jni::objects::JObject<'local>>
where
    jni::objects::JObject<'local>: IntoRustType<'local, i32> + IntoRustType<'local, T>,
{
    fn into_rust(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<JList<T>> {
        let obj = self.l()?;
        JList::from_j_object(obj, env)
    }
}

impl<'local, T> JTypeInfo<'local> for JList<T>
where
    T: IntoJavaType<'local, jni::objects::JObject<'local>>,
{
    fn j_return_type() -> jni::signature::ReturnType {
        jni::signature::ReturnType::Object
    }

    fn j_type() -> jni::signature::JavaType {
        jni::signature::JavaType::Object("java/util/List".to_string())
    }

    fn into_j_value(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<JValueOwned<'local>> {
        let obj = self.into_j_object(env)?;
        Ok(JValueOwned::Object(jni::objects::JObject::from(obj)))
    }
}
