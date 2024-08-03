#![allow(non_camel_case_types, non_snake_case, unused)]

use std::ffi::CString;

mod superblt_flat;

use crate::superblt_flat::{
    luaL_checkinteger, lua_State, lua_createtable, lua_pushcclosure, lua_pushinteger,
    lua_pushstring, lua_setfield,
};

type lua_access_func = extern "C" fn(*const std::ffi::c_char) -> std::ffi::c_void;

#[no_mangle]
pub extern "C" fn SuperBLT_Plugin_Setup(get_exposed_function: lua_access_func) {
    #![deny(unconditional_recursion)]
    #[link(name="sblt_plugin")]
    extern "C" {
        fn SuperBLT_Plugin_Setup(get_exposed_function: lua_access_func);
    }
    unsafe { SuperBLT_Plugin_Setup(get_exposed_function) }
}
#[no_mangle]
pub extern "C" fn SuperBLT_Plugin_Init_State(L: *mut lua_State) {
    #![deny(unconditional_recursion)]
    #[link(name="sblt_plugin")]
    extern "C" {
        fn SuperBLT_Plugin_Init_State(L: *mut lua_State);
    }
    unsafe { SuperBLT_Plugin_Init_State(L) }
}
#[no_mangle]
pub extern "C" fn SuperBLT_Plugin_Update() {
    #![deny(unconditional_recursion)]
    #[link(name="sblt_plugin")]
    extern "C" {
        fn SuperBLT_Plugin_Update();
    }
    unsafe { SuperBLT_Plugin_Update() }
}
#[no_mangle]
pub extern "C" fn SuperBLT_Plugin_PushLua(L: *mut lua_State) {
    #![deny(unconditional_recursion)]
    #[link(name="sblt_plugin")]
    extern "C" {
        fn SuperBLT_Plugin_PushLua(L: *mut lua_State);
    }
    unsafe { SuperBLT_Plugin_PushLua(L) }
}

pub unsafe extern "C" fn say_hello(L: *mut lua_State) -> std::os::raw::c_int {
    let cancer = luaL_checkinteger(L, 1);

    //this complains but since we're compiling for 32-bit it is i32
    lua_pushinteger(L, (cancer << 2) as usize);

    1
}

pub extern "C" fn Plugin_Init() {}

pub extern "C" fn Plugin_Setup_Lua(_L: *mut lua_State) {}

pub extern "C" fn Plugin_Update() {}

pub extern "C" fn Plugin_PushLua(L: *mut lua_State) -> std::os::raw::c_int {
    //unsafe {
    //    lua_createtable(L, 0, 0);
//
    //    let hello_world_string = CString::new("Hellow, World!").unwrap();
    //    lua_pushstring(L, hello_world_string.as_ptr());
    //    let test = CString::new("mystring").unwrap();
    //    lua_setfield(L, -2, test.as_ptr());
//
    //    lua_pushcclosure(L, say_hello, 0);
    //    let myFuncName = CString::new("myfunction").unwrap();
    //    lua_setfield(L, -2, myFuncName.as_ptr());
    //}

    return 1;
}

//cannot replace these with c_* types as of now
#[no_mangle]
pub static MODULE_LICENCE_DECLARATION: &[u8] = b"This module is licenced under the GNU GPL version 2 or later, or another compatible licence\0";

#[no_mangle]
pub static MODULE_SOURCE_CODE_LOCATION: &[u8] = b"https://github.com/Siri-chan/Heisters-Haptics\0";

#[no_mangle]
pub static MODULE_SOURCE_CODE_REVISION: &[u8] = b"1\0";

#[no_mangle]
pub static SBLT_API_REVISION: u64 /*uint64_t*/ = 1; //this is unused still but don't change it
