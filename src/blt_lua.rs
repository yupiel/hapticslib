#![allow(non_camel_case_types, unused)]
use std::{
    collections::HashMap,
    ffi::{c_char, c_int, c_void, CStr, CString},
    sync::{LazyLock, Mutex, OnceLock},
};

use crate::lua_types::{
    lua_CFunction, lua_Integer, lua_Number, lua_State, LUA_TNUMBER, LUA_TSTRING,
};

//this is currently a nightly feature in rust core::ffi::c_size_t
//https://github.com/rust-lang/rust/issues/88345
//TODO: remove once implemented
pub type c_size_t = usize;

#[repr(C)]
pub struct luaL_Reg {
    pub name: *const c_char,
    pub func: lua_CFunction,
}

pub const IMPORTED_LUA_FUNCTION_NAMES: &[&str] = &[
    "is_active_state",
    "luaL_checkstack",
    "lua_rawequal",
    "lua_call",
    "lua_pcall",
    "lua_gettop",
    "lua_settop",
    "lua_toboolean",
    "lua_tointeger",
    "lua_tonumber",
    "lua_tolstring",
    "lua_objlen",
    "lua_touserdata",
    "luaL_loadfilex",
    "luaL_loadstring",
    "lua_getfield",
    "lua_setfield",
    "lua_createtable",
    "lua_newuserdata",
    "lua_insert",
    "lua_replace",
    "lua_remove",
    "lua_close",
    "lua_gettable",
    "lua_settable",
    "lua_setmetatable",
    "lua_getmetatable",
    "lua_pushnumber",
    "lua_pushinteger",
    "lua_pushboolean",
    "lua_pushcclosure",
    "lua_pushlstring",
    "lua_pushstring",
    "lua_pushfstring",
    "lua_checkstack",
    "lua_pushvalue",
    "lua_pushnil",
    "luaI_openlib",
    "luaL_ref",
    "lua_rawget",
    "lua_rawset",
    "lua_rawgeti",
    "lua_rawseti",
    "lua_type",
    "lua_typename",
    "luaL_unref",
    "lua_equal",
    "luaL_newmetatable",
    "luaL_checkudata",
    "luaL_error",
    "lua_error",
];

macro_rules! create_blt_callable {
    ($func_name:ident, $($param:tt: $ty:ty), *; $ret:ty) => {
        pub fn $func_name(&self, $($param:$ty), *) -> $ret {
            static ACTUAL_FUNC: OnceLock<fn($($param:$ty), *) -> $ret> = OnceLock::new();
            ACTUAL_FUNC.get_or_init(|| unsafe {std::mem::transmute_copy(
                self.function_list.get(stringify!($func_name)).unwrap()
            )})($($param), *)
        }
    };
    ($func_name:ident, $($param:tt: $ty:ty), *) => {
        pub fn $func_name(&self, $($param:$ty), *) -> () {
            static ACTUAL_FUNC: OnceLock<fn($($param:$ty), *) -> ()> = OnceLock::new();
            ACTUAL_FUNC.get_or_init(|| unsafe {std::mem::transmute_copy(
                self.function_list.get(stringify!($func_name)).unwrap()
            )})($($param), *)
        }
    }
}

pub static BLT_LUA_INSTANCE: LazyLock<Mutex<BltLua>> =
    LazyLock::new(|| Mutex::new(BltLua::default()));

#[derive(Clone, Default)]
pub struct BltLua {
    function_list: HashMap<String, *mut c_void>,
}

unsafe impl Send for BltLua {}

impl BltLua {
    pub fn add_function(&mut self, function_name: &str, function_pointer: *mut c_void) {
        self.function_list
            .insert(function_name.to_owned(), function_pointer);
    }

    /*
       Structure kept largely the same, comments inclued from SuperBLT native-plugin-library fptrs.h
       https://gitlab.com/SuperBLT/native-plugin-library/-/blob/master/include/sblt_msw32_impl/fptrs.h
    */

    //BLT Internal + checkstack
    create_blt_callable!(is_active_state, L: *mut lua_State; bool);
    create_blt_callable!(luaL_checkstack, L: *mut lua_State, sz: c_int, msg: *const c_char);
    create_blt_callable!(lua_rawequal, L: *mut lua_State, index1: c_int, index2: c_int; c_int);

