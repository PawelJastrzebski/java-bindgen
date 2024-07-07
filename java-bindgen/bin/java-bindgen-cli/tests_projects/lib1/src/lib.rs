use java_bindgen::prelude::*;

#[derive(Clone)]
struct User {
    name: String,
    age: i32,
}

impl<'local> IntoJavaType<'local> for User {
    type JType = JObject<'local>;

    fn into_java(self, env: &mut jni::JNIEnv<'local>) -> JResult<Self::JType> {
        let p1 = env.new_string(&self.name).j_catch(env)?;
        let p1 = JValue::Object(&p1);
        let p2 = JValue::Int(self.age);

        // let class = env.find_class("java/lang/Integer").unwrap();
        // let Some(p2) = env.new_object(class, "(Ljava/lang/Integer;)V", &[p2]).j_catch(env) else {
        //     return None;
        // };
        // let p2 = JValue::Object(&p2);

        let class = env.find_class("com/test/User").j_catch(env)?;
        env.new_object(class, "(Ljava/lang/String;I)V", &[p1, p2])
            .j_catch(env)
    }
}

#[java_bindgen_raw]
fn user<'local>(mut env: JNIEnv<'local>, _class: JClass<'local>) -> JResult<JObject<'local>> {
    let user = User {
        name: "Hello".to_string(),
        age: 22,
    };

    user.into_java(&mut env)
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
    env.new_object(class, "(Ljava/lang/String;)V", &[JValue::Object(&r)])
        .j_catch(&mut env)
}

#[java_bindgen_raw]
fn hello<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    input: JString<'local>,
) -> JResult<JString<'local>> {
    let input = input.into_rust(&mut env)?;
    let r = format!("Hello Java Bindgen, {}!", input);
    r.into_java(&mut env)
}

#[java_bindgen_raw]
pub fn helloByte<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass,
    input: JByteArray<'local>,
) -> JResult<JByteArray<'local>> {
    let _input = input.into_rust(&mut env)?;
    let buf = [1; 2000];
    buf.into_java(&mut env)
}
