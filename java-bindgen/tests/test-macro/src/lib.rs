// Test case (All macro should compile)

// Return types
pub mod return_types {
    use java_bindgen::prelude::*;

    // primitives

    #[java_bindgen]
    fn returns_void() -> JResult<()> {
        Ok(())
    }

    #[java_bindgen]
    fn returns_jshort() -> JResult<jshort> {
        Ok(16_i16)
    }
    #[java_bindgen]
    fn returns_jshort_i16() -> JResult<i16> {
        Ok(17)
    }

    #[java_bindgen]
    fn returns_jint() -> JResult<jint> {
        Ok(32_i32)
    }
    #[java_bindgen]
    fn returns_jint_i32() -> JResult<i32> {
        Ok(32)
    }

    #[java_bindgen]
    fn returns_jlong() -> JResult<jlong> {
        Ok(64_i64)
    }
    #[java_bindgen]
    fn returns_jlong_i64() -> JResult<i64> {
        Ok(64)
    }

    #[java_bindgen]
    fn returns_jbyte() -> JResult<jbyte> {
        Ok(8)
    }
    #[java_bindgen]
    fn returns_jbyte_u8() -> JResult<u8> {
        Ok(8)
    }

    #[java_bindgen]
    fn returns_jfloat() -> JResult<jfloat> {
        Ok(32.0_f32)
    }
    #[java_bindgen]
    fn returns_jfloat_f32() -> JResult<f32> {
        Ok(32.0)
    }

    #[java_bindgen]
    fn returns_jdouble() -> JResult<jdouble> {
        Ok(64.0_f64)
    }
    #[java_bindgen]
    fn returns_jdouble_i64() -> JResult<f64> {
        Ok(64.0)
    }

    // todo?
    // #[java_bindgen]
    // fn returns_boolean() -> JResult<jboolean> {
    //     Ok(true)
    // }

    #[java_bindgen]
    fn returns_boolean_bool() -> JResult<bool> {
        Ok(true)
    }

    #[java_bindgen]
    fn returns_jchar() -> JResult<jchar> {
        Ok('y' as u16)
    }
    #[java_bindgen]
    fn returns_jchar_char() -> JResult<char> {
        Ok('y')
    }

    // objects

    #[java_bindgen]
    fn returns_string() -> JResult<String> {
        Ok("ok string".to_string())
    }

    #[java_bindgen]
    fn returns_byte_array() -> JResult<Vec<u8>> {
        Ok(vec![1, 2, 3])
    }

    // primitive class wrappers

    #[java_bindgen]
    fn returns_JByte() -> JResult<JByte> {
        Ok(JByte(2))
    }

    #[java_bindgen]
    fn returns_JShort() -> JResult<JShort> {
        Ok(JShort(3))
    }

    #[java_bindgen]
    fn returns_JInt() -> JResult<JInt> {
        Ok(JInt(4))
    }

    #[java_bindgen]
    fn returns_JLong() -> JResult<JLong> {
        Ok(JLong(4))
    }

    #[java_bindgen]
    fn returns_JFloat() -> JResult<JFloat> {
        Ok(JFloat(5.0))
    }

    #[java_bindgen]
    fn returns_JDouble() -> JResult<JDouble> {
        Ok(JDouble(6.0))
    }

    #[java_bindgen]
    fn returns_JBoolean() -> JResult<JBoolean> {
        Ok(JBoolean(true))
    }  
    
    #[java_bindgen]
    fn returns_JChar() -> JResult<JChar> {
        Ok(JChar('y'))
    }
}

// Input types
pub mod input_types {
    use java_bindgen::prelude::*;

    #[java_bindgen]
    fn input_u8(_input: u8) -> JResult<()> {
        Ok(())
    }

    #[java_bindgen]
    fn input_i16(_input: i16) -> JResult<()> {
        Ok(())
    }

    #[java_bindgen]
    fn input_i32(_input: i32) -> JResult<()> {
        Ok(())
    }

    #[java_bindgen]
    fn input_i64(_input: i64) -> JResult<()> {
        Ok(())
    }

    #[java_bindgen]
    fn input_f32(_input: f32) -> JResult<()> {
        Ok(())
    }

    #[java_bindgen]
    fn input_f64(_input: f64) -> JResult<()> {
        Ok(())
    }

