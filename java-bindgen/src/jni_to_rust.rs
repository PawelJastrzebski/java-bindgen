pub use java_to_rust::*;
mod java_to_rust {
    use crate::exception::*;

    // Java To Rust
    pub trait IntoRustType<'local>
    where
        Self: Default,
    {
        type RType;
        fn into_rust(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<Self::RType>;
    }

    impl<'local> IntoRustType<'local> for jni::objects::JString<'local> {
        type RType = String;

        fn into_rust(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<Self::RType> {
            let string = env.get_string(&self).j_catch(env)?;
            string.to_str().map(|s| s.to_string()).j_catch(env)
        }
    }

    impl<'local> IntoRustType<'local> for jni::objects::JByteArray<'local> {
        type RType = Vec<u8>;

        fn into_rust(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<Self::RType> {
            env.convert_byte_array(&self).j_catch(env)
        }
    }
}

pub use rust_to_java::*;
mod rust_to_java {
    use crate::exception::*;

    // Rust to Java
    pub trait IntoJavaType<'local>
    where
        Self::JType: Default
    {
        type JType;
        fn into_java(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<Self::JType>;
    }

    // jni types

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

    // Byte array

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
mod java_types {
    use jni::{
        objects::{JObject, JValue, JValueOwned},
        signature::{JavaType, ReturnType},
    };

    use crate::{
        prelude::JavaCatch,
        JResult,
    };

    // Signature Builder
    pub struct TypeSignatureBuilder {
        args: Vec<jni::signature::JavaType>,
    }

    #[allow(dead_code)]
    impl TypeSignatureBuilder {
        pub fn new1<V: JTypeInfo>(_: &V) -> Self {
            Self {
                args: vec![V::j_sig()],
            }
        }

        pub fn new_noargs1<R: JTypeInfo>(_: &R) -> jni::signature::TypeSignature {
            jni::signature::TypeSignature {
                args: vec![],
                ret: R::j_return_sig(),
            }
        }

        pub fn arg1<V: JTypeInfo>(mut self, _: &V) -> Self {
            self.args.push(V::j_sig());
            self
        }

        pub fn ret1<R: JTypeInfo>(self, _: &R) -> jni::signature::TypeSignature {
            jni::signature::TypeSignature {
                args: self.args,
                ret: R::j_return_sig(),
            }
        }

        pub fn new<V: JTypeInfo>() -> Self {
            Self {
                args: vec![V::j_sig()],
            }
        }

        pub fn new_noargs<R: JTypeInfo>() -> jni::signature::TypeSignature {
            jni::signature::TypeSignature {
                args: vec![],
                ret: R::j_return_sig(),
            }
        }

        pub fn arg<V: JTypeInfo>(mut self) -> Self {
            self.args.push(V::j_sig());
            self
        }

        pub fn ret<R: JTypeInfo>(self) -> jni::signature::TypeSignature {
            jni::signature::TypeSignature {
                args: self.args,
                ret: R::j_return_sig(),
            }
        }
    }

    pub fn arg1<V: JTypeInfo>(v: &V) -> TypeSignatureBuilder {
        TypeSignatureBuilder::new1(v)
    }

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
    pub struct JByte(pub u8);
    #[repr(transparent)]
    pub struct JShort(pub u16);
    #[repr(transparent)]
    pub struct JInt(pub u32);
    #[repr(transparent)]
    pub struct JLong(pub u64);
    #[repr(transparent)]
    pub struct JFloat(pub f32);
    #[repr(transparent)]
    pub struct JDouble(pub f64);
    #[repr(transparent)]
    pub struct JBoolean(pub bool);
    #[repr(transparent)]
    pub struct JChar(pub char);

    #[repr(transparent)]
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
        fn j_return_sig() -> jni::signature::ReturnType;
        fn j_sig() -> jni::signature::JavaType;
        fn j_signature(&self) -> jni::signature::JavaType {
            Self::j_sig()
        }
        fn j_type(self) -> JType<Self> {
            JType {
                sig: Self::j_sig(),
                value: self,
            }
        }
        fn into_j_value<'local>(self, _: &mut jni::JNIEnv<'local>) -> JResult<JValueOwned<'local>>;
    }

    impl JTypeInfo for JVoid {
        fn j_sig() -> jni::signature::JavaType {
            JavaType::Primitive(jni::signature::Primitive::Void)
        }

        fn j_return_sig() -> jni::signature::ReturnType {
            ReturnType::Primitive(jni::signature::Primitive::Void)
        }

