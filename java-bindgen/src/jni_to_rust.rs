
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
        Self::JType: Default,
        Self: Clone,
    {
        type JType;
        fn into_java(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<Self::JType>;
    }

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
}
