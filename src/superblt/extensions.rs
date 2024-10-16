use std::ffi::{c_int, CStr, CString};

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

    pub fn luaY_getstringparam(&self, L: *mut lua_State, position: c_int) -> Option<String> {
        let retrived_param = self.luaL_checkstring(L, position);

        if retrived_param.is_null() {
            return None;
        }

        let param_cstring = unsafe { CStr::from_ptr(retrived_param) };

        match param_cstring.to_str() {
            Ok(param_string) => Some(param_string.into()),
            Err(_) => None,
        }
    }

    pub fn luaY_pushstring(&self, L: *mut lua_State, value: String) {
        let return_value = CString::new(value).unwrap();
        self.lua_pushstring(L, return_value.as_ptr());
    }

    pub fn luaY_setfield(&self, L: *mut lua_State, index: c_int, key: String) {
        let key_cstring = CString::new(key).unwrap();
        self.lua_setfield(L, index, key_cstring.as_ptr());
    }

    pub fn luaY_vectoarraytable(&self, L: *mut lua_State, string_list: &[String]) {
        self.lua_newtable(L);

        for (index, string_entry) in string_list.iter().enumerate() {
            self.luaY_pushstring(L, string_entry.into());
            self.luaY_setfield(L, -2, (index + 1).to_string());
        }
    }
}
