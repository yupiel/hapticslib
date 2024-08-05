#![allow(non_camel_case_types, non_snake_case)]

use std::ffi::{c_char, c_int, CString};

use blt_lua::{lua_State, pd2hook_log_log, ALL_LUA_FUNCS, BLT_LUA_INSTANCE};

mod blt_funcs;
mod blt_lua;

const LUA_IDSIZE: usize = 60;

struct lua_Debug {
    pub event: c_int,
    pub name: *const c_char,
    pub namewhat: *const c_char,
    pub what: *const c_char,
    pub source: *const c_char,
    pub currentline: c_int,
    pub nups: c_int,
    pub linedefined: c_int,
    pub lastlinedefined: c_int,
    pub short_src: [char; LUA_IDSIZE],
    i_ci: c_int,
}

type lua_access_func = extern "C" fn(*const std::ffi::c_char) -> *mut std::ffi::c_void;

#[no_mangle]
pub extern "C" fn SuperBLT_Plugin_Setup(get_exposed_function: lua_access_func) {
    let mut blt_lua_instance = BLT_LUA_INSTANCE.lock().unwrap();

    //this can import everything declared with IMPORT_FUNC or
    //CREATE_NORMAL_CALLABLE_SIGNATURE in blt's native plugin library
    //https://gitlab.com/SuperBLT/native-plugin-library/-/blob/master/include/sblt_msw32_impl/fptrs.h
    for func_name in ALL_LUA_FUNCS.into_iter() {
        let curr_func_name = CString::new(func_name.to_owned()).unwrap();
        blt_lua_instance.add_function(*func_name, get_exposed_function(curr_func_name.as_ptr()));
    }

    pd2hook_log_log!(&blt_lua_instance, "hehe we even get logs now yayy");

    blt_funcs::plugin_init();
}
#[no_mangle]
pub extern "C" fn SuperBLT_Plugin_Init_State(L: *mut lua_State) {
    blt_funcs::plugin_setup_lua(L);
}
#[no_mangle]
pub extern "C" fn SuperBLT_Plugin_Update() {
    blt_funcs::plugin_update();
}
#[no_mangle]
pub extern "C" fn SuperBLT_Plugin_PushLua(L: *mut lua_State) -> c_int {
    blt_funcs::plugin_push_lua(L)
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
