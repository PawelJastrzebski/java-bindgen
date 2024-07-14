// Test case (All macro should compile)

// Return types
pub mod return_types {
    use java_bindgen::prelude::*;

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
    fn returns_byte_array() -> JResult<Vec<u8>> {
        Ok(vec![1, 2, 3])
    }
}

// Input types
pub mod input_types {
    use java_bindgen::prelude::*;

    #[java_bindgen]
    fn input_u8(input: u8) -> JResult<()> {
        println!("byte: {input}");
        Ok(())
    }

    #[java_bindgen]
    fn input_i16(input: i16) -> JResult<()> {
        println!("short: {input}");
        Ok(())
    }

    #[java_bindgen]
    fn input_i32(input: i32) -> JResult<()> {
        println!("integer: {input}");
        Ok(())
    }

    #[java_bindgen]
    fn input_i64(input: i64) -> JResult<()> {
        println!("long: {input}");
        Ok(())
    }

    #[java_bindgen]
    fn input_f32(input: f32) -> JResult<()> {
        println!("float: {input}");
        Ok(())
    }

    #[java_bindgen]
    fn input_f64(input: f64) -> JResult<()> {
        println!("double: {input}");
        Ok(())
    }

    #[java_bindgen]
    fn input_string(input: String) -> JResult<()> {
        println!("string: {input}");
        Ok(())
    }

    #[java_bindgen]
    fn input_byte_array(input: Vec<u8>) -> JResult<()> {
        println!("array: {input:?}");
        Ok(())
    }
}

// Return types
pub mod return_custom_object {
    use java_bindgen::prelude::*;

    #[derive(IntoJava, Default)]
    struct UserClass {
        name: String,
    }

    #[java_bindgen(return = UserClass)]
    fn get_user() -> JResult<UserClass> {
        Ok(UserClass {
            name: "Tom".to_string(),
        })
    }
}

#[cfg(test)]
pub mod tests {
    use super::input_types::*;
    use super::return_types::*;
    use java_bindgen::prelude::*;

    #[test]
    pub fn should_compile() {}

    #[test_jvm]
    fn should_input_jshort<'a>(_: &mut JNIEnv<'a>, env: JNIEnv<'a>, class: JClass) -> JResult<()> {
        let result = Java_com_test_macro_TestMacro_input_u8(env, class, 12);
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
        let result = Java_com_test_macro_TestMacro_input_byte_array(env, class, array);
        assert_eq!(result.is_null(), true);
        Ok(())
    }

    #[test_jvm]
    fn should_return_jshort<'a>(
        test_env: &mut JNIEnv<'a>,
        env: JNIEnv<'a>,
        class: JClass,
    ) -> JResult<()> {
        let result = Java_com_test_macro_TestMacro_returns_jshort(env, class);
        assert_eq!(result.into_rust(test_env)?, 16);
        Ok(())
    }
}
