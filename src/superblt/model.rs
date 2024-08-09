use std::{
    collections::HashMap,
    ffi::c_void,
    ptr::null_mut,
    sync::{LazyLock, Mutex},
};

/// Global instance of SuperBLT which holds all associated internal and lua functions.
pub static SUPERBLT: LazyLock<Mutex<SuperBLT>> = LazyLock::new(|| Mutex::new(SuperBLT::default()));

/// Structure representing the environment injected by [SuperBLT](https://gitlab.com/znixian/payday2-superblt) into it's native Plugins.
/// 
/// Holds all of the [SuperBLT](https://gitlab.com/znixian/payday2-superblt)-internal and lua functions.
#[derive(Clone, Default)]
pub struct SuperBLT {
    function_list: HashMap<String, *mut c_void>,
}

unsafe impl Send for SuperBLT {}

impl SuperBLT {
    /// Saves a function pointer together with the function name as it's key.
    pub fn import_function(&mut self, function_name: &str, function_pointer: *mut c_void) {
        self.function_list
            .insert(function_name.to_owned(), function_pointer);
    }

    /// Retrieves the function pointer associated with the provided name.
    pub fn get_function(&self, function_name: &str) -> &*mut c_void {
        self.function_list.get(function_name).unwrap_or(&null_mut())
    }
}
