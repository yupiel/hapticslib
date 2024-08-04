#![allow(non_camel_case_types, non_snake_case)]

use std::{
    ffi::{c_char, c_int, c_void, CStr, CString},
    sync::Mutex,
};

use lazy_static::lazy_static;
mod blt_funcs;

pub type lua_State = *mut *mut c_void;
pub type lua_Integer = i32;
pub type lua_CFunction = unsafe extern "C-unwind" fn(L: *mut lua_State) -> c_int;

const ALL_LUA_FUNCS: &[&str] = &[
    "pd2_log",
    "lua_tointeger",
    "lua_isnumber",
    "lua_pushinteger",
    "lua_createtable",
    "lua_pushstring",
    "lua_setfield",
    "lua_pushcclosure",
];

const LUA_IDSIZE: usize = 60;

struct lua_Debug {
    pub event: c_int,
    pub name: *const c_char,
    pub namewhat: *const c_char,
    pub what: *const c_char,
    pub source: *const c_char,
    pub currentline: c_int,
    pub nups: c_int,
    pub linedefined: c_int,
    pub lastlinedefined: c_int,
    pub short_src: [char; LUA_IDSIZE],
    i_ci: c_int,
}

pub struct FuncSig {
    pub name: String,
    pub address: *mut c_void,
}

unsafe impl Send for FuncSig {}

pub struct AllFuncSigs {
    pub sigs: Vec<FuncSig>,
}

impl AllFuncSigs {
    pub fn new() -> Self {
        Self {
            sigs: Default::default(),
        }
    }

    pub fn add_sig(&mut self, sig: FuncSig) {
        self.sigs.push(sig);
    }

    pub fn lua_tointeger(&self, L: *mut lua_State, index: c_int) -> lua_Integer {
        let actual_func: fn(L: *mut lua_State, index: c_int) -> lua_Integer = unsafe {
            std::mem::transmute_copy(
                &self
                    .sigs
                    .iter()
                    .find(|x| x.name == "lua_tointeger")
                    .unwrap()
                    .address,
            )
        };

        actual_func(L, index)
    }

    pub fn lua_isnumber(&self, L: *mut lua_State, index: c_int) -> c_int {
        let actual_func: fn(L: *mut lua_State, index: c_int) -> c_int = unsafe {
            std::mem::transmute_copy(
                &self
                    .sigs
                    .iter()
                    .find(|x| x.name == "lua_isnumber")
                    .unwrap()
                    .address,
            )
        };

        actual_func(L, index)
    }

    pub fn luaL_checkinteger(&self, L: *mut lua_State, narg: c_int) -> lua_Integer {
        let d: lua_Integer = self.lua_tointeger(L, narg);
        if d == 0 && self.lua_isnumber(L, narg) == 0 {
            //sadly tag_error is not importable via blt so not quite sure what to do here
            //tag_error(L, narg, LUA_TNUMBER);
        }
        return d;
    }

    //this is more or less equivalent to tag_error

    pub fn lua_pushfstring(
        &self,
        L: *mut lua_State,
        fmt: *const c_char, /*... variable params */
    ) -> *const c_char {
        let actual_func: fn(
            L: *mut lua_State,
            fmt: *const c_char, /*... variable params */
        ) -> *const c_char = unsafe {
            std::mem::transmute_copy(
                &self
                    .sigs
                    .iter()
                    .find(|x| x.name == "lua_pushfstring")
                    .unwrap()
                    .address,
            )
        };

        actual_func(L, fmt)
    }

    pub fn lua_typename(&self, L: *mut lua_State, tp: c_int) -> *const c_char {
        let actual_func: fn(L: *mut lua_State, tp: c_int) -> *const c_char = unsafe {
            std::mem::transmute_copy(
                &self
                    .sigs
                    .iter()
                    .find(|x| x.name == "lua_typename")
                    .unwrap()
                    .address,
            )
        };

        actual_func(L, tp)
    }

    pub fn lua_type(&self, L: *mut lua_State, index: c_int) -> c_int {
        let actual_func: fn(L: *mut lua_State, index: c_int) -> c_int = unsafe {
            std::mem::transmute_copy(
                &self
                    .sigs
                    .iter()
                    .find(|x| x.name == "lua_type")
                    .unwrap()
                    .address,
            )
        };

        actual_func(L, index)
    }

    pub fn luaL_typename(&self, L: *mut lua_State, index: c_int) -> *const c_char {
        self.lua_typename(L, self.lua_type(L, index))
    }

    pub fn luaL_argerror(&self, L: *mut lua_State, narg: c_int, extramsg: *const c_char) {
        let mut ar: lua_Debug;

        //if !lua_getstack(L, 0, &ar) getstack isn't exported by blt either and as far as i can tell has no equivalent impl
    }

