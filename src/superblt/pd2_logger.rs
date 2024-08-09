#![allow(unused)]
use std::{
    ffi::{c_char, c_int},
    sync::OnceLock,
};

#[allow(non_camel_case_types)]
#[repr(C)]
pub enum LogType {
    LOGGING_FUNC = 0,
    LOGGING_LOG,
    LOGGING_LUA,
    LOGGING_WARN,
    LOGGING_ERROR,
}

pub static PD2HOOK_LOG: OnceLock<
    fn(message: *const c_char, level: c_int, file: *const c_char, line: c_int),
> = OnceLock::new();

macro_rules! PD2HOOK_LOG_LEVEL {
    ($level:path; $($arg:tt)*) => {
        let log_message_cstring = CString::new(std::fmt::format(format_args!($($arg)*))).unwrap();
        let file_cstring = CString::new(file!()).unwrap();

        $crate::superblt::pd2_logger::PD2HOOK_LOG.get().unwrap()(
            log_message_cstring.as_ptr(),
            $level as std::ffi::c_int,
            file_cstring.as_ptr(),
            line!() as c_int,
        )
    };
    ($level:path, $file:literal; $($arg:tt)*) => {
        let log_message_cstring = CString::new(std::fmt::format(format_args!($($arg)*))).unwrap();
        let file_cstring = CString::new(file!()).unwrap();

        $crate::superblt::pd2_logger::PD2HOOK_LOG.get().unwrap()(
            log_message_cstring.as_ptr(),
            $level as std::ffi::c_int,
            file_cstring.as_ptr(),
            line!() as c_int,
        )
    };
    ($level:path, $file:literal, $line:literal; $($arg:tt)*) => {
        let log_message_cstring = CString::new(std::fmt::format(format_args!($($arg)*))).unwrap();
        let file_cstring = CString::new($file).unwrap();

        $crate::superblt::pd2_logger::PD2HOOK_LOG.get().unwrap()(
            log_message_cstring.as_ptr(),
            $level as std::ffi::c_int,
            file_cstring.as_ptr(),
            $line as c_int,
        )
    }
}

pub(crate) use PD2HOOK_LOG_LEVEL;

macro_rules! PD2HOOK_LOG_FUNC {
    ($($arg:tt)*) => {
        $crate::superblt::pd2_logger::PD2HOOK_LOG_LEVEL!(
            $crate::superblt::pd2_logger::LogType::LOGGING_FUNC,
            file!();
            $($arg)*
        )
    };
}
pub(crate) use PD2HOOK_LOG_FUNC;

macro_rules! PD2HOOK_LOG_LOG {
    ($($arg:tt)*) => {
        $crate::superblt::pd2_logger::PD2HOOK_LOG_LEVEL!(
            $crate::superblt::pd2_logger::LogType::LOGGING_LOG;
            $($arg)*
        )
    };
}
pub(crate) use PD2HOOK_LOG_LOG;

macro_rules! PD2HOOK_LOG_LUA {
    ($($arg:tt)*) => {
        $crate::superblt::pd2_logger::PD2HOOK_LOG_LEVEL!(
            $crate::superblt::pd2_logger::LogType::LOGGING_LUA,
            "",
            -1;
            $($arg)*
        )
    };
}
pub(crate) use PD2HOOK_LOG_LUA;

macro_rules! PD2HOOK_LOG_WARN {
    ($($arg:tt)*) => {
        $crate::superblt::pd2_logger::PD2HOOK_LOG_LEVEL!(
            $crate::superblt::pd2_logger::LogType::LOGGING_WARN;
            $($arg)*
        )
    };
}
pub(crate) use PD2HOOK_LOG_WARN;

macro_rules! PD2HOOK_LOG_ERROR {
    ($($arg:tt)*) => {
        $crate::superblt::pd2_logger::PD2HOOK_LOG_LEVEL!(
            $crate::superblt::pd2_logger::LogType::LOGGING_ERROR;
            $($arg)*
        )
    };
}
pub(crate) use PD2HOOK_LOG_ERROR;

macro_rules! PD2HOOK_LOG_PANIC {
    ($($arg:tt)*) => {
        $crate::superblt::pd2_logger::PD2HOOK_LOG_LEVEL!(
            $crate::superblt::pd2_logger::LogType::LOGGING_ERROR,
            "",
            0;
            $($arg)*
        );
    };
}
pub(crate) use PD2HOOK_LOG_PANIC;
