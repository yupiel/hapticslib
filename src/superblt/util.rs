macro_rules! create_blt_callable {
    ($func_name:ident, $($param:tt: $ty:ty), *; $ret:ty) => {
        pub fn $func_name(&self, $($param:$ty), *) -> $ret {
            static ACTUAL_FUNC: std::sync::OnceLock<fn($($param:$ty), *) -> $ret> = std::sync::OnceLock::new();
            ACTUAL_FUNC.get_or_init(|| unsafe {std::mem::transmute_copy(
                self.get_function(stringify!($func_name))
            )})($($param), *)
        }
    };
    ($func_name:ident, $($param:tt: $ty:ty), *) => {
        pub fn $func_name(&self, $($param:$ty), *) -> () {
            static ACTUAL_FUNC: std::sync::OnceLock<fn($($param:$ty), *) -> ()> = std::sync::OnceLock::new();
            ACTUAL_FUNC.get_or_init(|| unsafe {std::mem::transmute_copy(
                self.get_function(stringify!($func_name))
            )})($($param), *)
        }
    }
}

pub(crate) use create_blt_callable;
