#![allow(dead_code, non_snake_case)]
use std::ffi::{c_char, c_int, CStr, CString};

use crate::{
    globals::{LUA_TNUMBER, LUA_TSTRING},
    types::{c_size_t, lua_CFunction, lua_Integer, lua_Number, lua_State},
};

use super::model::SuperBLT;

impl SuperBLT {
    pub fn lua_newtable(&self, L: *mut lua_State) {
        self.lua_createtable(L, 0, 0);
    }

    pub fn lua_isnumber(&self, L: *mut lua_State, index: c_int) -> c_int {
        (self.lua_type(L, index) == LUA_TNUMBER) as c_int
    }

    pub fn luaL_typename(&self, L: *mut lua_State, index: c_int) -> *const c_char {
        self.lua_typename(L, self.lua_type(L, index))
    }

    pub fn luaL_argerror(&self, L: *mut lua_State, narg: c_int, extramsg: *const c_char) -> c_int {
        let extramsg_cstring = unsafe { CStr::from_ptr(extramsg) };
        let message_cstring = CString::new(format!(
            "bad argument {} {} in Rust plugin",
            narg,
            extramsg_cstring.to_str().unwrap()
        ))
        .unwrap();

        self.luaL_error(L, message_cstring.as_ptr())
    }

    pub fn luaL_typerror(&self, L: *mut lua_State, narg: c_int, tname: *const c_char) -> c_int {
        let actual_cstring = unsafe { CStr::from_ptr(tname) };
        let expected = self.luaL_typename(L, narg);
        let expected_cstring = unsafe { CStr::from_ptr(expected) };

        let message_cstring = CString::new(format!(
            "{} expected, got {}",
            actual_cstring.to_str().unwrap(),
            expected_cstring.to_str().unwrap()
        ))
        .unwrap();

        self.luaL_argerror(L, narg, message_cstring.as_ptr())
    }

    pub fn tag_error(&self, L: *mut lua_State, narg: c_int, tag: c_int) {
        self.luaL_typerror(L, narg, self.lua_typename(L, tag));
    }

    pub fn lua_isnoneornil(&self, L: *mut lua_State, index: c_int) -> bool {
        self.lua_type(L, index) <= 0
    }

    pub fn luaL_checklstring(
        &self,
        L: *mut lua_State,
        narg: c_int,
        len: *mut c_size_t,
    ) -> *const c_char {
        let s: *const c_char = self.lua_tolstring(L, narg, len);
        if s.is_null() {
            self.tag_error(L, narg, LUA_TSTRING)
        }

        s
    }

    pub fn luaL_optlstring(
        &self,
        L: *mut lua_State,
        narg: c_int,
        def: *const c_char,
        len: *mut c_size_t,
    ) -> *const c_char {
        if self.lua_isnoneornil(L, narg) {
            if !len.is_null() {
                let def_cstring = unsafe { CStr::from_ptr(def) };
                unsafe {
                    *len = if def.is_null() {
                        def_cstring.to_str().unwrap().len()
                    } else {
                        0
                    }
                }
            }

            def
        } else {
            self.luaL_checklstring(L, narg, len)
        }
    }

    pub fn luaL_optstring(
        &self,
        L: *mut lua_State,
        narg: c_int,
        def: *const c_char,
    ) -> *const c_char {
        self.luaL_optlstring(L, narg, def, std::ptr::null_mut())
    }

    pub fn luaL_checkstring(&self, L: *mut lua_State, narg: c_int) -> *const c_char {
        self.luaL_checklstring(L, narg, std::ptr::null_mut())
    }

    pub fn luaL_checkoption(
        &self,
        L: *mut lua_State,
        narg: c_int,
        def: *const c_char,
        lst: *const [*const c_char],
    ) -> c_int {
        let name: *const c_char = if def.is_null() {
            self.luaL_optstring(L, narg, def)
        } else {
            self.luaL_checkstring(L, narg)
        };

        //HACK: I HAVE NO IDEA IF THIS WORKS I NEED TO TEST THIS HELP
        for (i, val) in (unsafe { &(*lst) }).iter().enumerate() {
            if *val == name {
                return i as c_int;
            }
        }

        let name_cstring = unsafe { CStr::from_ptr(name) };
        //LUA_QS is nothing more than ' " %s " ' unfurled
        let error_cstring = CString::new(format!(
            "invalid option '\" {} \"' ",
            name_cstring.to_str().unwrap()
        ))
        .unwrap();

        self.luaL_argerror(L, narg, error_cstring.as_ptr())
    }

    pub fn luaL_checkinteger(&self, L: *mut lua_State, narg: c_int) -> lua_Integer {
        let d: lua_Integer = self.lua_tointeger(L, narg);
        if d == 0 && self.lua_isnumber(L, narg) == 0 {
            self.tag_error(L, narg, LUA_TNUMBER);
        }

        d
    }

    // this *sometimes* does not work for some reason all numbers sent through this are near 0
    pub fn luaL_checknumber(&self, L: *mut lua_State, narg: c_int) -> lua_Number {
        let d: lua_Number = self.lua_tonumber(L, narg);
        if d == (0 as lua_Number) && self.lua_isnumber(L, narg) == 0 {
            self.tag_error(L, narg, LUA_TNUMBER);
        }

        d
    }

    pub fn lua_pushcfunction(&self, L: *mut lua_State, func: lua_CFunction) {
        self.lua_pushcclosure(L, func, 0)
    }
}
