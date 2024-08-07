use std::{
    ffi::{c_char, c_int, CStr, CString},
    thread,
};

use buttplug::{
    client::{ButtplugClient, ButtplugClientError},
    core::{connector::new_json_ws_client_connector, errors::ButtplugError},
    util::{async_manager::block_on, future},
};
use tokio::runtime::Runtime;

use crate::{
    blt_lua::BLT_LUA_INSTANCE,
    lua_types::{lua_Integer, lua_State},
    pd2_logger::{PD2HOOK_LOG_ERROR, PD2HOOK_LOG_LOG},
};

pub unsafe extern "C-unwind" fn say_hello(L: *mut lua_State) -> c_int {
    let all_sigs = BLT_LUA_INSTANCE.lock().unwrap();

    let cancer: lua_Integer = all_sigs.luaL_checkinteger(L, 1);

    //this complains but since we're compiling for 32-bit it is i32
    all_sigs.lua_pushinteger(L, cancer_test(cancer.into()) << 2);

    1
}
fn cancer_test(idk: i32) -> i32 {
    idk + 1
}

pub unsafe extern "C-unwind" fn connect_haptics(L: *mut lua_State) -> c_int {
    let lua_instance = BLT_LUA_INSTANCE.lock().unwrap();

    let ip_addr: *const c_char = lua_instance.luaL_checkstring(L, 1);
    let ip_addr_cstring: String = unsafe { CStr::from_ptr(ip_addr).to_str().unwrap().into() };

    {
        thread::spawn(move || {
            let rt = Runtime::new().unwrap();

            rt.block_on(async {
                let connector = new_json_ws_client_connector(&format!("ws://{}", ip_addr_cstring));

                let client = ButtplugClient::new("PD2 Heister's Haptics");

                PD2HOOK_LOG_LOG!("Attempting connection to intiface.");
                if let Err(e) = client.connect(connector).await {
                    match e {
                        ButtplugClientError::ButtplugConnectorError(error) => {
                            PD2HOOK_LOG_ERROR!("Can't connect, exiting! Message: {}", error);
                        }
                        ButtplugClientError::ButtplugError(error) => match error {
                            ButtplugError::ButtplugHandshakeError(error) => {
                                PD2HOOK_LOG_ERROR!("Handshake issue, exiting! Message: {}", error);
                            }
                            error => {
                                PD2HOOK_LOG_ERROR!("Unexpected error type! {}", error);
                            }
                        },
                    }
                }
                PD2HOOK_LOG_LOG!("Connected!");

                
            });
        });
    }

    return 0;
}

#[allow(unused_variables)] //you can remove this if you actually plan to use the lua_State here
pub fn plugin_setup_lua(L: *mut lua_State) {}

pub fn plugin_init() {}

pub fn plugin_update() {}

pub fn plugin_push_lua(L: *mut lua_State) -> c_int {
    let all_sigs = BLT_LUA_INSTANCE.lock().unwrap();

    all_sigs.lua_newtable(L);

    let message = CString::new("Hellow, World!").unwrap();
    all_sigs.lua_pushstring(L, message.as_ptr());
    let test = CString::new("mystring").unwrap();
    all_sigs.lua_setfield(L, -2, test.as_ptr());

    all_sigs.lua_pushcclosure(L, say_hello, 0);
    let myFuncName = CString::new("myfunction").unwrap();
    all_sigs.lua_setfield(L, -2, myFuncName.as_ptr());

    all_sigs.lua_pushcclosure(L, connect_haptics, 0);
    let haptics_connect_name_cstring = CString::new("connectHaptics").unwrap();
    all_sigs.lua_setfield(L, -2, haptics_connect_name_cstring.as_ptr());

    return 1;
}
