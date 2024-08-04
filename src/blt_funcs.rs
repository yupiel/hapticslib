use std::ffi::{c_int, CString};

use crate::{all_func_sigs, lua_Integer, lua_State};

pub unsafe extern "C-unwind" fn say_hello(L: *mut lua_State) -> c_int {
    let all_sigs = all_func_sigs.lock().unwrap();

    let cancer: lua_Integer = all_sigs.luaL_checkinteger(L, 1);

    //this complains but since we're compiling for 32-bit it is i32
    all_sigs.lua_pushinteger(L, cancer_test(cancer.into()) << 2);

    1
}
fn cancer_test(idk: i32) -> i32 {
    idk + 1
}

pub fn plugin_setup_lua(L: *mut lua_State) {}

pub fn plugin_init() {}

pub fn plugin_update() {}

pub fn plugin_push_lua(L: *mut lua_State) -> c_int {
    let all_sigs = all_func_sigs.lock().unwrap();

    all_sigs.lua_newtable(L);

    let message = CString::new("Hellow, World!").unwrap();
    all_sigs.lua_pushstring(L, message.as_ptr());
    let test = CString::new("mystring").unwrap();
    all_sigs.lua_setfield(L, -2, test.as_ptr());
    
    all_sigs.lua_pushcclosure(L, say_hello, 0);

    let myFuncName = CString::new("myfunction").unwrap();
    all_sigs.lua_setfield(L, -2, myFuncName.as_ptr());

    return 1;
}
