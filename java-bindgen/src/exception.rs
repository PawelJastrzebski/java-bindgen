use std::{backtrace::Backtrace, fmt::Debug, rc::Rc};

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

impl std::fmt::Display for JExceptionClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for JExceptionClass {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

#[derive(Clone)]
pub struct JException {
    pub class: JExceptionClass,
    pub error: Rc<dyn std::error::Error>,
}

impl JException {
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
    E: std::error::Error + Send + Sync + 'static,
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

// Macro Util

// #[macro_export]
// macro_rules! java_try {
//     ($($code:tt)*) => {
//         let mut _inner = || {
//             $($code)*
//         };
//         let Some(res) = _inner() else {
//             return Default::default()
//         };
//         return res;
//     };
// }

// JNIEnv Util

macro_rules! jthrow {
    ( $env:expr => $j_class:expr , $message:tt) => {
        let error = $env.exception_occurred().unwrap_or_default();
        if error.is_null() {
            let message = format!(
                "\"{}\" [Rust Error]\nRust Backtrace:\n{}\n",
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
    fn j_throw_msg(&mut self, j_class: &JExceptionClass, message: &str);
}

impl<'local> JNIEnvUtils for jni::JNIEnv<'local> {
    fn j_throw_msg(&mut self, j_class: &JExceptionClass, message: &str) {
        jthrow!( self => j_class, message);
    }
    fn j_throw_cause(&mut self, j_class: JExceptionClass, cause: &impl std::error::Error) {
        jthrow!( self => j_class, cause);
    }
    fn j_throw(&mut self, j_class: JExceptionClass) {
        jthrow!( self => j_class, j_class);
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
                Err(crate::exception::JException::from_std(err))
            }
        }
    }
}
