#![allow(non_snake_case)]
use std::{
    ffi::{c_char, c_int, CStr, CString},
    sync::{mpsc::Sender, OnceLock},
    thread,
};

use buttplug::{
    client::{ButtplugClient, ButtplugClientError},
    core::{connector::new_json_ws_client_connector, errors::ButtplugError},
};
use tokio::runtime::Runtime;

use crate::{
    superblt::pd2_logger::{PD2HOOK_LOG_ERROR, PD2HOOK_LOG_LOG},
    types::{lua_Integer, lua_State},
    SUPERBLT,
};

pub unsafe extern "C-unwind" fn say_hello(L: *mut lua_State) -> c_int {
    let all_sigs = SUPERBLT.lock().unwrap();

    let cancer: lua_Integer = all_sigs.luaL_checkinteger(L, 1);

    all_sigs.lua_pushinteger(L, cancer_test(cancer.into()) << 2);

    1
}
fn cancer_test(idk: i32) -> i32 {
    idk + 1
}

pub static HAPTICS_SENDER: OnceLock<Sender<f64>> = OnceLock::new();

pub unsafe extern "C-unwind" fn connect_haptics(L: *mut lua_State) -> c_int {
    let lua_instance = SUPERBLT.lock().unwrap();

    let ip_addr: *const c_char = lua_instance.luaL_checkstring(L, 1);
    let ip_addr_cstring: String = unsafe { CStr::from_ptr(ip_addr).to_str().unwrap().into() };

    let (tx, rx) = std::sync::mpsc::channel::<f64>();
    HAPTICS_SENDER.set(tx).unwrap();

    {
        thread::spawn(move || {
            let rt = Runtime::new().unwrap();

            rt.block_on(async move {
                let connector = new_json_ws_client_connector(&format!("ws://{}", ip_addr_cstring));

                let client = ButtplugClient::new("PD2 Heister's Haptics");

                PD2HOOK_LOG_LOG!("Attempting connection to intiface.");
                if let Err(e) = client.connect(connector).await {
                    match e {
                        ButtplugClientError::ButtplugConnectorError(error) => {
                            PD2HOOK_LOG_ERROR!("Can't connect to Intiface server. Dropping connection. Message: {}", error);
                        }
                        ButtplugClientError::ButtplugError(error) => match error {
                            ButtplugError::ButtplugHandshakeError(error) => {
                                PD2HOOK_LOG_ERROR!("Handshake issue with Intiface server. Dropping connection. Message: {}", error);
                            }
                            error => {
                                PD2HOOK_LOG_ERROR!("Unexpected error when trying to connect to Intiface! {}", error);
                            }
                        },
                    }
                }
                PD2HOOK_LOG_LOG!("Connected to intiface! Scanning for devices...");

                client.start_scanning().await.unwrap();
                client.stop_scanning().await.unwrap();

                PD2HOOK_LOG_LOG!("Finished scanning. Listing devices.");
                for device in client.devices().iter() {
                    PD2HOOK_LOG_LOG!("Device {} [{}] found.", device.display_name().clone().map_or("Unknown".into(), |name| name), device.name());
                }

                loop {
                    match rx.recv() {
                        Ok(strength) => {
                            for device in client.devices() {
                                device.vibrate(&buttplug::client::ScalarValueCommand::ScalarValue(strength)).await.unwrap();
                            }
                        },
                        Err(_) => {
                            PD2HOOK_LOG_ERROR!("Sender died.");
                            break;
                        }
                    }
                }
            });
        });
    }

    return 0;
}

pub extern "C-unwind" fn set_haptic_strength(L: *mut lua_State) -> c_int {
    let lua_instance = SUPERBLT.lock().unwrap();

    let lua_param: lua_Integer = lua_instance.luaL_checkinteger(L, 1);
    match HAPTICS_SENDER
        .get()
        .unwrap()
        .send((lua_param as f64) / 100_f64)
    {
        Ok(_) => {
            //TODO: I really need to write something to make these Cstrings maybe even the full response
            let response_cstring =
                CString::new(format!("Set haptics strength to: {}%", lua_param.to_string())).unwrap();
            lua_instance.lua_pushstring(L, response_cstring.as_ptr());
        }
        Err(_) => {
            let response_cstring =
                CString::new("Haptics connection died. Please re-establish.").unwrap();
            lua_instance.lua_pushstring(L, response_cstring.as_ptr());
        }
    }

    return 1;
}

#[allow(unused_variables)] //you can remove this if you actually plan to use the lua_State here
pub fn plugin_setup_lua(L: *mut lua_State) {}

pub fn plugin_init() {}

pub fn plugin_update() {}

pub fn plugin_push_lua(L: *mut lua_State) -> c_int {
    let all_sigs = SUPERBLT.lock().unwrap();

    all_sigs.lua_newtable(L);

    let message = CString::new("Hellow, World!").unwrap();
    all_sigs.lua_pushstring(L, message.as_ptr());
    let test = CString::new("mystring").unwrap();
    all_sigs.lua_setfield(L, -2, test.as_ptr());

    all_sigs.luaY_pushcfunction(L, say_hello, "myfunction");

    all_sigs.luaY_pushcfunction(L, connect_haptics, "connectHaptics");

    all_sigs.luaY_pushcfunction(L, set_haptic_strength, "setStrength");

    return 1;
}
