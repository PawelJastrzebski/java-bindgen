use std::{backtrace::Backtrace, fmt::Debug, rc::Rc};

use crate::prelude::IntoJavaType;

#[derive(Debug, Clone)]
pub enum JExceptionClass {
    RuntimeException,
    ArithmeticException,
    ArrayIndexOutOfBoundsException,
    ArrayStoreException,
    ClassCastException,
    IllegalArgumentException,
    IllegalMonitorStateException,
    IllegalStateException,
    IllegalThreadStateException,
    IndexOutOfBoundsException,
    NegativeArraySizeException,
    NullPointerException,
    NumberFormatException,
    SecurityException,
    StringIndexOutOfBounds,
    UnsupportedOperationException,
    ClassNotFoundException,
    CloneNotSupportedException,
    IllegalAccessException,
    InstantiationException,
    InterruptedException,
    NoSuchFieldException,
    NoSuchMethodException,
}

impl JExceptionClass {
    pub fn get_class_path(&self) -> String {
        format!("java/lang/{:?}", self)
    }
}

// Convert JNI Error to JavaException (class)
impl From<&jni::errors::Error> for JExceptionClass {
    fn from(value: &jni::errors::Error) -> Self {
        match value {
            jni::errors::Error::WrongJValueType(_, _) => JExceptionClass::ClassCastException,
            jni::errors::Error::InvalidCtorReturn => JExceptionClass::IllegalArgumentException,
            jni::errors::Error::InvalidArgList(_) => JExceptionClass::IllegalArgumentException,
            jni::errors::Error::MethodNotFound { name: _, sig: _ } => {
                JExceptionClass::NoSuchMethodException
            }
            jni::errors::Error::FieldNotFound { name: _, sig: _ } => {
                JExceptionClass::NoSuchFieldException
            }
            jni::errors::Error::JavaException => JExceptionClass::RuntimeException,
            jni::errors::Error::JNIEnvMethodNotFound(_) => JExceptionClass::NoSuchMethodException,
            jni::errors::Error::NullPtr(_) => JExceptionClass::NullPointerException,
            jni::errors::Error::NullDeref(_) => JExceptionClass::NullPointerException,
            jni::errors::Error::TryLock => JExceptionClass::IllegalStateException,
            jni::errors::Error::JavaVMMethodNotFound(_) => JExceptionClass::NoSuchMethodException,
            jni::errors::Error::FieldAlreadySet(_) => JExceptionClass::IllegalStateException,
            jni::errors::Error::ThrowFailed(_) => JExceptionClass::IllegalStateException,
            jni::errors::Error::ParseFailed(_, _) => JExceptionClass::IllegalStateException,
            jni::errors::Error::JniCall(_) => JExceptionClass::UnsupportedOperationException,
        }
    }
}

impl std::fmt::Display for JExceptionClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<JExceptionClass> for JException {
    fn from(val: JExceptionClass) -> Self {
        let msg = &format!("{}", val);
        JException::from_class_and_msg(val, msg)
    }
}

#[derive(Clone)]
pub struct JException {
    pub class: JExceptionClass,
    pub error: Rc<dyn std::error::Error>,
}

impl JException {
    pub fn from_class_and_msg(class: JExceptionClass, msg: &str) -> Self {
        let error: Box<dyn std::error::Error> = msg.to_string().into();
        let error = Rc::<dyn std::error::Error>::from(error);
        Self { class, error }
    }

    pub fn from_std<E: std::error::Error + 'static>(error: E) -> Self {
        Self {
            class: JExceptionClass::RuntimeException,
            error: Rc::new(error),
        }
    }

    pub fn from_std_with_class<E: std::error::Error + 'static>(
        error: E,
        j_class: JExceptionClass,
    ) -> Self {
        Self {
            class: j_class,
            error: Rc::new(error),
        }
    }
}

impl jni::errors::ToException for JException {
    fn to_exception(&self) -> jni::errors::Exception {
        jni::errors::Exception {
            class: self.class.get_class_path(),
            msg: format!("{}", self.error),
        }
    }
}

impl<E> From<E> for JException
where
    E: std::error::Error + 'static,
{
    fn from(error: E) -> Self {
        JException::from_std(error)
    }
}

impl std::fmt::Display for JException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.error)
    }
}

impl std::fmt::Debug for JException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.error)
    }
}

pub type JResult<T, E = JException> = core::result::Result<T, E>;