        fn into_j_value<'local>(self, _: &mut jni::JNIEnv<'local>) -> JResult<JValueOwned<'local>> {
            Ok(JValueOwned::Void)
        }
    }

    impl JTypeInfo for u8 {
        fn j_sig() -> jni::signature::JavaType {
            JavaType::Primitive(jni::signature::Primitive::Byte)
        }

        fn j_return_sig() -> jni::signature::ReturnType {
            ReturnType::Primitive(jni::signature::Primitive::Byte)
        }

        fn into_j_value<'local>(self, _: &mut jni::JNIEnv<'local>) -> JResult<JValueOwned<'local>> {
            Ok(JValueOwned::Byte(self as i8))
        }
    }

    impl JTypeInfo for i16 {
        fn j_sig() -> jni::signature::JavaType {
            JavaType::Primitive(jni::signature::Primitive::Short)
        }

        fn j_return_sig() -> jni::signature::ReturnType {
            ReturnType::Primitive(jni::signature::Primitive::Short)
        }
        
        fn into_j_value<'local>(self, _: &mut jni::JNIEnv<'local>) -> JResult<JValueOwned<'local>> {
            Ok(JValueOwned::Short(self))
        }
    }

    impl JTypeInfo for i32 {
        fn j_sig() -> jni::signature::JavaType {
            JavaType::Primitive(jni::signature::Primitive::Int)
        }

        fn j_return_sig() -> jni::signature::ReturnType {
            ReturnType::Primitive(jni::signature::Primitive::Int)
        }

        fn into_j_value<'local>(self, _: &mut jni::JNIEnv<'local>) -> JResult<JValueOwned<'local>> {
            Ok(JValueOwned::Int(self))
        }
    }

    impl JTypeInfo for i64 {
        fn j_sig() -> jni::signature::JavaType {
            JavaType::Primitive(jni::signature::Primitive::Long)
        }

        fn j_return_sig() -> jni::signature::ReturnType {
            ReturnType::Primitive(jni::signature::Primitive::Long)
        }
        
        fn into_j_value<'local>(self, _: &mut jni::JNIEnv<'local>) -> JResult<JValueOwned<'local>> {
            Ok(JValueOwned::Long(self))
        }
    }

    impl JTypeInfo for f32 {
        fn j_sig() -> jni::signature::JavaType {
            JavaType::Primitive(jni::signature::Primitive::Float)
        }

        fn j_return_sig() -> jni::signature::ReturnType {
            ReturnType::Primitive(jni::signature::Primitive::Float)
        }
        
        fn into_j_value<'local>(self, _: &mut jni::JNIEnv<'local>) -> JResult<JValueOwned<'local>> {
            Ok(JValueOwned::Float(self))
        }
    }

    impl JTypeInfo for f64 {
        fn j_sig() -> jni::signature::JavaType {
            JavaType::Primitive(jni::signature::Primitive::Double)
        }

        fn j_return_sig() -> jni::signature::ReturnType {
            ReturnType::Primitive(jni::signature::Primitive::Double)
        }

        fn into_j_value<'local>(self, _: &mut jni::JNIEnv<'local>) -> JResult<JValueOwned<'local>> {
            Ok(JValueOwned::Double(self))
        }
    }

    impl JTypeInfo for bool {
        fn j_sig() -> jni::signature::JavaType {
            JavaType::Primitive(jni::signature::Primitive::Boolean)
        }

        fn j_return_sig() -> jni::signature::ReturnType {
            ReturnType::Primitive(jni::signature::Primitive::Boolean)
        }
        
        fn into_j_value<'local>(self, _: &mut jni::JNIEnv<'local>) -> JResult<JValueOwned<'local>> {
            Ok(JValueOwned::Bool(self as u8))
        }
    }

    impl JTypeInfo for char {
        fn j_sig() -> jni::signature::JavaType {
            JavaType::Primitive(jni::signature::Primitive::Char)
        }

        fn j_return_sig() -> jni::signature::ReturnType {
            ReturnType::Primitive(jni::signature::Primitive::Char)
        }
        
        fn into_j_value<'local>(self, _: &mut jni::JNIEnv<'local>) -> JResult<JValueOwned<'local>> {
            Ok(JValueOwned::Char(self as u16))
        }
    }

    // Implementing JTypeInfo for wrapper classes

    impl JTypeInfo for JByte {
        fn j_sig() -> jni::signature::JavaType {
            JavaType::Object("java/lang/Byte".to_string())
        }

        fn j_return_sig() -> jni::signature::ReturnType {
            ReturnType::Object
        }
        
        fn into_j_value<'local>(self, env: &mut jni::JNIEnv<'local>) -> JResult<JValueOwned<'local>> {
            let class = env.find_class("java/lang/Byte").j_catch(env)?;
            let args = &[JValue::Byte(self.0 as i8)];
            env.call_static_method(class, "valueOf", "(B)Ljava/lang/Byte;", args).j_catch(env)
        }
    }

    impl JTypeInfo for JShort {
        fn j_sig() -> jni::signature::JavaType {
            JavaType::Object("java/lang/Short".to_string())
        }

        fn j_return_sig() -> jni::signature::ReturnType {
            ReturnType::Object
        }
        
        fn into_j_value<'local>(self, env: &mut jni::JNIEnv<'local>) -> JResult<JValueOwned<'local>> {
            let class = env.find_class("java/lang/Short").j_catch(env)?;
            let args = &[JValue::Short(self.0 as i16)];
            env.call_static_method(class, "valueOf", "(S)Ljava/lang/Short;", args).j_catch(env)
        }
    }

    impl JTypeInfo for JInt {
        fn j_sig() -> jni::signature::JavaType {
            JavaType::Object("java/lang/Integer".to_string())
        }

        fn j_return_sig() -> jni::signature::ReturnType {
            ReturnType::Object
        }
        
        fn into_j_value<'local>(self, env: &mut jni::JNIEnv<'local>) -> JResult<JValueOwned<'local>> {
            let class = env.find_class("java/lang/Integer").j_catch(env)?;
            let args = &[JValue::Int(self.0 as i32)];
            env.call_static_method(class, "valueOf", "(I)Ljava/lang/Integer;", args).j_catch(env)
        }
    }

    impl JTypeInfo for JLong {
        fn j_sig() -> jni::signature::JavaType {
            JavaType::Object("java/lang/Long".to_string())
        }

        fn j_return_sig() -> jni::signature::ReturnType {
            ReturnType::Object
        }
        
        fn into_j_value<'local>(self, env: &mut jni::JNIEnv<'local>) -> JResult<JValueOwned<'local>> {
            let class = env.find_class("java/lang/Long").j_catch(env)?;
            let args = &[JValue::Long(self.0 as i64)];
            env.call_static_method(class, "valueOf", "(J)Ljava/lang/Long;", args).j_catch(env)
        }
    }

    impl JTypeInfo for JFloat {
        fn j_sig() -> jni::signature::JavaType {
            JavaType::Object("java/lang/Float".to_string())
        }

        fn j_return_sig() -> jni::signature::ReturnType {
            ReturnType::Object
        }

        fn into_j_value<'local>(self, env: &mut jni::JNIEnv<'local>) -> JResult<JValueOwned<'local>> {
            let class = env.find_class("java/lang/Float").j_catch(env)?;
            let args = &[JValue::Float(self.0)];
            env.call_static_method(class, "valueOf", "(F)Ljava/lang/Float;", args).j_catch(env)
        }
    }

    impl JTypeInfo for JDouble {
        fn j_sig() -> jni::signature::JavaType {
            JavaType::Object("java/lang/Double".to_string())
        }

        fn j_return_sig() -> jni::signature::ReturnType {
            ReturnType::Object
        }

        fn into_j_value<'local>(self, env: &mut jni::JNIEnv<'local>) -> JResult<JValueOwned<'local>> {
            let class = env.find_class("java/lang/Double").j_catch(env)?;
            let args = &[JValue::Double(self.0)];
            env.call_static_method(class, "valueOf", "(D)Ljava/lang/Double;", args).j_catch(env)
        }
    }

    impl JTypeInfo for JBoolean {
        fn j_sig() -> jni::signature::JavaType {
            JavaType::Object("java/lang/Boolean".to_string())
        }

        fn j_return_sig() -> jni::signature::ReturnType {
            ReturnType::Object
        }

        fn into_j_value<'local>(self, env: &mut jni::JNIEnv<'local>) -> JResult<JValueOwned<'local>> {
            let class = env.find_class("java/lang/Boolean").j_catch(env)?;
            let args = &[JValue::Bool(self.0 as u8)];
            env.call_static_method(class, "valueOf", "(Z)Ljava/lang/Boolean;", args).j_catch(env)
        }
    }

    impl JTypeInfo for JChar {
        fn j_sig() -> jni::signature::JavaType {
            JavaType::Object("java/lang/Character".to_string())
        }

        fn j_return_sig() -> jni::signature::ReturnType {
            ReturnType::Object
        }

        fn into_j_value<'local>(self, env: &mut jni::JNIEnv<'local>) -> JResult<JValueOwned<'local>> {
            let class = env.find_class("java/lang/Character").j_catch(env)?;
            let args = &[JValue::Char(self.0 as u16)];
            env.call_static_method(class, "valueOf", "(C)Ljava/lang/Character;", args).j_catch(env)
        }
    }

    // String

    impl JTypeInfo for String {
        fn j_sig() -> jni::signature::JavaType {
            JavaType::Object("java/lang/String".to_string())
        }

        fn j_return_sig() -> jni::signature::ReturnType {
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
        fn j_sig() -> jni::signature::JavaType {
            JavaType::Array(
                Box::new(JavaType::Primitive(jni::signature::Primitive::Byte))
            )
        }

        fn j_return_sig() -> jni::signature::ReturnType {
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
            assert_eq!(
                100u8.j_type().signature(),
                &JavaType::Primitive(Primitive::Byte)
            );
            assert_eq!(100u8.j_type().into_value(), 100);

            assert_eq!(
                100i16.j_type().signature(),
                &JavaType::Primitive(Primitive::Short)
            );
            assert_eq!(100i16.j_type().into_value(), 100);

            assert_eq!(
                100i32.j_type().signature(),
                &JavaType::Primitive(Primitive::Int)
            );
            assert_eq!(100i32.j_type().into_value(), 100);

            assert_eq!(
                100i64.j_type().signature(),
                &JavaType::Primitive(Primitive::Long)
            );
            assert_eq!(100i64.j_type().into_value(), 100);

            assert_eq!(
                100.0f32.j_type().signature(),
                &JavaType::Primitive(Primitive::Float)
            );
            assert_eq!(100.0f32.j_type().into_value(), 100.0);

            assert_eq!(
                100.0f64.j_type().signature(),
                &JavaType::Primitive(Primitive::Double)
            );
            assert_eq!(100.0f64.j_type().into_value(), 100.0);

            assert_eq!(
                true.j_type().signature(),
                &JavaType::Primitive(Primitive::Boolean)
            );
            assert_eq!(true.j_type().into_value(), true);

            assert_eq!(
                'a'.j_type().signature(),
                &JavaType::Primitive(Primitive::Char)
            );
            assert_eq!('a'.j_type().into_value(), 'a');
        }

        #[test]
        pub fn test_wrapper_types() {
            assert_eq!(
                JByte(100).j_type().signature(),
                &JavaType::Object("java/lang/Byte".to_string())
            );
            assert_eq!(JByte(100).j_type().into_value().0, 100);

            assert_eq!(
                JShort(100).j_type().signature(),
                &JavaType::Object("java/lang/Short".to_string())
            );
            assert_eq!(JShort(100).j_type().into_value().0, 100);

            assert_eq!(
                JInt(100).j_type().signature(),
                &JavaType::Object("java/lang/Integer".to_string())
            );
            assert_eq!(JInt(100).j_type().into_value().0, 100);

            assert_eq!(
                JLong(100).j_type().signature(),
                &JavaType::Object("java/lang/Long".to_string())
            );
            assert_eq!(JLong(100).j_type().into_value().0, 100);

            assert_eq!(
                JFloat(100.0).j_type().signature(),
                &JavaType::Object("java/lang/Float".to_string())
            );
            assert_eq!(JFloat(100.0).j_type().into_value().0, 100.0);

            assert_eq!(
                JDouble(100.0).j_type().signature(),
                &JavaType::Object("java/lang/Double".to_string())
            );
            assert_eq!(JDouble(100.0).j_type().into_value().0, 100.0);

            assert_eq!(
                JBoolean(true).j_type().signature(),
                &JavaType::Object("java/lang/Boolean".to_string())
            );
            assert_eq!(JBoolean(true).j_type().into_value().0, true);

            assert_eq!(
                JChar('a').j_type().signature(),
                &JavaType::Object("java/lang/Character".to_string())
            );
            assert_eq!(JChar('a').j_type().into_value().0, 'a');
        }

        #[test]
        pub fn test_class_types() {
            assert_eq!(
                "string".to_string().j_type().signature(),
                &JavaType::Object("java/lang/String".to_string())
            );
            assert_eq!(
                "string".to_string().j_type().into_value(),
                "string".to_string()
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
