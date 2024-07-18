use std::sync::{Arc, OnceLock};

use jni::objects::JClass;

static JVM_CACHE: OnceLock<Arc<jni::JavaVM>> = OnceLock::new();

pub fn get_jvm() -> Arc<jni::JavaVM> {
    JVM_CACHE
        .get_or_init(|| {
            let args = jni::InitArgsBuilder::new()
                .version(jni::JNIVersion::V8)
                .option("-Xcheck:jni")
                .build()
                .expect("Failed to parse JVM args");
            let jvm = jni::JavaVM::new(args).expect("Failed to start JVM");
            Arc::new(jvm)
        })
        .clone()
}

pub fn run_in_jvm(
    fun: for<'a> fn(&mut jni::JNIEnv<'a>, jni::JNIEnv<'a>, JClass) -> crate::exception::JResult<()>,
) -> crate::JResult<()> {
    let jvm = get_jvm();
    let mut test_env = jvm.attach_current_thread_as_daemon()?;
    let env = jvm.attach_current_thread_as_daemon()?;
    fun(&mut test_env, env, JClass::default())
}