pub fn j_result_handler<'a, T, R: Default>(
    result: JResult<T, JException>,
    env: &mut jni::JNIEnv<'a>,
) -> R
where
    T: IntoJavaType<'a, R> + Default,
{
    match result {
        Ok(ok) => match ok.into_java(env) {
            Ok(ok) => ok,
            Err(err) => {
                env.j_throw_exception(err);
                Default::default()
            }
        },
        Err(err) => {
            env.j_throw_exception(err);
            Default::default()
        }
    }
}

// JNIEnv Util

macro_rules! jthrow {
    ( $env:expr => $j_class:expr , $message:tt) => {
        let error = $env.exception_occurred().unwrap_or_default();
        if error.is_null() {
            let message = format!(
                "\nRust Error:  {}\nRust Backtrace:\n{}\n",
                &$message,
                Backtrace::force_capture()
            );
            $env.throw_new(($j_class).get_class_path(), &message).ok();
        }
    };
}

#[allow(non_snake_case)]
pub trait JNIEnvUtils {
    fn j_throw_cause(&mut self, j_class: JExceptionClass, cause: &impl std::error::Error);
    fn j_throw(&mut self, j_class: JExceptionClass);
    fn j_throw_msg(&mut self, message: &str);
    fn j_throw_exception(&mut self, ex: JException);

    fn get_string_owned(
        &mut self,
        jstring: &jni::objects::JString<'_>,
    ) -> jni::errors::Result<String>;
}

impl<'local> JNIEnvUtils for jni::JNIEnv<'local> {
    fn j_throw_msg(&mut self, message: &str) {
        jthrow!( self => &JExceptionClass::RuntimeException, message);
    }
    fn j_throw_cause(&mut self, j_class: JExceptionClass, cause: &impl std::error::Error) {
        jthrow!( self => j_class, cause);
    }
    fn j_throw(&mut self, j_class: JExceptionClass) {
        jthrow!( self => j_class, j_class);
    }
    fn j_throw_exception(&mut self, ex: JException) {
        let c = ex.class;
        let e = ex.error;
        jthrow!( self => c, e);
    }

    fn get_string_owned(
        &mut self,
        jstring: &jni::objects::JString<'_>,
    ) -> jni::errors::Result<String> {
        let string = self.get_string(jstring)?;
        string
            .to_str()
            .map(|s| s.to_string())
            .map_err(|_| jni::errors::Error::WrongJValueType("JString", "-"))
    }
}

/// Propagate error to Java
pub trait JavaCatch<'local, T> {
    fn j_catch(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<T>;
}

impl<'local, T, E: std::error::Error + 'static> JavaCatch<'local, T> for JResult<T, E> {
    fn j_catch(self, env: &mut jni::JNIEnv<'local>) -> crate::JResult<T> {
        match self {
            Ok(ok) => Ok(ok),
            Err(err) => {
                jthrow!(env => JExceptionClass::RuntimeException, err);
                Err(JException::from_std(err))
            }
        }
    }
}

pub trait JavaCatchINI<'local, T> {
    fn j_catch_ini(self, env: &mut jni::JNIEnv<'local>, msg: &str) -> crate::JResult<T>;
}

impl<'local, T> JavaCatchINI<'local, T> for Result<T, jni::errors::Error> {
    fn j_catch_ini(self, env: &mut jni::JNIEnv<'local>, msg: &str) -> crate::JResult<T> {
        match self {
            Ok(ok) => Ok(ok),
            Err(err) => {
                let exception = JExceptionClass::from(&err);
                let message = format!("{err}\n   Cause: {msg}");
                jthrow!(env => exception, message);
                Err(JException::from_class_and_msg(exception, &message))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate as java_bindgen;
    use crate::prelude::*;

    #[allow(non_snake_case)]
    fn user<'a>(_: &mut JNIEnv<'a>, _class: JClass<'_>) -> JResult<String> {
        Ok("ok".to_string())
    }

    #[allow(unused_mut, non_snake_case)]
    pub extern "system" fn Java_com_test_Lib1_user<'a>(
        mut env: JNIEnv<'a>,
        _class: JClass<'_>,
    ) -> jni::objects::JString<'a> {
        let r = user(&mut env, _class);
        j_result_handler(r, &mut env)
    }

    #[test_jvm]
    fn should_return_ok<'a>(
        test_env: &mut JNIEnv<'a>,
        env: JNIEnv<'a>,
        class: JClass,
    ) -> JResult<()> {
        // Call Java function
        let result = Java_com_test_Lib1_user(env, class);
        // Convert result into rust
        let result: String = result.into_rust(test_env)?;
        assert_eq!(&result, "ok");
        Ok(())
    }
}
