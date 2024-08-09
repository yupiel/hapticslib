use std::{
    collections::HashMap,
    ffi::c_void,
    ptr::null_mut,
    sync::{LazyLock, Mutex},
};

pub static SUPERBLT: LazyLock<Mutex<BltLua>> =
    LazyLock::new(|| Mutex::new(BltLua::default()));

#[derive(Clone, Default)]
pub struct BltLua {
    pub(crate) function_list: HashMap<String, *mut c_void>,
}

unsafe impl Send for BltLua {}

impl BltLua {
    pub fn add_function(&mut self, function_name: &str, function_pointer: *mut c_void) {
        self.function_list
            .insert(function_name.to_owned(), function_pointer);
    }

    pub fn get_function(&self, function_name: &str) -> &*mut c_void {
        self.function_list.get(function_name).unwrap_or(&null_mut())
    }
}
