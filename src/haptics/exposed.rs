use std::{ffi::c_int, sync::atomic::Ordering};

use crate::{
    superblt::SUPERBLT,
    types::{lua_Integer, lua_State},
};

use super::{
    channel::{HAPTICS_DEVICES, HAPTICS_IS_SCANNING},
    intiface::{
        haptics_create_connection, haptics_ping, haptics_scan_start, haptics_scan_stop,
        haptics_stop_all, haptics_vibrate,
    },
};

fn connection_died(L: *mut lua_State) {
    let superblt_instance = SUPERBLT.read().unwrap();
    superblt_instance.luaY_pushstring(L, "Haptics connection died. Please re-establish.".into());
}

pub extern "C-unwind" fn connect_haptics(L: *mut lua_State) -> c_int {
    let superblt_instance = SUPERBLT.read().unwrap();
    let websocket_uri = superblt_instance.luaY_getstringparam(L, 1);

    match websocket_uri {
        Some(actual_uri) => match haptics_create_connection(actual_uri) {
            Ok(msg) => superblt_instance.luaY_pushstring(L, msg),
            Err(_) => connection_died(L),
        },
        None => superblt_instance.luaY_pushstring(
            L,
            "Parameter to connectHaptics was malformed. Expected format <ip:port>".into(),
        ),
    }

    return 1;
}

pub extern "C-unwind" fn ping(L: *mut lua_State) -> c_int {
    let superblt_instance = SUPERBLT.read().unwrap();

    match haptics_ping() {
        Ok(msg) => superblt_instance.luaY_pushstring(L, msg),
        Err(_) => connection_died(L),
    }

    return 1;
}

pub extern "C-unwind" fn scan_start(L: *mut lua_State) -> c_int {
    let superblt_instance = SUPERBLT.read().unwrap();

    match haptics_scan_start() {
        Ok(msg) => superblt_instance.luaY_pushstring(L, msg),
        Err(_) => connection_died(L),
    }

    return 1;
}

pub extern "C-unwind" fn scan_stop(L: *mut lua_State) -> c_int {
    let superblt_instance = SUPERBLT.read().unwrap();

    match haptics_scan_stop() {
        Ok(msg) => superblt_instance.luaY_pushstring(L, msg),
        Err(_) => connection_died(L),
    }

    return 1;
}

pub extern "C-unwind" fn list_devices(L: *mut lua_State) -> c_int {
    let superblt_instance = SUPERBLT.read().unwrap();

    if HAPTICS_IS_SCANNING.load(Ordering::SeqCst) {
        superblt_instance.luaY_pushstring(
            L,
            "Cannot list devices. Scan is currently in progress.".into(),
        );

        return 1;
    }

    match HAPTICS_DEVICES.lock() {
        Ok(device_list) => {
            superblt_instance.luaY_vectoarraytable(L, &device_list);
        }
        Err(err) => {
            superblt_instance.luaY_pushstring(
                L,
                format!(
                    "Devices Mutex was poisoned. This shouldn't happen. Message: {}",
                    err
                ),
            );
        }
    }

    return 1;
}

pub extern "C-unwind" fn stop_all(L: *mut lua_State) -> c_int {
    let superblt_instance = SUPERBLT.read().unwrap();

    match haptics_stop_all() {
        Ok(msg) => superblt_instance.luaY_pushstring(L, msg),
        Err(_) => connection_died(L),
    }

    return 1;
}

pub extern "C-unwind" fn vibrate(L: *mut lua_State) -> c_int {
    let superblt_instance = SUPERBLT.read().unwrap();

    // This is 0 if the param is not an integer, so I will ignore error cases
    let lua_param: lua_Integer = superblt_instance.luaL_checkinteger(L, 1);

    match haptics_vibrate(lua_param) {
        Ok(msg) => superblt_instance.luaY_pushstring(L, msg),
        Err(_) => connection_died(L),
    }

    return 1;
}
