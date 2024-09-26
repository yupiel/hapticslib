#![allow(dead_code, non_snake_case)]
use std::ffi::{c_char, c_int, c_void};

use crate::types::c_size_t;
use crate::types::{luaL_Reg, lua_CFunction, lua_Integer, lua_Number, lua_State};

use super::{model::SuperBLT, util::create_blt_callable};

impl SuperBLT {
    /*
     * Structure kept largely the same, comments inclued from SuperBLT native-plugin-library fptrs.h
     * https://gitlab.com/SuperBLT/native-plugin-library/-/blob/master/include/sblt_msw32_impl/fptrs.h
     */

    // BLT Internal + checkstack
    create_blt_callable!(is_active_state, L: *mut lua_State; bool);
    create_blt_callable!(luaL_checkstack, L: *mut lua_State, sz: c_int, msg: *const c_char);
    create_blt_callable!(lua_rawequal, L: *mut lua_State, index1: c_int, index2: c_int; c_int);

    // Other Lua functions
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
    // create_blt_callable!(lua_load, L: *mut lua_State, reader: lua_Reader, data: *mut c_void, chunkname: *const c_char; c_int);
    create_blt_callable!(lua_getfield, L: *mut lua_State, index: c_int, k: *const c_char);
    create_blt_callable!(lua_setfield,  L: *mut lua_State, index: c_int, k: *const c_char);
    create_blt_callable!(lua_createtable, L: *mut lua_State, narr: c_int, nrec: c_int);
    create_blt_callable!(lua_newuserdata, L: *mut lua_State, size_t: c_size_t; *mut c_void);
    create_blt_callable!(lua_insert, L: *mut lua_State, index: c_int);
    create_blt_callable!(lua_replace, L: *mut lua_State, index: c_int);
    create_blt_callable!(lua_remove, L: *mut lua_State, index: c_int);
    // create_blt_callable!(lua_newstate,  lua_Alloc, *; *mut lua_State); pretty sure this is internally disabled?
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
}