    //Other Lua functions
    create_blt_callable!(lua_call, L: *mut lua_State, nargs: c_int, nresults: c_int);
    create_blt_callable!(lua_pcall, L: *mut lua_State, nargs: c_int, nresults: c_int, errfunc: c_int; c_int);
    create_blt_callable!(lua_gettop, L: *mut lua_State; c_int);
    create_blt_callable!(lua_settop, L: *mut lua_State, index: c_int);
    create_blt_callable!(lua_toboolean, L: *mut lua_State, index: c_int; c_int);
    create_blt_callable!(lua_tointeger, L: *mut lua_State, index: c_int; lua_Integer);
    create_blt_callable!(lua_tonumber, L: *mut lua_State, index: c_int; lua_Number);
    create_blt_callable!(lua_tolstring, L: *mut lua_State, index: c_int, len: *const c_size_t; *const c_char);
    create_blt_callable!(lua_objlen, L: *mut lua_State, index: c_int; c_size_t);
    create_blt_callable!(lua_touserdata, L: *mut lua_State, index: c_int; *mut c_void);
    // This is actually luaL_loadfilex() (as per Lua 5.2) now. The new parameter corresponds to mode, and specifying NULL causes Lua
    // to default to "bt", i.e. 'binary and text'
    // https://www.lua.org/manual/5.2/manual.html#luaL_loadfilex
    // https://www.lua.org/manual/5.2/manual.html#pdf-load
    create_blt_callable!(luaL_loadfilex, L: *mut lua_State, filename: *const c_char, mode: *const c_char; c_int);
    create_blt_callable!(luaL_loadstring,  L: *mut lua_State, s: *const c_char; c_int);
    //create_blt_callable!(lua_load, L: *mut lua_State, reader: lua_Reader, data: *mut c_void, chunkname: *const c_char; c_int);
    create_blt_callable!(lua_getfield, L: *mut lua_State, index: c_int, k: *const c_char);
    create_blt_callable!(lua_setfield,  L: *mut lua_State, index: c_int, k: *const c_char);
    create_blt_callable!(lua_createtable, L: *mut lua_State, narr: c_int, nrec: c_int);
    create_blt_callable!(lua_newuserdata, L: *mut lua_State, size_t: c_size_t; *mut c_void);
    create_blt_callable!(lua_insert, L: *mut lua_State, index: c_int);
    create_blt_callable!(lua_replace, L: *mut lua_State, index: c_int);
    create_blt_callable!(lua_remove, L: *mut lua_State, index: c_int);
    //create_blt_callable!(lua_newstate,  lua_Alloc, *; *mut lua_State); pretty sure this is internally disabled?
    create_blt_callable!(lua_close, L: *mut lua_State);

    // Reviving lua_settable() since the function exists again, and because the Crimefest 2015 alternative relied upon internal Lua
    // VM functions, which do not apply to LuaJIT
    create_blt_callable!(lua_gettable, L: *mut lua_State, index: c_int);
    create_blt_callable!(lua_settable,  L: *mut lua_State, index: c_int);
    create_blt_callable!(lua_setmetatable,  L: *mut lua_State, index: c_int; c_int);
    create_blt_callable!(lua_getmetatable,  L: *mut lua_State, index: c_int; c_int);
    create_blt_callable!(lua_pushnumber,  L: *mut lua_State, n: lua_Number);
    create_blt_callable!(lua_pushinteger,  L: *mut lua_State, n: lua_Integer);
    create_blt_callable!(lua_pushboolean,  L: *mut lua_State, b: c_int);
    create_blt_callable!(lua_pushcclosure,  L: *mut lua_State, func: lua_CFunction, n: c_int);
    // lua_pushstring()'s signature was found before lua_pushlstring()'s, so I'm leaving it here now since it's valid anyway
    // It was used as a quick and dirty - and broken - workaround since most lua_pushlstring() calls are inlined, but it ended up
    // breaking HTTP downloads of zip archives due to its sensitivity to premature null characters. A non-inlined signature for
    // lua_pushlstring() was found by cross-referencing the string 'loaders' to lj_cf_package_require(), which is part of LuaJIT
    create_blt_callable!(lua_pushlstring,  L: *mut lua_State, s: *const char, len: c_size_t);
    create_blt_callable!(lua_pushstring,  L: *mut lua_State, s: *const c_char);
    create_blt_callable!(lua_pushfstring,  L: *mut lua_State, fmt: *const c_char; *const c_char);
    create_blt_callable!(lua_checkstack,  L: *mut lua_State, extra: c_int; c_int);
    create_blt_callable!(lua_pushvalue,  L: *mut lua_State, index: c_int);
    create_blt_callable!(lua_pushnil,  L: *mut lua_State);

    // luaI_openlib() is really luaL_openlib(), see lauxlib.h in Lua 5.1's source code
    create_blt_callable!(luaI_openlib,  L: *mut lua_State, libname: *const c_char, l: *const luaL_Reg, nup: c_int);
    create_blt_callable!(luaL_ref,  L: *mut lua_State, t: c_int; c_int);

    // Reviving lua_rawgeti() since the function exists again, and because the Crimefest 2015 alternative relied upon internal Lua VM
    // functions, which do not apply to LuaJIT
    create_blt_callable!(lua_rawget,  L: *mut lua_State, index: c_int);
    create_blt_callable!(lua_rawset,  L: *mut lua_State, index: c_int);
    create_blt_callable!(lua_rawgeti,  L: *mut lua_State, index: c_int, n: c_int);
    create_blt_callable!(lua_rawseti,  L: *mut lua_State, index: c_int, n: c_int);
    create_blt_callable!(lua_type,  L: *mut lua_State, index: c_int; c_int);
    create_blt_callable!(lua_typename,  L: *mut lua_State, tp: c_int; *const c_char);
    create_blt_callable!(luaL_unref,  L: *mut lua_State, t: c_int, refrence: c_int);
    create_blt_callable!(lua_equal,  L: *mut lua_State, index1: c_int, index2: c_int; c_int);
    create_blt_callable!(luaL_newmetatable,  L: *mut lua_State, tname: *const c_char; c_int);
    create_blt_callable!(luaL_checkudata,  L: *mut lua_State, ud: c_int, tname: *const c_char; c_int);
    create_blt_callable!(luaL_error,  L: *mut lua_State, fmt: *const c_char; c_int);
    create_blt_callable!(lua_error,  L: *mut lua_State; c_int);

    //lauxlib defined shorthands
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
        let name: *const c_char = if def == std::ptr::null() {
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

    //this does not work for some reason all numbers sent through this are near 0
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

    pub fn luaY_pushcfunction(&self, L: *mut lua_State, func: lua_CFunction, func_name: &str) {
        self.lua_pushcfunction(L, func);
        let func_name_cstring = CString::new(func_name).unwrap();
        self.lua_setfield(L, -2, func_name_cstring.as_ptr())
    }
}
