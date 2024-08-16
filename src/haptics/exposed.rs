use std::ffi::c_int;

use crate::{
    superblt::SUPERBLT,
    types::{lua_Integer, lua_State},
};

use super::intiface::{
    haptics_create_connection, haptics_ping, haptics_re_scan, haptics_set_strength,
    haptics_stop_all,
};

fn connection_died(L: *mut lua_State) {
    let superblt_instance = SUPERBLT.read().unwrap();
    superblt_instance
        .luaY_stringreturnvalue(L, "Haptics connection died. Please re-establish.".into());
}

pub extern "C-unwind" fn connect_haptics(L: *mut lua_State) -> c_int {
    let superblt_instance = SUPERBLT.read().unwrap();
    let websocket_uri = superblt_instance.luaY_getstringparam(L, 1);

    match websocket_uri {
        Some(actual_uri) => match haptics_create_connection(actual_uri) {
            Ok(msg) => superblt_instance.luaY_stringreturnvalue(L, msg),
            Err(_) => connection_died(L),
        },
        None => superblt_instance.luaY_stringreturnvalue(
            L,
            "Parameter to connectHaptics was malformed. Expected format <ip:port>".into(),
        ),
    }

    return 1;
}

pub extern "C-unwind" fn ping(L: *mut lua_State) -> c_int {
    let superblt_instance = SUPERBLT.read().unwrap();

    match haptics_ping() {
        Ok(msg) => superblt_instance.luaY_stringreturnvalue(L, msg),
        Err(_) => connection_died(L),
    }

    return 1;
}

pub extern "C-unwind" fn re_scan(L: *mut lua_State) -> c_int {
    let superblt_instance = SUPERBLT.read().unwrap();

    match haptics_re_scan() {
        Ok(msg) => superblt_instance.luaY_stringreturnvalue(L, msg),
        Err(_) => connection_died(L),
    }

    return 1;
}

pub extern "C-unwind" fn set_strength(L: *mut lua_State) -> c_int {
    let superblt_instance = SUPERBLT.read().unwrap();

    // This is 0 if the param is not an integer, so I will ignore error cases
    let lua_param: lua_Integer = superblt_instance.luaL_checkinteger(L, 1);

    match haptics_set_strength(lua_param) {
        Ok(msg) => superblt_instance.luaY_stringreturnvalue(L, msg),
        Err(_) => connection_died(L),
    }

    return 1;
}

pub extern "C-unwind" fn stop_all(L: *mut lua_State) -> c_int {
    let superblt_instance = SUPERBLT.read().unwrap();

    match haptics_stop_all() {
        Ok(msg) => superblt_instance.luaY_stringreturnvalue(L, msg),
        Err(_) => connection_died(L),
    }

    return 1;
}