    pub fn luaL_typerror(&self, L: *mut lua_State, narg: c_int, tname: *const c_char) /* -> c_int */
    {
        let expected = unsafe { CStr::from_ptr(tname).to_str().unwrap() };
        let actual = unsafe {
            CStr::from_ptr(self.luaL_typename(L, narg))
                .to_str()
                .unwrap()
        };
        let message_cstring =
            CString::new(format!("{} expected, got {}", expected, actual)).unwrap();

        let msg: *const c_char = self.lua_pushfstring(L, message_cstring.as_ptr());

        //return luaL_argerror(L, narg, msg);
    }

    pub fn lua_pushinteger(&self, L: *mut lua_State, n: lua_Integer) -> () {
        let actual_func: fn(L: *mut lua_State, n: lua_Integer) -> () = unsafe {
            std::mem::transmute_copy(
                &self
                    .sigs
                    .iter()
                    .find(|x| x.name == "lua_pushinteger")
                    .unwrap()
                    .address,
            )
        };

        actual_func(L, n)
    }

    pub fn lua_newtable(&self, L: *mut lua_State) -> () {
        let actual_func: fn(L: *mut lua_State, narr: c_int, nrec: c_int) = unsafe {
            std::mem::transmute_copy(
                &self
                    .sigs
                    .iter()
                    .find(|x| x.name == "lua_createtable")
                    .unwrap()
                    .address,
            )
        };

        actual_func(L, 0, 0)
    }

    pub fn lua_pushstring(&self, L: *mut lua_State, s: *const c_char) -> *const c_char {
        let actual_func: fn(L: *mut lua_State, s: *const c_char) -> *const c_char = unsafe {
            std::mem::transmute_copy(
                &self
                    .sigs
                    .iter()
                    .find(|x| x.name == "lua_pushstring")
                    .unwrap()
                    .address,
            )
        };

        actual_func(L, s)
    }

    pub fn lua_setfield(&self, L: *mut lua_State, idx: c_int, k: *const c_char) -> () {
        let actual_func: fn(L: *mut lua_State, idx: c_int, k: *const c_char) -> () = unsafe {
            std::mem::transmute_copy(
                &self
                    .sigs
                    .iter()
                    .find(|x| x.name == "lua_setfield")
                    .unwrap()
                    .address,
            )
        };

        actual_func(L, idx, k)
    }

    pub fn lua_pushcclosure(&self, L: *mut lua_State, f: lua_CFunction, n: c_int) -> () {
        let actual_func: fn(L: *mut lua_State, f: lua_CFunction, n: c_int) -> () = unsafe {
            std::mem::transmute_copy(
                &self
                    .sigs
                    .iter()
                    .find(|x| x.name == "lua_pushcclosure")
                    .unwrap()
                    .address,
            )
        };

        actual_func(L, f, n)
    }

    pub fn PD2HOOK_LOG_LOG(&self, message: &str) {
        let message_cstr = CString::new(message).unwrap();

        let actual_func: fn(
            msg: *const c_char,
            level: c_int,
            file: *const c_char,
            line: c_int,
        ) -> () = unsafe {
            std::mem::transmute_copy(
                &self
                    .sigs
                    .iter()
                    .find(|x| x.name == "pd2_log")
                    .unwrap()
                    .address,
            )
        };

        let file_name = CString::new("lib.rs").unwrap();
        actual_func(message_cstr.as_ptr(), 1, file_name.as_ptr(), 147);
    }
}

lazy_static! {
    pub static ref all_func_sigs: Mutex<AllFuncSigs> = Mutex::new(AllFuncSigs::new());
}

type lua_access_func = extern "C" fn(*const std::ffi::c_char) -> *mut std::ffi::c_void;

#[no_mangle]
pub extern "C" fn SuperBLT_Plugin_Setup(get_exposed_function: lua_access_func) {
    let mut all_sigs = all_func_sigs.lock().unwrap();

    //this can import everything declared with IMPORT_FUNC or
    //CREATE_NORMAL_CALLABLE_SIGNATURE in blt's native plugin library
    //https://gitlab.com/SuperBLT/native-plugin-library/-/blob/master/include/sblt_msw32_impl/fptrs.h
    for func_name in ALL_LUA_FUNCS.into_iter() {
        let curr_func_name = CString::new(func_name.to_owned()).unwrap();

        let do_i_get_anything_here = get_exposed_function(curr_func_name.as_ptr());

        all_sigs.add_sig(FuncSig {
            name: func_name.to_owned().to_owned(),
            address: do_i_get_anything_here,
        });
    }

    all_sigs.PD2HOOK_LOG_LOG("hehe we even get logs now yayy");

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
