#![allow(non_camel_case_types, unused)]

use std::ffi::{c_char, c_double, c_int, c_void};

// ---------- C ----------

/// This is currently a nightly feature in rust core::ffi::c_size_t
/// https://github.com/rust-lang/rust/issues/88345
///
/// TODO: remove once implemented
pub type c_size_t = usize;

// ---------- lua ----------
pub struct lua_State;

pub type lua_Number = c_double;
pub type lua_Integer = i32;
pub type lua_CFunction = unsafe extern "C-unwind" fn(L: *mut lua_State) -> c_int;

#[repr(C)]
pub struct luaL_Reg {
    pub name: *const c_char,
    pub func: lua_CFunction,
}

// ---------- superblt ----------

pub type lua_access_func = extern "C" fn(*const c_char) -> *mut c_void;
