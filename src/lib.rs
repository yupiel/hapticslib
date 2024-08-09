#![allow(non_snake_case)]

#[cfg(not(any(
    all(
        target_arch = "x86",
        target_os = "windows",
        any(target_env = "msvc", target_env = "gnu")
    ),
    i_know_what_im_doing
)))]
compile_error!("This library does not currently support this target. If you know what you are doing, for example, if you are porting the library, use the `i_know_what_im_doing` config flag.");

use std::{
    ffi::{c_int, CString},
    panic,
};

use superblt::{
    pd2_logger::{PD2HOOK_LOG, PD2HOOK_LOG_PANIC},
    SUPERBLT, SUPERBLT_EXPORTED_FUNCTIONS,
};
use types::{lua_State, lua_access_func};

mod blt_funcs;
mod globals;
mod superblt;
mod types;

#[no_mangle]
pub extern "C" fn SuperBLT_Plugin_Setup(get_exposed_function: lua_access_func) {
    //We take out the logging function separately to spread our macros throughout the project
    let pd2_log_func_cstring = CString::new("pd2_log").unwrap();
    PD2HOOK_LOG.get_or_init(|| unsafe {
        std::mem::transmute_copy(&get_exposed_function(pd2_log_func_cstring.as_ptr()))
    });

    //all panics will now produce error logs in mods/logs
    panic::set_hook(Box::new(|panic_info| {
        PD2HOOK_LOG_PANIC!("{}", panic_info);
    }));

    //this imports everything declared with IMPORT_FUNC or
    //CREATE_NORMAL_CALLABLE_SIGNATURE in blt's native plugin library
    //https://gitlab.com/SuperBLT/native-plugin-library/-/blob/master/include/sblt_msw32_impl/fptrs.h
    let mut blt_lua_instance = SUPERBLT.lock().unwrap();
    for func_name in SUPERBLT_EXPORTED_FUNCTIONS.into_iter() {
        let curr_func_name = CString::new(func_name.to_owned()).unwrap();
        blt_lua_instance.add_function(*func_name, get_exposed_function(curr_func_name.as_ptr()));
    }

    blt_funcs::plugin_init();
}
#[no_mangle]
pub extern "C" fn SuperBLT_Plugin_Init_State(L: *mut lua_State) {
    blt_funcs::plugin_setup_lua(L);
}
#[no_mangle]
pub extern "C" fn SuperBLT_Plugin_Update() {
    blt_funcs::plugin_update();
}
#[no_mangle]
pub extern "C" fn SuperBLT_Plugin_PushLua(L: *mut lua_State) -> c_int {
    blt_funcs::plugin_push_lua(L)
}

//cannot replace these with c_* types as of now
#[no_mangle]
pub static MODULE_LICENCE_DECLARATION: &[u8] = b"This module is licenced under the GNU GPL version 2 or later, or another compatible licence\0";

#[no_mangle]
pub static MODULE_SOURCE_CODE_LOCATION: &[u8] = b"https://github.com/Siri-chan/Heisters-Haptics\0";

#[no_mangle]
pub static MODULE_SOURCE_CODE_REVISION: &[u8] = b"1\0";

#[no_mangle]
pub static SBLT_API_REVISION: u64 /*uint64_t*/ = 1; //this is unused still but don't change it
