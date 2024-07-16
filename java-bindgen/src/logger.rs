use crate::{interop::*, prelude::JavaCatch, JResult};

#[derive(Default)]
pub struct JLoggerCore<'a> {
    logger_obj: jni::objects::JObject<'a>,
}

impl<'a> JLoggerCore<'a> {
    pub fn new(env: &mut jni::JNIEnv<'a>, lib_class_path: &str) -> JResult<Self> {
        let class = env.find_class(lib_class_path).j_catch(env)?;
        let logger = env
            .get_static_field(&class, "logger", "Lorg/slf4j/Logger;")
            .j_catch(env)?;
        let logger_obj = logger.l().j_catch(env)?;

        Ok(JLoggerCore { logger_obj })
    }

    fn _log<T: Into<String>>(&self, msg: T, level: &str, env: &mut jni::JNIEnv<'a>) {
        let has_exception = env.exception_check().unwrap_or_default();

        if let Ok(msg) = msg.into().into_j_value(env) {
            env.call_method(
                &self.logger_obj,
                level,
                "(Ljava/lang/String;)V",
                &[msg.borrow()],
            )
            .j_catch(env)
            .ok();
        }

        if !has_exception {
            env.exception_clear().ok();
        }
    }

    pub fn info<T: Into<String>>(&self, msg: T, env: &mut jni::JNIEnv<'a>) {
        self._log(msg, "info", env);
    }

    pub fn warn<T: Into<String>>(&self, msg: T, env: &mut jni::JNIEnv<'a>) {
        self._log(msg, "warn", env);
    }

    pub fn error<T: Into<String>>(&self, msg: T, env: &mut jni::JNIEnv<'a>) {
        self._log(msg, "error", env);
    }

    pub fn debug<T: Into<String>>(&self, msg: T, env: &mut jni::JNIEnv<'a>) {
        self._log(msg, "debug", env);
    }

    pub fn trace<T: Into<String>>(&self, msg: T, env: &mut jni::JNIEnv<'a>) {
        self._log(msg, "trace", env);
    }
}
