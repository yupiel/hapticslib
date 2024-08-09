#![allow(non_camel_case_types, unused)]

use std::ffi::{c_char, c_double, c_int, c_void};

// ---------- C ----------

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

pub type lua_Number = c_double;
pub type lua_Integer = i32;
pub type lua_CFunction = unsafe extern "C-unwind" fn(L: *mut lua_State) -> c_int;

#[repr(C)]
pub struct luaL_Reg {
    pub name: *const c_char,
    pub func: lua_CFunction,
}

// ---------- SuperBLT ----------

/// Function that is injected by SuperBLT and provides function pointers based on the name parameter.
pub type lua_access_func = extern "C" fn(*const c_char) -> *mut c_void;
