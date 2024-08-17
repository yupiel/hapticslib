#![allow(unused)]

use std::ffi::CString;

use log::Level;

#[allow(non_camel_case_types)]
#[repr(C)]
pub enum LogType {
    LOGGING_FUNC = 0,
    LOGGING_LOG,
    LOGGING_LUA,
    LOGGING_WARN,
    LOGGING_ERROR,
}

/// Global instance of the PD2HOOK_LOG function exported by SuperBLT.
///
/// This is held separately from the global SuperBLT instance for convenience of use.
pub static PD2HOOK_LOG: std::sync::OnceLock<
    fn(
        message: *const std::ffi::c_char,
        level: std::ffi::c_int,
        file: *const std::ffi::c_char,
        line: std::ffi::c_int,
    ),
> = std::sync::OnceLock::new();

/// Raw Logging function that is used internally by the other specialized logging functions.
///
/// * `level` (required) - Sets the text color when outputting to the console.
/// * `file`  (optional) - Sets the filename in the logged line of text.
/// * `line`  (optional) - Sets the line number in the logged line of text.
/// * `[var]` (required) - Variable number of arguments that work similar to the format! macro.
macro_rules! PD2HOOK_LOG_LEVEL {
    ($level:path; $($arg:tt)*) => {
        let log_message_cstring = std::ffi::CString::new(std::fmt::format(format_args!($($arg)*))).unwrap();
        let file_cstring = std::ffi::CString::new(file!()).unwrap();

        $crate::superblt::pd2_logger::PD2HOOK_LOG.get().unwrap()(
            log_message_cstring.as_ptr(),
            $level.into(),
            file_cstring.as_ptr(),
            line!() as std::ffi::c_int,
        )
    };
    ($level:path, $file:literal; $($arg:tt)*) => {
        let log_message_cstring = std::ffi::CString::new(std::fmt::format(format_args!($($arg)*))).unwrap();
        let file_cstring = std::ffi::CString::new(file!()).unwrap();

        $crate::superblt::pd2_logger::PD2HOOK_LOG.get().unwrap()(
            log_message_cstring.as_ptr(),
            $level.into(),
            file_cstring.as_ptr(),
            line!() as std::ffi::c_int,
        )
    };
    ($level:path, $file:literal, $line:literal; $($arg:tt)*) => {
        let log_message_cstring = std::ffi::CString::new(std::fmt::format(format_args!($($arg)*))).unwrap();
        let file_cstring = std::ffi::CString::new($file).unwrap();

        $crate::superblt::pd2_logger::PD2HOOK_LOG.get().unwrap()(
            log_message_cstring.as_ptr(),
            $level.into(),
            file_cstring.as_ptr(),
            $line as std::ffi::c_int,
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

// Rust log implementation for convenience

// Necessary mapping because these are more or less in reverse order or don't exist
impl From<log::Level> for LogType {
    fn from(value: log::Level) -> Self {
        match value {
            Level::Error => LogType::LOGGING_ERROR,
            Level::Warn => LogType::LOGGING_WARN,
            Level::Info => LogType::LOGGING_LOG,
            _ => LogType::LOGGING_LOG,
        }
    }
}

impl Into<std::ffi::c_int> for LogType {
    fn into(self) -> std::ffi::c_int {
        self as std::ffi::c_int
    }
}

pub static RUST_PD2_LOGGER: PD2Logger = PD2Logger;

pub struct PD2Logger;

impl log::Log for PD2Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= Level::Debug
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            let message_cstring = CString::new(format!("{}", record.args())).unwrap();
            let file_cstring = CString::new(format!("{}", record.file().unwrap_or(""))).unwrap();

            PD2HOOK_LOG.get().unwrap()(
                message_cstring.as_ptr(),
                LogType::from(record.level()).into(),
                file_cstring.as_ptr(),
                record.line().unwrap_or(0) as i32,
            );
        }
    }

    // Not necessary since this is done on the BLT end and we just pass down the logs
    fn flush(&self) {}
}
