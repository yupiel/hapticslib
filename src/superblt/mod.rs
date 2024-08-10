pub mod extensions;
pub mod laux;
pub mod lua;
pub mod model;
pub mod pd2_logger;

pub(self) mod util;
pub use model::SUPERBLT;

pub const SUPERBLT_EXPORTED_FUNCTIONS: &[&str] = &[
    // superblt special handling
    "is_active_state",
    "luaL_checkstack",
    "lua_rawequal",

    // direct lua export
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