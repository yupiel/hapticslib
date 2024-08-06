#![allow(non_camel_case_types, unused)]

use std::ffi::{c_double, c_int, c_void};

pub static LUA_TNONE: c_int = -1;
pub static LUA_TNIL: c_int = 0;
pub static LUA_TBOOLEAN: c_int = 1;
pub static LUA_TLIGHTUSERDATA: c_int = 2;
pub static LUA_TNUMBER: c_int = 3;
pub static LUA_TSTRING: c_int = 4;
pub static LUA_TTABLE: c_int = 5;
pub static LUA_TFUNCTION: c_int = 6;
pub static LUA_TUSERDATA: c_int = 7;
pub static LUA_TTHREAD: c_int = 8;

pub static LUA_MINSTACK: c_int = 20;

pub struct lua_State;
pub type lua_Number = c_double;
pub type lua_Integer = i32;
pub type lua_CFunction = unsafe extern "C-unwind" fn(L: *mut lua_State) -> c_int;