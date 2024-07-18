use java_bindgen::prelude::*;

#[derive(IntoJava)]
struct UserClass {
    age: i32,
    name: String,
    byte_value: u8,
    class_byte: JByte,
    array: Vec<u8>
}

#[java_bindgen_raw(return = UserClass)]
fn user<'a>(mut env: JNIEnv<'a>, _class: JClass<'_>) -> JResult<JObject<'a>> {
    let user = UserClass {
        name: "Hello".to_string(),
        age: 220,
        byte_value: 1,
        class_byte: JByte(20),
        array: vec![1, 2, 3]
    };

    user.into_java(&mut env)
}

#[java_bindgen]
fn user_raw<'a>(_env: &mut JNIEnv<'a>) -> JResult<String> {
    Ok("String ok".to_string())
}

#[java_bindgen]
fn user1<'a>(_env: &mut JNIEnv<'a>) -> JResult<jbyte> {
    Ok(127)
}

#[java_bindgen_raw]
pub fn ethrow<'local>(
    mut env: jni::JNIEnv<'local>,
    _class: jni::objects::JClass<'local>,
) -> JResult<JObject<'local>> {
    let r: String = Err(std::io::Error::other("always throw error")).j_catch(&mut env)?;
    let r = env.new_string(r).j_catch(&mut env)?;

    // env.get_field_id(class, name, sig)
    // let init = env.get_method_id(class, "<init>", "(Ljava/lang/String)Ljava/lang/String").unwrap();
    // let obj = env.new_object(class, "<init>", &[
    //     JValue::Object(&r)
    // ]).unwrap();

    let class = env.find_class("java/lang/String").unwrap();
    env.new_object(class, "(Ljava/lang/String;)V", &[jni::objects::JValue::Object(&r)])
        .j_catch(&mut env)
}


#[java_bindgen]
fn hello_1<'aa>(env: &mut JNIEnv<'aa>, input: JString<'aa>) -> JResult<String> {
    let input: String = input.into_rust(env)?;
    Ok(format!("Hello Java Bindgen 222, {}!", input))
}

#[java_bindgen]
fn hello_2<'aa>(env: &mut JNIEnv<'aa>, input: String) -> JResult<String> {
    Ok(format!("Hello Java Bindgen 222, {}!", input))
}

#[java_bindgen]
fn hello(input: String) -> JResult<String> {
    Ok(format!("Hello Java java_bindgen 4444 FullAuto, {}!", input))
}


// #[java_bindgen_raw]
// fn hello<'local>(
//     mut env: JNIEnv<'local>,
//     _class: JClass<'local>,
//     input: JString<'local>,
// ) -> JResult<JString<'local>> {
//     let input = input.into_rust(&mut env)?;
//     let r = format!("Hello Java Bindgen, {}!", input);
//     r.into_java(&mut env)
// }

#[java_bindgen_raw]
pub fn helloByte<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass,
    input: jni::objects::JByteArray<'local>,
) -> JResult<JByteArray<'local>> {
    let _input = input.into_rust(&mut env)?;
    let buf = [1; 2000];
    buf.into_java(&mut env)
}
