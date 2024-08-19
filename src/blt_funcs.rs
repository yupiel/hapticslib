#![allow(non_snake_case)]
use std::ffi::c_int;

use crate::{
    haptics::exposed::{connect_haptics, ping, scan_start, scan_stop, stop_all, vibrate},
    superblt::SUPERBLT,
    types::lua_State,
};

#[allow(unused_variables)] // You can remove this if you actually plan to use the lua_State here
pub fn plugin_setup_lua(L: *mut lua_State) {}

pub fn plugin_init() {}

pub fn plugin_update() {}

pub fn plugin_push_lua(L: *mut lua_State) -> c_int {
    let superblt_instance = SUPERBLT.read().unwrap();

    superblt_instance.lua_newtable(L);

    superblt_instance.luaY_pushcfunction(L, connect_haptics, "connectHaptics");
    superblt_instance.luaY_pushcfunction(L, ping, "ping");
    superblt_instance.luaY_pushcfunction(L, scan_start, "scanStart");
    superblt_instance.luaY_pushcfunction(L, scan_stop, "scanStop");
    superblt_instance.luaY_pushcfunction(L, stop_all, "stopAll");
    superblt_instance.luaY_pushcfunction(L, vibrate, "vibrate");

    return 1;
}
