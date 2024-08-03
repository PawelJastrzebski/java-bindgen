// Test case (All macro should compile)

pub mod return_types {
    use java_bindgen::prelude::*;

    #[java_bindgen]
    fn returns_nothing() {}

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

pub mod pass_types {
    use java_bindgen::prelude::*;

    #[java_bindgen]
    fn pass_u8(input: u8) -> JResult<u8> {
        Ok(input)
    }

    #[java_bindgen]
    fn pass_i8(input: i8) -> JResult<i8> {
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

pub mod return_custom_type {
    use java_bindgen::prelude::*;

    #[derive(Default, IntoJava, IntoRust)]
    struct UserClass {
        name: String,
        age: i32,
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

    #[java_bindgen]
    fn pass_user_list(name: String, user: JList<UserClass>) -> JResult<JList<UserClass>> {
        let mut users = user;
        let mut vec = vec![UserClass {
            age: 10,
            name: format!("1{}", name),
        }];
        vec.append(&mut users.0);

        Ok(JList(vec))
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

    #[java_bindgen]
    fn pass_all_types_list(object: JList<AllJavaTypes>) -> JResult<JList<AllJavaTypes>> {
        Ok(object)
    }

    #[derive(Default, IntoJava, IntoRust)]
    struct JavaClassWrappers {
        java_b: JByte,
        java_s: JShort,
        java_i: JInt,
        java_l: JLong,
        java_f: JFloat,
        java_d: JDouble,
        java_c: JChar,
        java_bool: JBoolean,
    }

    #[java_bindgen]
    fn pass_java_class_wrappers(object: JavaClassWrappers) -> JResult<JavaClassWrappers> {
        Ok(object)
    }

    #[java_bindgen]
    fn pass_java_class_wrappers_list(
        object: JList<JavaClassWrappers>,
    ) -> JResult<JList<JavaClassWrappers>> {
        Ok(object)
    }

    #[derive(Default, JavaClass)]
    struct EmbededTypes {
        parent: EmbededNode,
        children: JList<EmbededNode>,
    }

    #[derive(Default, JavaClass)]
    struct EmbededNode {
        node_id: i32,
    }

    #[java_bindgen]
    fn pass_java_class_embeded(object: EmbededTypes) -> JResult<EmbededTypes> {
        Ok(object)
    }
}

pub mod return_optional {
    use java_bindgen::prelude::*;

    #[java_bindgen]
    fn return_int_optional_some() -> Option<i32> {
        Some(10)
    }

    #[java_bindgen]
    fn return_int_optional_none() -> Option<i32> {
        None
    }

    #[java_bindgen]
    fn return_JInt_optional_some() -> Option<JInt> {
        Some(JInt(10))
    }

    #[java_bindgen]
    fn return_JInt_optional_none() -> Option<JInt> {
        None
    }

    #[java_bindgen]
    fn return_void_optional_some() -> Option<()> {
        Some(())
    }

    #[java_bindgen]
    fn return_void_optional_none() -> Option<()> {
        None
    }

    #[java_bindgen]
    fn return_bool_optional_some() -> Option<bool> {
        Some(true)
    }

    #[java_bindgen]
    fn return_bool_optional_none() -> Option<bool> {
        None
    }

    #[java_bindgen]
    fn return_char_optional_some() -> Option<char> {
        Some('j')
    }

    #[java_bindgen]
    fn return_char_optional_none() -> Option<char> {
        None
    }
}

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

pub mod raw_types_order {
    use java_bindgen::prelude::*;

    #[java_bindgen]
    fn test_raw_types_1<'a>(e: &mut JNIEnv<'a>, name: String) -> JResult<String> {
        let _ = e;
        Ok(name)
    }

    #[java_bindgen]
    fn test_raw_types_2<'a>(name: String, env: &mut JNIEnv<'a>) -> JResult<String> {
        let _ = env;
        let _ = name;
        Ok(name)
    }

    #[java_bindgen]
    fn test_raw_types_3<'a>(_c: JClass<'a>, env: &mut JNIEnv<'a>, name: String) -> JResult<String> {
        let _ = env;
        let _ = _c;
        Ok(name)
    }

    #[java_bindgen]
    fn test_raw_types_4<'a>(name: String, cl: JClass<'a>, env: &mut JNIEnv<'a>) -> JResult<String> {
        let _ = env;
        let _ = cl;
        Ok(name)
    }

    #[java_bindgen]
    fn test_raw_types_5<'a>(c: JClass<'a>, n: String, e: &mut JNIEnv<'a>) -> JResult<String> {
        let _ = e;
        let _ = c;
        Ok(n)
    }

    #[java_bindgen]
    fn test_raw_types_6<'a>(_: JClass<'a>, name: String, env: &mut JNIEnv<'a>) -> JResult<String> {
        let _ = env;
        Ok(name)
    }

    #[java_bindgen]
    fn test_raw_types_7<'a>(_: JClass<'a>, name: String, _: &mut JNIEnv<'a>) -> JResult<String> {
        Ok(name)
    }
}

pub mod raw_return_type {
    use java_bindgen::prelude::*;

    #[derive(IntoJava, Default)]
    struct EmptyClass {}

    #[java_bindgen(return = EmptyClass)]
    fn raw_return_object<'a>(env: &mut JNIEnv<'a>, _class: JClass<'_>) -> JResult<JObject<'a>> {
        let empty = EmptyClass {};
        empty.into_java(env)
    }

    #[java_bindgen(return = String)]
    fn raw_return_string_1<'a>(env: &mut JNIEnv<'a>, _class: JClass<'_>) -> JResult<JString<'a>> {
        "Hello".into_java(env)
    }