    #[java_bindgen]
    fn input_string(_input: String) -> JResult<()> {
        Ok(())
    }

    #[java_bindgen]
    fn input_byte_array<'a>(_input: Vec<u8>) -> JResult<()> {
        Ok(())
    }
}

// Pass types
pub mod pass_types {
    use java_bindgen::prelude::*;

    #[java_bindgen]
    fn pass_u8(input: u8) -> JResult<u8> {
        Ok(input)
    }

    #[java_bindgen]
    fn pass_i16(input: i16) -> JResult<i16> {
        Ok(input)
    }

    #[java_bindgen]
    fn pass_i32(input: i32) -> JResult<i32> {
        Ok(input)
    }

    #[java_bindgen]
    fn pass_i64(input: i64) -> JResult<i64> {
        Ok(input)
    }

    #[java_bindgen]
    fn pass_f32(input: f32) -> JResult<f32> {
        Ok(input)
    }

    #[java_bindgen]
    fn pass_f64(input: f64) -> JResult<f64> {
        Ok(input)
    }

    #[java_bindgen]
    fn pass_string(input: String) -> JResult<String> {
        Ok(input)
    }

    #[java_bindgen]
    fn pass_byte_array<'a>(input: Vec<u8>) -> JResult<Vec<u8>> {
        Ok(input)
    }
}

// Return Custom Class
pub mod custom_type {
    use java_bindgen::prelude::*;

    #[derive(Default, IntoJava, IntoRust)]
    struct UserClass {
        name: String,
        age: i32
    }

    #[java_bindgen]
    fn get_user() -> JResult<UserClass> {
        Ok(UserClass {
            age: 20,
            name: "Tom".to_string(),
        })
    }

    #[java_bindgen]
    fn pass_user(name: String, user: UserClass) -> JResult<UserClass> {
        Ok(UserClass {
            age: user.age + 100,
            name: format!("{name}{}", user.name),
        })
    }

    #[derive(Default, IntoJava, IntoRust)]
    struct AllJavaTypes {
        java_b: u8,
        java_s: i16,
        java_i: i32,
        java_l: i64,
        java_f: f32,
        java_d: f64,
        java_c: char,
        java_bool: bool,
        java_string: String,
        java_barray: Vec<u8>,
    }

    #[java_bindgen]
    fn pass_all_types(object: AllJavaTypes) -> JResult<AllJavaTypes> {
        Ok(object)
    }
}

// Logger
pub mod java_logger {
    use java_bindgen::prelude::*;

    #[derive(JLogger)]
    struct Log();

    #[java_bindgen]
    fn test_logger<'a>(env: &mut JNIEnv<'a>, name: String) -> JResult<()> {
        let logger = Log::init(env);
        let msg = format!("Hello {name}, Welcome to Rust!");
        logger.info(msg, env);
        logger.info("This is [info] level", env);
        logger.warn("This is [warn] level", env);
        logger.error("This is [error] level", env);
        logger.debug("This is [debug] level", env);
        logger.trace("This is [trace] level", env);
        Ok(())
    }

}

#[cfg(test)]
pub mod tests {
    use super::input_types::*;
    use super::return_types::*;
    use java_bindgen::prelude::*;

    #[test_jvm]
    fn should_input_jshort<'a>(_: &mut JNIEnv<'a>, env: JNIEnv<'a>, class: JClass) -> JResult<()> {
        let result = Java_com_test_macro_TestMacro_input_1u8(env, class, 12);
        assert_eq!(result.is_null(), true);
        Ok(())
    }

    #[test_jvm]
    fn should_input_array<'a>(
        test_env: &mut JNIEnv<'a>,
        env: JNIEnv<'a>,
        class: JClass,
    ) -> JResult<()> {
        let array = vec![2_u8, 10].into_java(test_env)?;
        let result = Java_com_test_macro_TestMacro_input_1byte_1array(env, class, array);
        assert_eq!(result.is_null(), true);
        Ok(())
    }

    #[test_jvm]
    fn should_return_jshort<'a>(
        test_env: &mut JNIEnv<'a>,
        env: JNIEnv<'a>,
        class: JClass,
    ) -> JResult<()> {
        let result = Java_com_test_macro_TestMacro_returns_1jshort(env, class);
        assert_eq!(result.into_rust(test_env)?, 16);
        Ok(())
    }
}
