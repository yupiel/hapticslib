use std::ffi::CString;

use crate::{
    superblt::model::SuperBLT,
    types::{lua_CFunction, lua_State},
};

impl SuperBLT {
    pub fn luaY_pushcfunction(&self, L: *mut lua_State, func: lua_CFunction, func_name: &str) {
        self.lua_pushcfunction(L, func);
        let func_name_cstring = CString::new(func_name).unwrap();
        self.lua_setfield(L, -2, func_name_cstring.as_ptr())
    }
}
