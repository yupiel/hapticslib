#![allow(non_camel_case_types, unused)]

use std::ffi::{c_char, c_double, c_int, c_void, CStr};

// ---------- C ----------

/// Sync implementation for `*const c_char`, equivalent of C's `const char*` for safe ffi.
#[repr(transparent)]
pub struct SyncPtr(pub *const c_char);
unsafe impl Sync for SyncPtr {}

/// This is currently a nightly feature in rust core::ffi::c_size_t
/// https://github.com/rust-lang/rust/issues/88345
///
/// TODO: remove once implemented
pub type c_size_t = usize;

// ---------- lua ----------

/// The lua state is required by every lua function, however we never directly access it.
///
/// Therefore it's defined as a zero size struct.
///
/// Attempts to access *any* state internal value directly **WILL** fail.
pub struct lua_State;

/// Represents numbers with double floating point precision in lua.
///
/// Equivalent of `f64` in rust or `double` in a bunch of other languages.
pub type lua_Number = c_double;

/// Represents 32-bit integers in lua.
///
/// Equivalent of `i32` in rust or `int` in a bunch of other languages.
pub type lua_Integer = i32;

/// Signature of a C-compatible function. The only thing lua accepts.
pub type lua_CFunction = unsafe extern "C-unwind" fn(L: *mut lua_State) -> c_int;

#[repr(C)]
pub struct luaL_Reg {
    pub name: *const c_char,
    pub func: lua_CFunction,
}

// ---------- SuperBLT ----------

/// Function that is injected by SuperBLT and provides function pointers based on the name parameter.
///
/// [Details](https://gitlab.com/znixian/payday2-superblt/-/blob/master/platforms/w32/plugins/plugins-w32.cpp?ref_type=heads#L87)
pub type lua_access_func = extern "C" fn(*const c_char) -> *mut c_void;