    #[java_bindgen]
    fn raw_return_string_2<'a>(env: &mut JNIEnv<'a>, _class: JClass<'_>) -> JResult<JString<'a>> {
        "Hello".into_java(env)
    }

    #[java_bindgen]
    fn raw_return_bytes<'a>(env: &mut JNIEnv<'a>, bytes: Vec<u8>) -> JResult<JByteArray<'a>> {
        bytes.into_java(env)
    }
}

pub mod raw_input_type {
    use java_bindgen::prelude::*;

    #[java_bindgen]
    fn raw_input_type_1<'a>(env: &mut JNIEnv<'a>, input: JString<'a>) -> JResult<String> {
        let input_str: String = input.into_rust(env)?;
        Ok(input_str)
    }

    #[java_bindgen]
    fn raw_input_type_2<'a>(env: &mut JNIEnv<'a>, input: JObject<'a>) -> JResult<String> {
        let input_str: String = input.into_rust(env)?;
        Ok(input_str)
    }

    #[java_bindgen]
    fn raw_input_type_3<'a>(env: &mut JNIEnv<'a>, input: JByteArray<'a>) -> JResult<Vec<u8>> {
        let bytes: Vec<u8> = input.into_rust(env)?;
        Ok(bytes)
    }

    #[java_bindgen]
    fn raw_input_type_4<'a>(env: &mut JNIEnv<'a>, input: JObject<'a>) -> JResult<JList<JLong>> {
        let list: JList<JLong> = input.into_rust(env)?;
        Ok(list)
    }
}

pub mod throw_exception {
    use java_bindgen::prelude::*;

    #[java_bindgen]
    fn should_throw_exception_1<'a>(env: &mut JNIEnv<'a>, nr: i32) -> JResult<i32> {
        env.j_throw_msg("[err_message_1]");
        Ok(nr)
    }

    #[java_bindgen]
    fn should_throw_exception_2<'a>(env: &mut JNIEnv<'a>, nr: i32) -> JResult<i32> {
        env.j_throw_cause(
            JExceptionClass::ArithmeticException,
            &std::io::Error::other("[err_message_2]"),
        );
        Ok(nr)
    }

    #[java_bindgen]
    fn should_throw_exception_3<'a>(env: &mut JNIEnv<'a>, nr: i32) -> JResult<i32> {
        env.j_throw(JExceptionClass::IllegalStateException);
        Ok(nr)
    }

    #[java_bindgen]
    fn should_throw_exception_4<'a>(_: &mut JNIEnv<'a>, _nr: i32) -> JResult<i32> {
        Err(JExceptionClass::SecurityException.into())
    }

    #[java_bindgen]
    fn should_throw_exception_5<'a>(env: &mut JNIEnv<'a>, _nr: i32) -> JResult<i32> {
        Err(std::io::Error::other("Always Throw")).j_catch(env)?
    }

    #[java_bindgen]
    fn should_throw_exception_in_order<'a>(env: &mut JNIEnv<'a>, _nr: i32) -> JResult<i32> {
        env.j_throw(JExceptionClass::IndexOutOfBoundsException);
        Err(JExceptionClass::UnsupportedOperationException.into())
    }
}

pub mod pass_list {
    use java_bindgen::prelude::*;

    #[java_bindgen]
    fn pass_list_u8(input: JList<u8>) -> JResult<JList<u8>> {
        Ok(input)
    }

    #[java_bindgen]
    fn pass_list_i8(input: JList<i8>) -> JResult<JList<i8>> {
        Ok(input)
    }

    #[java_bindgen]
    fn pass_list_i16(input: JList<i16>) -> JResult<JList<i16>> {
        Ok(input)
    }

    #[java_bindgen]
    fn pass_list_i32(input: JList<i32>) -> JResult<JList<i32>> {
        Ok(input)
    }

    #[java_bindgen]
    fn pass_list_i64(input: JList<i64>) -> JResult<JList<i64>> {
        Ok(input)
    }

    #[java_bindgen]
    fn pass_list_f32(input: JList<f32>) -> JResult<JList<f32>> {
        Ok(input)
    }

    #[java_bindgen]
    fn pass_list_f64(input: JList<f64>) -> JResult<JList<f64>> {
        Ok(input)
    }

    #[java_bindgen]
    fn pass_list_char(input: JList<char>) -> JResult<JList<char>> {
        Ok(input)
    }

    #[java_bindgen]
    fn pass_list_bool(input: JList<bool>) -> JResult<JList<bool>> {
        Ok(input)
    }

    #[java_bindgen]
    fn pass_list_string(input: JList<String>) -> JResult<JList<String>> {
        Ok(input)
    }

    #[java_bindgen]
    fn pass_list_byte_array(input: JList<Vec<u8>>) -> JResult<JList<Vec<u8>>> {
        Ok(input)
    }

    #[java_bindgen]
    fn pass_list_of_lists(input: JList<JList<i32>>) -> JResult<JList<JList<i32>>> {
        Ok(input)
    }
}

pub mod readme_examples {
    use java_bindgen::prelude::*;

    #[derive(Default, JavaClass)]
    struct Element {
        parent: Node,
        children: JList<Node>,
    }

    #[derive(Default, JavaClass)]
    struct Node {
        node_id: i32,
    }

    #[java_bindgen]
    fn add_new_node(node: Node, element: Element) -> JResult<Element> {
        let mut update = element;
        update.children.add(node);
        Ok(update)
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
        assert_eq!(result, 16);
        Ok(())
    }
}
