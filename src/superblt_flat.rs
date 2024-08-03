#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, unused)]

pub const _VCRT_COMPILER_PREPROCESSOR: u32 = 1;
pub const _SAL_VERSION: u32 = 20;
pub const __SAL_H_VERSION: u32 = 180000000;
pub const _USE_DECLSPECS_FOR_SAL: u32 = 0;
pub const _USE_ATTRIBUTES_FOR_SAL: u32 = 0;
pub const _CRT_PACKING: u32 = 8;
pub const _HAS_EXCEPTIONS: u32 = 1;
pub const _STL_LANG: u32 = 0;
pub const _HAS_CXX17: u32 = 0;
pub const _HAS_CXX20: u32 = 0;
pub const _HAS_CXX23: u32 = 0;
pub const _HAS_NODISCARD: u32 = 0;
pub const _ARM_WINAPI_PARTITION_DESKTOP_SDK_AVAILABLE: u32 = 1;
pub const _CRT_BUILD_DESKTOP_APP: u32 = 1;
pub const _ARGMAX: u32 = 100;
pub const _CRT_INT_MAX: u32 = 2147483647;
pub const _CRT_FUNCTIONS_REQUIRED: u32 = 1;
pub const _CRT_HAS_CXX17: u32 = 0;
pub const _CRT_HAS_C11: u32 = 1;
pub const _CRT_INTERNAL_NONSTDC_NAMES: u32 = 1;
pub const __STDC_SECURE_LIB__: u32 = 200411;
pub const __GOT_SECURE_LIB__: u32 = 200411;
pub const __STDC_WANT_SECURE_LIB__: u32 = 1;
pub const _SECURECRT_FILL_BUFFER_PATTERN: u32 = 254;
pub const _CRT_SECURE_CPP_OVERLOAD_STANDARD_NAMES: u32 = 0;
pub const _CRT_SECURE_CPP_OVERLOAD_STANDARD_NAMES_COUNT: u32 = 0;
pub const _CRT_SECURE_CPP_OVERLOAD_SECURE_NAMES: u32 = 1;
pub const _CRT_SECURE_CPP_OVERLOAD_STANDARD_NAMES_MEMORY: u32 = 0;
pub const _CRT_SECURE_CPP_OVERLOAD_SECURE_NAMES_MEMORY: u32 = 0;
pub const WINVER: u32 = 1281;
pub const CHAR_BIT: u32 = 8;
pub const SCHAR_MIN: i32 = -128;
pub const SCHAR_MAX: u32 = 127;
pub const UCHAR_MAX: u32 = 255;
pub const CHAR_MIN: i32 = -128;
pub const CHAR_MAX: u32 = 127;
pub const MB_LEN_MAX: u32 = 5;
pub const SHRT_MIN: i32 = -32768;
pub const SHRT_MAX: u32 = 32767;
pub const USHRT_MAX: u32 = 65535;
pub const INT_MIN: i32 = -2147483648;
pub const INT_MAX: u32 = 2147483647;
pub const UINT_MAX: u32 = 4294967295;
pub const LONG_MIN: i32 = -2147483648;
pub const LONG_MAX: u32 = 2147483647;
pub const ULONG_MAX: u32 = 4294967295;
pub const LUA_LDIR: &[u8; 7] = b"!\\lua\\\0";
pub const LUA_CDIR: &[u8; 3] = b"!\\\0";
pub const LUA_PATH_DEFAULT: &[u8; 38] = b".\\?.lua;!\\lua\\?.lua;!\\lua\\?\\init.lua;\0";
pub const LUA_CPATH_DEFAULT: &[u8; 30] = b".\\?.dll;!\\?.dll;!\\loadall.dll\0";
pub const LUA_PATH: &[u8; 9] = b"LUA_PATH\0";
pub const LUA_CPATH: &[u8; 10] = b"LUA_CPATH\0";
pub const LUA_INIT: &[u8; 9] = b"LUA_INIT\0";
pub const LUA_DIRSEP: &[u8; 2] = b"\\\0";
pub const LUA_PATHSEP: &[u8; 2] = b";\0";
pub const LUA_PATH_MARK: &[u8; 2] = b"?\0";
pub const LUA_EXECDIR: &[u8; 2] = b"!\0";
pub const LUA_IGMARK: &[u8; 2] = b"-\0";
pub const LUA_PATH_CONFIG: &[u8; 10] = b"\\\n;\n?\n!\n-\0";
pub const LUAI_MAXSTACK: u32 = 65500;
pub const LUAI_MAXCSTACK: u32 = 8000;
pub const LUAI_GCPAUSE: u32 = 200;
pub const LUAI_GCMUL: u32 = 200;
pub const LUA_MAXCAPTURES: u32 = 32;
pub const LUA_IDSIZE: u32 = 60;
pub const LUA_NUMBER_SCAN: &[u8; 4] = b"%lf\0";
pub const LUA_NUMBER_FMT: &[u8; 6] = b"%.14g\0";
pub const LUAI_MAXNUMBER2STR: u32 = 32;
pub const LUA_INTFRMLEN: &[u8; 2] = b"l\0";
pub const LUA_VERSION: &[u8; 8] = b"Lua 5.1\0";
pub const LUA_RELEASE: &[u8; 10] = b"Lua 5.1.4\0";
pub const LUA_VERSION_NUM: u32 = 501;
pub const LUA_COPYRIGHT: &[u8; 41] = b"Copyright (C) 1994-2008 Lua.org, PUC-Rio\0";
pub const LUA_AUTHORS: &[u8; 49] = b"R. Ierusalimschy, L. H. de Figueiredo & W. Celes\0";
pub const LUA_MULTRET: i32 = -1;
pub const LUA_REGISTRYINDEX: i32 = -10000;
pub const LUA_ENVIRONINDEX: i32 = -10001;
pub const LUA_GLOBALSINDEX: i32 = -10002;
pub const LUA_YIELD: u32 = 1;
pub const LUA_ERRRUN: u32 = 2;
pub const LUA_ERRSYNTAX: u32 = 3;
pub const LUA_ERRMEM: u32 = 4;
pub const LUA_ERRERR: u32 = 5;
pub const LUA_TNONE: i32 = -1;
pub const LUA_TNIL: u32 = 0;
pub const LUA_TBOOLEAN: u32 = 1;
pub const LUA_TLIGHTUSERDATA: u32 = 2;
pub const LUA_TNUMBER: u32 = 3;
pub const LUA_TSTRING: u32 = 4;
pub const LUA_TTABLE: u32 = 5;
pub const LUA_TFUNCTION: u32 = 6;
pub const LUA_TUSERDATA: u32 = 7;
pub const LUA_TTHREAD: u32 = 8;
pub const LUA_MINSTACK: u32 = 20;
pub const LUA_GCSTOP: u32 = 0;
pub const LUA_GCRESTART: u32 = 1;
pub const LUA_GCCOLLECT: u32 = 2;
pub const LUA_GCCOUNT: u32 = 3;
pub const LUA_GCCOUNTB: u32 = 4;
pub const LUA_GCSTEP: u32 = 5;
pub const LUA_GCSETPAUSE: u32 = 6;
pub const LUA_GCSETSTEPMUL: u32 = 7;
pub const LUA_HOOKCALL: u32 = 0;
pub const LUA_HOOKRET: u32 = 1;
pub const LUA_HOOKLINE: u32 = 2;
pub const LUA_HOOKCOUNT: u32 = 3;
pub const LUA_HOOKTAILRET: u32 = 4;
pub const LUA_MASKCALL: u32 = 1;
pub const LUA_MASKRET: u32 = 2;
pub const LUA_MASKLINE: u32 = 4;
pub const LUA_MASKCOUNT: u32 = 8;
pub const _CRT_INTERNAL_STDIO_SYMBOL_PREFIX: &[u8; 1] = b"\0";
pub const _CRT_INTERNAL_PRINTF_LEGACY_VSPRINTF_NULL_TERMINATION: u32 = 1;
pub const _CRT_INTERNAL_PRINTF_STANDARD_SNPRINTF_BEHAVIOR: u32 = 2;
pub const _CRT_INTERNAL_PRINTF_LEGACY_WIDE_SPECIFIERS: u32 = 4;
pub const _CRT_INTERNAL_PRINTF_LEGACY_MSVCRT_COMPATIBILITY: u32 = 8;
pub const _CRT_INTERNAL_PRINTF_LEGACY_THREE_DIGIT_EXPONENTS: u32 = 16;
pub const _CRT_INTERNAL_PRINTF_STANDARD_ROUNDING: u32 = 32;
pub const _CRT_INTERNAL_SCANF_SECURECRT: u32 = 1;
pub const _CRT_INTERNAL_SCANF_LEGACY_WIDE_SPECIFIERS: u32 = 2;
pub const _CRT_INTERNAL_SCANF_LEGACY_MSVCRT_COMPATIBILITY: u32 = 4;
pub const BUFSIZ: u32 = 512;
pub const _NSTREAM_: u32 = 512;
pub const _IOB_ENTRIES: u32 = 3;
pub const EOF: i32 = -1;
pub const _IOFBF: u32 = 0;
pub const _IOLBF: u32 = 64;
pub const _IONBF: u32 = 4;
pub const L_tmpnam: u32 = 260;
pub const L_tmpnam_s: u32 = 260;
pub const SEEK_CUR: u32 = 1;
pub const SEEK_END: u32 = 2;
pub const SEEK_SET: u32 = 0;
pub const FILENAME_MAX: u32 = 260;
pub const FOPEN_MAX: u32 = 20;
pub const _SYS_OPEN: u32 = 20;
pub const TMP_MAX: u32 = 2147483647;
pub const TMP_MAX_S: u32 = 2147483647;
pub const _TMP_MAX_S: u32 = 2147483647;
pub const SYS_OPEN: u32 = 20;
pub const __bool_true_false_are_defined: u32 = 1;
pub const false_: u32 = 0;
pub const true_: u32 = 1;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct lua_State {
    _unused: [u8; 0],
}
pub type va_list = *mut ::std::os::raw::c_char;
extern "C" {
    pub fn __va_start(arg1: *mut *mut ::std::os::raw::c_char, ...);
}
pub type __vcrt_bool = bool;
pub type wchar_t = ::std::os::raw::c_ushort;
extern "C" {
    pub fn __security_init_cookie();
}
extern "C" {
    pub fn __security_check_cookie(_StackCookie: usize);
}
extern "C" {
    pub fn __report_gsfailure(_StackCookie: usize) -> !;
}
extern "C" {
    pub static mut __security_cookie: usize;
}
pub type __crt_bool = bool;
extern "C" {
    pub fn _invalid_parameter_noinfo();
}
extern "C" {
    pub fn _invalid_parameter_noinfo_noreturn() -> !;
}
extern "C" {
    pub fn _invoke_watson(
        _Expression: *const wchar_t,
        _FunctionName: *const wchar_t,
        _FileName: *const wchar_t,
        _LineNo: ::std::os::raw::c_uint,
        _Reserved: usize,
    ) -> !;
}
pub type errno_t = ::std::os::raw::c_int;
pub type wint_t = ::std::os::raw::c_ushort;
pub type wctype_t = ::std::os::raw::c_ushort;
pub type __time32_t = ::std::os::raw::c_long;
pub type __time64_t = ::std::os::raw::c_longlong;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __crt_locale_data_public {
    pub _locale_pctype: *const ::std::os::raw::c_ushort,
    pub _locale_mb_cur_max: ::std::os::raw::c_int,
    pub _locale_lc_codepage: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout___crt_locale_data_public() {
    const UNINIT: ::std::mem::MaybeUninit<__crt_locale_data_public> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<__crt_locale_data_public>(),
        16usize,
        concat!("Size of: ", stringify!(__crt_locale_data_public))
    );
    assert_eq!(
        ::std::mem::align_of::<__crt_locale_data_public>(),
        8usize,
        concat!("Alignment of ", stringify!(__crt_locale_data_public))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._locale_pctype) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__crt_locale_data_public),
            "::",
            stringify!(_locale_pctype)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._locale_mb_cur_max) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__crt_locale_data_public),
            "::",
            stringify!(_locale_mb_cur_max)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._locale_lc_codepage) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(__crt_locale_data_public),
            "::",
            stringify!(_locale_lc_codepage)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __crt_locale_pointers {
    pub locinfo: *mut __crt_locale_data,
    pub mbcinfo: *mut __crt_multibyte_data,
}
#[test]
fn bindgen_test_layout___crt_locale_pointers() {
    const UNINIT: ::std::mem::MaybeUninit<__crt_locale_pointers> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<__crt_locale_pointers>(),
        16usize,
        concat!("Size of: ", stringify!(__crt_locale_pointers))
    );
    assert_eq!(
        ::std::mem::align_of::<__crt_locale_pointers>(),
        8usize,
        concat!("Alignment of ", stringify!(__crt_locale_pointers))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).locinfo) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__crt_locale_pointers),
            "::",
            stringify!(locinfo)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).mbcinfo) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__crt_locale_pointers),
            "::",
            stringify!(mbcinfo)
        )
    );
}
pub type _locale_t = *mut __crt_locale_pointers;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _Mbstatet {
    pub _Wchar: ::std::os::raw::c_ulong,
    pub _Byte: ::std::os::raw::c_ushort,
    pub _State: ::std::os::raw::c_ushort,
}
#[test]
fn bindgen_test_layout__Mbstatet() {
    const UNINIT: ::std::mem::MaybeUninit<_Mbstatet> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_Mbstatet>(),
        8usize,
        concat!("Size of: ", stringify!(_Mbstatet))
    );
    assert_eq!(
        ::std::mem::align_of::<_Mbstatet>(),
        4usize,
        concat!("Alignment of ", stringify!(_Mbstatet))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._Wchar) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_Mbstatet),
            "::",
            stringify!(_Wchar)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._Byte) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(_Mbstatet),
            "::",
            stringify!(_Byte)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._State) as usize - ptr as usize },
        6usize,
        concat!(
            "Offset of field: ",
            stringify!(_Mbstatet),
            "::",
            stringify!(_State)
        )
    );
}
pub type mbstate_t = _Mbstatet;
pub type time_t = __time64_t;
pub type rsize_t = usize;
extern "C" {
    pub fn _errno() -> *mut ::std::os::raw::c_int;
}
extern "C" {
    pub fn _set_errno(_Value: ::std::os::raw::c_int) -> errno_t;
}
extern "C" {
    pub fn _get_errno(_Value: *mut ::std::os::raw::c_int) -> errno_t;
}
extern "C" {
    pub fn __threadid() -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn __threadhandle() -> usize;
}
pub type lua_CFunction = unsafe extern "C" fn(L: *mut lua_State) -> ::std::os::raw::c_int;
pub type lua_Reader = ::std::option::Option<
    unsafe extern "C" fn(
        L: *mut lua_State,
        ud: *mut ::std::os::raw::c_void,
        sz: *mut usize,
    ) -> *const ::std::os::raw::c_char,
>;
pub type lua_Writer = ::std::option::Option<
    unsafe extern "C" fn(
        L: *mut lua_State,
        p: *const ::std::os::raw::c_void,
        sz: usize,
        ud: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int,
>;
pub type lua_Alloc = ::std::option::Option<
    unsafe extern "C" fn(
        ud: *mut ::std::os::raw::c_void,
        ptr: *mut ::std::os::raw::c_void,
        osize: usize,
        nsize: usize,
    ) -> *mut ::std::os::raw::c_void,
>;
pub type lua_Number = f64;
pub type lua_Integer = isize;
pub type lua_Hook =
    ::std::option::Option<unsafe extern "C" fn(L: *mut lua_State, ar: *mut lua_Debug)>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct lua_Debug {
    pub event: ::std::os::raw::c_int,
    pub name: *const ::std::os::raw::c_char,
    pub namewhat: *const ::std::os::raw::c_char,
    pub what: *const ::std::os::raw::c_char,
    pub source: *const ::std::os::raw::c_char,
    pub currentline: ::std::os::raw::c_int,
    pub nups: ::std::os::raw::c_int,
    pub linedefined: ::std::os::raw::c_int,
    pub lastlinedefined: ::std::os::raw::c_int,
    pub short_src: [::std::os::raw::c_char; 60usize],
    pub i_ci: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_lua_Debug() {
    const UNINIT: ::std::mem::MaybeUninit<lua_Debug> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<lua_Debug>(),
        120usize,
        concat!("Size of: ", stringify!(lua_Debug))
    );
    assert_eq!(
        ::std::mem::align_of::<lua_Debug>(),
        8usize,
        concat!("Alignment of ", stringify!(lua_Debug))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).event) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(lua_Debug),
            "::",
            stringify!(event)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).name) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(lua_Debug),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).namewhat) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(lua_Debug),
            "::",
            stringify!(namewhat)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).what) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(lua_Debug),
            "::",
            stringify!(what)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).source) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(lua_Debug),
            "::",
            stringify!(source)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).currentline) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(lua_Debug),
            "::",
            stringify!(currentline)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).nups) as usize - ptr as usize },
        44usize,
        concat!(
            "Offset of field: ",
            stringify!(lua_Debug),
            "::",
            stringify!(nups)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).linedefined) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(lua_Debug),
            "::",
            stringify!(linedefined)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).lastlinedefined) as usize - ptr as usize },
        52usize,
        concat!(
            "Offset of field: ",
            stringify!(lua_Debug),
            "::",
            stringify!(lastlinedefined)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).short_src) as usize - ptr as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(lua_Debug),
            "::",
            stringify!(short_src)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).i_ci) as usize - ptr as usize },
        116usize,
        concat!(
            "Offset of field: ",
            stringify!(lua_Debug),
            "::",
            stringify!(i_ci)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _iobuf {
    pub _Placeholder: *mut ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout__iobuf() {
    const UNINIT: ::std::mem::MaybeUninit<_iobuf> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_iobuf>(),
        8usize,
        concat!("Size of: ", stringify!(_iobuf))
    );
    assert_eq!(
        ::std::mem::align_of::<_iobuf>(),
        8usize,
        concat!("Alignment of ", stringify!(_iobuf))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._Placeholder) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_iobuf),
            "::",
            stringify!(_Placeholder)
        )
    );
}
pub type FILE = _iobuf;
extern "C" {
    pub fn __acrt_iob_func(_Ix: ::std::os::raw::c_uint) -> *mut FILE;
}
extern "C" {
    pub fn fgetwc(_Stream: *mut FILE) -> wint_t;
}
extern "C" {
    pub fn _fgetwchar() -> wint_t;
}
extern "C" {
    pub fn fputwc(_Character: wchar_t, _Stream: *mut FILE) -> wint_t;
}
extern "C" {
    pub fn _fputwchar(_Character: wchar_t) -> wint_t;
}
extern "C" {
    pub fn getwc(_Stream: *mut FILE) -> wint_t;
}
extern "C" {
    pub fn getwchar() -> wint_t;
}
extern "C" {
    pub fn fgetws(
        _Buffer: *mut wchar_t,
        _BufferCount: ::std::os::raw::c_int,
        _Stream: *mut FILE,
    ) -> *mut wchar_t;
}
extern "C" {
    pub fn fputws(_Buffer: *const wchar_t, _Stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _getws_s(_Buffer: *mut wchar_t, _BufferCount: usize) -> *mut wchar_t;
}
extern "C" {
    pub fn putwc(_Character: wchar_t, _Stream: *mut FILE) -> wint_t;
}
extern "C" {
    pub fn putwchar(_Character: wchar_t) -> wint_t;
}
extern "C" {
    pub fn _putws(_Buffer: *const wchar_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ungetwc(_Character: wint_t, _Stream: *mut FILE) -> wint_t;
}
extern "C" {
    pub fn _wfdopen(_FileHandle: ::std::os::raw::c_int, _Mode: *const wchar_t) -> *mut FILE;
}
extern "C" {
    pub fn _wfopen(_FileName: *const wchar_t, _Mode: *const wchar_t) -> *mut FILE;
}
extern "C" {
    pub fn _wfopen_s(
        _Stream: *mut *mut FILE,
        _FileName: *const wchar_t,
        _Mode: *const wchar_t,
    ) -> errno_t;
}
extern "C" {
    pub fn _wfreopen(
        _FileName: *const wchar_t,
        _Mode: *const wchar_t,
        _OldStream: *mut FILE,
    ) -> *mut FILE;
}
extern "C" {
    pub fn _wfreopen_s(
        _Stream: *mut *mut FILE,
        _FileName: *const wchar_t,
        _Mode: *const wchar_t,
        _OldStream: *mut FILE,
    ) -> errno_t;
}
extern "C" {
    pub fn _wfsopen(
        _FileName: *const wchar_t,
        _Mode: *const wchar_t,
        _ShFlag: ::std::os::raw::c_int,
    ) -> *mut FILE;
}
extern "C" {
    pub fn _wperror(_ErrorMessage: *const wchar_t);
}
extern "C" {
    pub fn _wpopen(_Command: *const wchar_t, _Mode: *const wchar_t) -> *mut FILE;
}
extern "C" {
    pub fn _wremove(_FileName: *const wchar_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _wtempnam(_Directory: *const wchar_t, _FilePrefix: *const wchar_t) -> *mut wchar_t;
}
extern "C" {
    pub fn _wtmpnam_s(_Buffer: *mut wchar_t, _BufferCount: usize) -> errno_t;
}
extern "C" {
    pub fn _wtmpnam(_Buffer: *mut wchar_t) -> *mut wchar_t;
}
extern "C" {
    pub fn _fgetwc_nolock(_Stream: *mut FILE) -> wint_t;
}
extern "C" {
    pub fn _fputwc_nolock(_Character: wchar_t, _Stream: *mut FILE) -> wint_t;
}
extern "C" {
    pub fn _getwc_nolock(_Stream: *mut FILE) -> wint_t;
}
extern "C" {
    pub fn _putwc_nolock(_Character: wchar_t, _Stream: *mut FILE) -> wint_t;
}
extern "C" {
    pub fn _ungetwc_nolock(_Character: wint_t, _Stream: *mut FILE) -> wint_t;
}
extern "C" {
    pub fn __stdio_common_vfwprintf(
        _Options: ::std::os::raw::c_ulonglong,
        _Stream: *mut FILE,
        _Format: *const wchar_t,
        _Locale: _locale_t,
        _ArgList: va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __stdio_common_vfwprintf_s(
        _Options: ::std::os::raw::c_ulonglong,
        _Stream: *mut FILE,
        _Format: *const wchar_t,
        _Locale: _locale_t,
        _ArgList: va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __stdio_common_vfwprintf_p(
        _Options: ::std::os::raw::c_ulonglong,
        _Stream: *mut FILE,
        _Format: *const wchar_t,
        _Locale: _locale_t,
        _ArgList: va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __stdio_common_vfwscanf(
        _Options: ::std::os::raw::c_ulonglong,
        _Stream: *mut FILE,
        _Format: *const wchar_t,
        _Locale: _locale_t,
        _ArgList: va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __stdio_common_vswprintf(
        _Options: ::std::os::raw::c_ulonglong,
        _Buffer: *mut wchar_t,
        _BufferCount: usize,
        _Format: *const wchar_t,
        _Locale: _locale_t,
        _ArgList: va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __stdio_common_vswprintf_s(
        _Options: ::std::os::raw::c_ulonglong,
        _Buffer: *mut wchar_t,
        _BufferCount: usize,
        _Format: *const wchar_t,
        _Locale: _locale_t,
        _ArgList: va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __stdio_common_vsnwprintf_s(
        _Options: ::std::os::raw::c_ulonglong,
        _Buffer: *mut wchar_t,
        _BufferCount: usize,
        _MaxCount: usize,
        _Format: *const wchar_t,
        _Locale: _locale_t,
        _ArgList: va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __stdio_common_vswprintf_p(
        _Options: ::std::os::raw::c_ulonglong,
        _Buffer: *mut wchar_t,
        _BufferCount: usize,
        _Format: *const wchar_t,
        _Locale: _locale_t,
        _ArgList: va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __stdio_common_vswscanf(
        _Options: ::std::os::raw::c_ulonglong,
        _Buffer: *const wchar_t,
        _BufferCount: usize,
        _Format: *const wchar_t,
        _Locale: _locale_t,
        _ArgList: va_list,
    ) -> ::std::os::raw::c_int;
}
pub type fpos_t = ::std::os::raw::c_longlong;
extern "C" {
    pub fn _get_stream_buffer_pointers(
        _Stream: *mut FILE,
        _Base: *mut *mut *mut ::std::os::raw::c_char,
        _Pointer: *mut *mut *mut ::std::os::raw::c_char,
        _Count: *mut *mut ::std::os::raw::c_int,
    ) -> errno_t;
}
extern "C" {
    pub fn clearerr_s(_Stream: *mut FILE) -> errno_t;
}
extern "C" {
    pub fn fopen_s(
        _Stream: *mut *mut FILE,
        _FileName: *const ::std::os::raw::c_char,
        _Mode: *const ::std::os::raw::c_char,
    ) -> errno_t;
}
extern "C" {
    pub fn fread_s(
        _Buffer: *mut ::std::os::raw::c_void,
        _BufferSize: usize,
        _ElementSize: usize,
        _ElementCount: usize,
        _Stream: *mut FILE,
    ) -> usize;
}
extern "C" {
    pub fn freopen_s(
        _Stream: *mut *mut FILE,
        _FileName: *const ::std::os::raw::c_char,
        _Mode: *const ::std::os::raw::c_char,
        _OldStream: *mut FILE,
    ) -> errno_t;
}
extern "C" {
    pub fn gets_s(
        _Buffer: *mut ::std::os::raw::c_char,
        _Size: rsize_t,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn tmpfile_s(_Stream: *mut *mut FILE) -> errno_t;
}
extern "C" {
    pub fn tmpnam_s(_Buffer: *mut ::std::os::raw::c_char, _Size: rsize_t) -> errno_t;
}
extern "C" {
    pub fn clearerr(_Stream: *mut FILE);
}
extern "C" {
    pub fn fclose(_Stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _fcloseall() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _fdopen(
        _FileHandle: ::std::os::raw::c_int,
        _Mode: *const ::std::os::raw::c_char,
    ) -> *mut FILE;
}
extern "C" {
    pub fn feof(_Stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ferror(_Stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fflush(_Stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fgetc(_Stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _fgetchar() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fgetpos(_Stream: *mut FILE, _Position: *mut fpos_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fgets(
        _Buffer: *mut ::std::os::raw::c_char,
        _MaxCount: ::std::os::raw::c_int,
        _Stream: *mut FILE,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn _fileno(_Stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _flushall() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fopen(
        _FileName: *const ::std::os::raw::c_char,
        _Mode: *const ::std::os::raw::c_char,
    ) -> *mut FILE;
}
extern "C" {
    pub fn fputc(_Character: ::std::os::raw::c_int, _Stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _fputchar(_Character: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fputs(
        _Buffer: *const ::std::os::raw::c_char,
        _Stream: *mut FILE,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fread(
        _Buffer: *mut ::std::os::raw::c_void,
        _ElementSize: ::std::os::raw::c_ulonglong,
        _ElementCount: ::std::os::raw::c_ulonglong,
        _Stream: *mut FILE,
    ) -> ::std::os::raw::c_ulonglong;
}
extern "C" {
    pub fn freopen(
        _FileName: *const ::std::os::raw::c_char,
        _Mode: *const ::std::os::raw::c_char,
        _Stream: *mut FILE,
    ) -> *mut FILE;
}
extern "C" {
    pub fn _fsopen(
        _FileName: *const ::std::os::raw::c_char,
        _Mode: *const ::std::os::raw::c_char,
        _ShFlag: ::std::os::raw::c_int,
    ) -> *mut FILE;
}
extern "C" {
    pub fn fsetpos(_Stream: *mut FILE, _Position: *const fpos_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fseek(
        _Stream: *mut FILE,
        _Offset: ::std::os::raw::c_long,
        _Origin: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _fseeki64(
        _Stream: *mut FILE,
        _Offset: ::std::os::raw::c_longlong,
        _Origin: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ftell(_Stream: *mut FILE) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn _ftelli64(_Stream: *mut FILE) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn fwrite(
        _Buffer: *const ::std::os::raw::c_void,
        _ElementSize: ::std::os::raw::c_ulonglong,
        _ElementCount: ::std::os::raw::c_ulonglong,
        _Stream: *mut FILE,
    ) -> ::std::os::raw::c_ulonglong;
}
extern "C" {
    pub fn getc(_Stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getchar() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _getmaxstdio() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _getw(_Stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn perror(_ErrorMessage: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn _pclose(_Stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _popen(
        _Command: *const ::std::os::raw::c_char,
        _Mode: *const ::std::os::raw::c_char,
    ) -> *mut FILE;
}
extern "C" {
    pub fn putc(_Character: ::std::os::raw::c_int, _Stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn putchar(_Character: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn puts(_Buffer: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _putw(_Word: ::std::os::raw::c_int, _Stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn remove(_FileName: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rename(
        _OldFileName: *const ::std::os::raw::c_char,
        _NewFileName: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _unlink(_FileName: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn unlink(_FileName: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rewind(_Stream: *mut FILE);
}
extern "C" {
    pub fn _rmtmp() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setbuf(_Stream: *mut FILE, _Buffer: *mut ::std::os::raw::c_char);
}
extern "C" {
    pub fn _setmaxstdio(_Maximum: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setvbuf(
        _Stream: *mut FILE,
        _Buffer: *mut ::std::os::raw::c_char,
        _Mode: ::std::os::raw::c_int,
        _Size: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _tempnam(
        _DirectoryName: *const ::std::os::raw::c_char,
        _FilePrefix: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn tmpfile() -> *mut FILE;
}
extern "C" {
    pub fn tmpnam(_Buffer: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn ungetc(_Character: ::std::os::raw::c_int, _Stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _lock_file(_Stream: *mut FILE);
}
extern "C" {
    pub fn _unlock_file(_Stream: *mut FILE);
}
extern "C" {
    pub fn _fclose_nolock(_Stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _fflush_nolock(_Stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _fgetc_nolock(_Stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _fputc_nolock(
        _Character: ::std::os::raw::c_int,
        _Stream: *mut FILE,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _fread_nolock(
        _Buffer: *mut ::std::os::raw::c_void,
        _ElementSize: usize,
        _ElementCount: usize,
        _Stream: *mut FILE,
    ) -> usize;
}
extern "C" {
    pub fn _fread_nolock_s(
        _Buffer: *mut ::std::os::raw::c_void,
        _BufferSize: usize,
        _ElementSize: usize,
        _ElementCount: usize,
        _Stream: *mut FILE,
    ) -> usize;
}
extern "C" {
    pub fn _fseek_nolock(
        _Stream: *mut FILE,
        _Offset: ::std::os::raw::c_long,
        _Origin: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _fseeki64_nolock(
        _Stream: *mut FILE,
        _Offset: ::std::os::raw::c_longlong,
        _Origin: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _ftell_nolock(_Stream: *mut FILE) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn _ftelli64_nolock(_Stream: *mut FILE) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn _fwrite_nolock(
        _Buffer: *const ::std::os::raw::c_void,
        _ElementSize: usize,
        _ElementCount: usize,
        _Stream: *mut FILE,
    ) -> usize;
}
extern "C" {
    pub fn _getc_nolock(_Stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _putc_nolock(
        _Character: ::std::os::raw::c_int,
        _Stream: *mut FILE,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _ungetc_nolock(
        _Character: ::std::os::raw::c_int,
        _Stream: *mut FILE,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __p__commode() -> *mut ::std::os::raw::c_int;
}
extern "C" {
    pub fn __stdio_common_vfprintf(
        _Options: ::std::os::raw::c_ulonglong,
        _Stream: *mut FILE,
        _Format: *const ::std::os::raw::c_char,
        _Locale: _locale_t,
        _ArgList: va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __stdio_common_vfprintf_s(
        _Options: ::std::os::raw::c_ulonglong,
        _Stream: *mut FILE,
        _Format: *const ::std::os::raw::c_char,
        _Locale: _locale_t,
        _ArgList: va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __stdio_common_vfprintf_p(
        _Options: ::std::os::raw::c_ulonglong,
        _Stream: *mut FILE,
        _Format: *const ::std::os::raw::c_char,
        _Locale: _locale_t,
        _ArgList: va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _set_printf_count_output(_Value: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _get_printf_count_output() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __stdio_common_vfscanf(
        _Options: ::std::os::raw::c_ulonglong,
        _Stream: *mut FILE,
        _Format: *const ::std::os::raw::c_char,
        _Locale: _locale_t,
        _Arglist: va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __stdio_common_vsprintf(
        _Options: ::std::os::raw::c_ulonglong,
        _Buffer: *mut ::std::os::raw::c_char,
        _BufferCount: usize,
        _Format: *const ::std::os::raw::c_char,
        _Locale: _locale_t,
        _ArgList: va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __stdio_common_vsprintf_s(
        _Options: ::std::os::raw::c_ulonglong,
        _Buffer: *mut ::std::os::raw::c_char,
        _BufferCount: usize,
        _Format: *const ::std::os::raw::c_char,
        _Locale: _locale_t,
        _ArgList: va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __stdio_common_vsnprintf_s(
        _Options: ::std::os::raw::c_ulonglong,
        _Buffer: *mut ::std::os::raw::c_char,
        _BufferCount: usize,
        _MaxCount: usize,
        _Format: *const ::std::os::raw::c_char,
        _Locale: _locale_t,
        _ArgList: va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __stdio_common_vsprintf_p(
        _Options: ::std::os::raw::c_ulonglong,
        _Buffer: *mut ::std::os::raw::c_char,
        _BufferCount: usize,
        _Format: *const ::std::os::raw::c_char,
        _Locale: _locale_t,
        _ArgList: va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __stdio_common_vsscanf(
        _Options: ::std::os::raw::c_ulonglong,
        _Buffer: *const ::std::os::raw::c_char,
        _BufferCount: usize,
        _Format: *const ::std::os::raw::c_char,
        _Locale: _locale_t,
        _ArgList: va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tempnam(
        _Directory: *const ::std::os::raw::c_char,
        _FilePrefix: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn fcloseall() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fdopen(
        _FileHandle: ::std::os::raw::c_int,
        _Format: *const ::std::os::raw::c_char,
    ) -> *mut FILE;
}
extern "C" {
    pub fn fgetchar() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fileno(_Stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn flushall() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fputchar(_Ch: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getw(_Stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn putw(_Ch: ::std::os::raw::c_int, _Stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rmtmp() -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct luaL_Reg {
    pub name: *const ::std::os::raw::c_char,
    pub func: lua_CFunction,
}
#[test]
fn bindgen_test_layout_luaL_Reg() {
    const UNINIT: ::std::mem::MaybeUninit<luaL_Reg> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<luaL_Reg>(),
        16usize,
        concat!("Size of: ", stringify!(luaL_Reg))
    );
    assert_eq!(
        ::std::mem::align_of::<luaL_Reg>(),
        8usize,
        concat!("Alignment of ", stringify!(luaL_Reg))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).name) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(luaL_Reg),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).func) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(luaL_Reg),
            "::",
            stringify!(func)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct luaL_Buffer {
    pub p: *mut ::std::os::raw::c_char,
    pub lvl: ::std::os::raw::c_int,
    pub L: *mut lua_State,
    pub buffer: [::std::os::raw::c_char; 512usize],
}
#[test]
fn bindgen_test_layout_luaL_Buffer() {
    const UNINIT: ::std::mem::MaybeUninit<luaL_Buffer> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<luaL_Buffer>(),
        536usize,
        concat!("Size of: ", stringify!(luaL_Buffer))
    );
    assert_eq!(
        ::std::mem::align_of::<luaL_Buffer>(),
        8usize,
        concat!("Alignment of ", stringify!(luaL_Buffer))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).p) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(luaL_Buffer),
            "::",
            stringify!(p)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).lvl) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(luaL_Buffer),
            "::",
            stringify!(lvl)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).L) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(luaL_Buffer),
            "::",
            stringify!(L)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).buffer) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(luaL_Buffer),
            "::",
            stringify!(buffer)
        )
    );
}
extern "C" {
    pub static mut pd2_log: ::std::option::Option<
        unsafe extern "C" fn(
            message: *const ::std::os::raw::c_char,
            level: ::std::os::raw::c_int,
            file: *const ::std::os::raw::c_char,
            line: ::std::os::raw::c_int,
        ),
    >;
}
extern "C" {
    pub static mut is_active_state:
        ::std::option::Option<unsafe extern "C" fn(L: *mut lua_State) -> bool>;
}
extern "C" {
    pub static mut luaL_checkstack: ::std::option::Option<
        unsafe extern "C" fn(
            L: *mut lua_State,
            sz: ::std::os::raw::c_int,
            msg: *const ::std::os::raw::c_char,
        ),
    >;
}
extern "C" {
    pub static mut lua_call: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut lua_State,
            arg2: ::std::os::raw::c_int,
            arg3: ::std::os::raw::c_int,
        ),
    >;
}
extern "C" {
    pub static mut lua_pcall: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut lua_State,
            arg2: ::std::os::raw::c_int,
            arg3: ::std::os::raw::c_int,
            arg4: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >;
}
extern "C" {
    pub static mut lua_gettop:
        ::std::option::Option<unsafe extern "C" fn(arg1: *mut lua_State) -> ::std::os::raw::c_int>;
}
extern "C" {
    pub static mut lua_settop: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut lua_State, arg2: ::std::os::raw::c_int),
    >;
}
extern "C" {
    pub static mut lua_toboolean: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut lua_State,
            arg2: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >;
}
extern "C" {
    pub static mut lua_tointeger: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut lua_State, arg2: ::std::os::raw::c_int) -> usize,
    >;
}
extern "C" {
    pub static mut lua_tonumber: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut lua_State, arg2: ::std::os::raw::c_int) -> lua_Number,
    >;
}
extern "C" {
    pub static mut lua_tolstring: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut lua_State,
            arg2: ::std::os::raw::c_int,
            arg3: *mut usize,
        ) -> *const ::std::os::raw::c_char,
    >;
}
extern "C" {
    pub static mut lua_objlen: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut lua_State, arg2: ::std::os::raw::c_int) -> usize,
    >;
}
extern "C" {
    pub static mut lua_touserdata: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut lua_State,
            arg2: ::std::os::raw::c_int,
        ) -> *mut ::std::os::raw::c_void,
    >;
}
extern "C" {
    pub static mut luaL_loadfilex: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut lua_State,
            arg2: *const ::std::os::raw::c_char,
            arg3: *const ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_int,
    >;
}
extern "C" {
    pub static mut luaL_loadstring: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut lua_State,
            arg2: *const ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_int,
    >;
}
extern "C" {
    pub static mut lua_getfield: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut lua_State,
            arg2: ::std::os::raw::c_int,
            arg3: *const ::std::os::raw::c_char,
        ),
    >;
}
extern "C" {
    pub fn lua_setfield(
        arg1: *mut lua_State,
        arg2: ::std::os::raw::c_int,
        arg3: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn lua_createtable(
        L: *mut lua_State,
        narr: ::std::os::raw::c_int,
        nrec: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub static mut lua_newuserdata: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut lua_State, arg2: usize) -> *mut ::std::os::raw::c_void,
    >;
}
extern "C" {
    pub static mut lua_insert: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut lua_State, arg2: ::std::os::raw::c_int),
    >;
}
extern "C" {
    pub static mut lua_replace: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut lua_State, arg2: ::std::os::raw::c_int),
    >;
}
extern "C" {
    pub static mut lua_remove: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut lua_State, arg2: ::std::os::raw::c_int),
    >;
}
extern "C" {
    pub static mut lua_newstate: ::std::option::Option<
        unsafe extern "C" fn(arg1: lua_Alloc, arg2: *mut ::std::os::raw::c_void) -> *mut lua_State,
    >;
}
extern "C" {
    pub static mut lua_close: ::std::option::Option<unsafe extern "C" fn(arg1: *mut lua_State)>;
}
extern "C" {
    pub static mut lua_gettable: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut lua_State, arg2: ::std::os::raw::c_int),
    >;
}
extern "C" {
    pub static mut lua_settable: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut lua_State, arg2: ::std::os::raw::c_int),
    >;
}
extern "C" {
    pub static mut lua_setmetatable: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut lua_State,
            arg2: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >;
}
extern "C" {
    pub static mut lua_getmetatable: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut lua_State,
            arg2: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >;
}
extern "C" {
    pub static mut lua_pushnumber:
        ::std::option::Option<unsafe extern "C" fn(arg1: *mut lua_State, arg2: lua_Number)>;
}
extern "C" {
    pub fn lua_pushinteger(arg1: *mut lua_State, arg2: usize);
}
extern "C" {
    pub static mut lua_pushboolean: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut lua_State, arg2: ::std::os::raw::c_int),
    >;
}
extern "C" {
    pub fn lua_pushcclosure(arg1: *mut lua_State, arg2: lua_CFunction, arg3: ::std::os::raw::c_int);
}
extern "C" {
    pub static mut lua_pushlstring: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut lua_State,
            arg2: *const ::std::os::raw::c_char,
            arg3: usize,
        ),
    >;
}
extern "C" {
    pub fn lua_pushstring(arg1: *mut lua_State, arg2: *const ::std::os::raw::c_char);
}
extern "C" {
    pub static mut lua_pushfstring: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut lua_State,
            arg2: *const ::std::os::raw::c_char,
            ...
        ) -> *const ::std::os::raw::c_char,
    >;
}
extern "C" {
    pub static mut lua_checkstack: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut lua_State,
            arg2: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >;
}
extern "C" {
    pub static mut lua_pushvalue: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut lua_State, arg2: ::std::os::raw::c_int),
    >;
}
extern "C" {
    pub static mut lua_pushnil: ::std::option::Option<unsafe extern "C" fn(arg1: *mut lua_State)>;
}
extern "C" {
    pub static mut luaI_openlib: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut lua_State,
            arg2: *const ::std::os::raw::c_char,
            arg3: *const luaL_Reg,
            arg4: ::std::os::raw::c_int,
        ),
    >;
}
extern "C" {
    pub static mut luaL_ref: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut lua_State,
            arg2: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >;
}
extern "C" {
    pub static mut lua_rawget: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut lua_State, arg2: ::std::os::raw::c_int),
    >;
}
extern "C" {
    pub static mut lua_rawset: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut lua_State, arg2: ::std::os::raw::c_int),
    >;
}
extern "C" {
    pub static mut lua_rawgeti: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut lua_State,
            arg2: ::std::os::raw::c_int,
            arg3: ::std::os::raw::c_int,
        ),
    >;
}
extern "C" {
    pub static mut lua_rawseti: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut lua_State,
            arg2: ::std::os::raw::c_int,
            arg3: ::std::os::raw::c_int,
        ),
    >;
}
extern "C" {
    pub static mut lua_type: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut lua_State,
            arg2: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >;
}
extern "C" {
    pub static mut lua_typename: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut lua_State,
            arg2: ::std::os::raw::c_int,
        ) -> *const ::std::os::raw::c_char,
    >;
}
extern "C" {
    pub static mut luaL_unref: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut lua_State,
            arg2: ::std::os::raw::c_int,
            arg3: ::std::os::raw::c_int,
        ),
    >;
}
extern "C" {
    pub static mut lua_equal: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut lua_State,
            arg2: ::std::os::raw::c_int,
            arg3: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >;
}
extern "C" {
    pub static mut luaL_newmetatable: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut lua_State,
            arg2: *const ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_int,
    >;
}
extern "C" {
    pub static mut luaL_checkudata: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut lua_State,
            arg2: ::std::os::raw::c_int,
            arg3: *const ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_int,
    >;
}
extern "C" {
    pub static mut luaL_error: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut lua_State,
            arg2: *const ::std::os::raw::c_char,
            ...
        ) -> ::std::os::raw::c_int,
    >;
}
extern "C" {
    pub static mut lua_error:
        ::std::option::Option<unsafe extern "C" fn(arg1: *mut lua_State) -> ::std::os::raw::c_int>;
}
extern "C" {
    pub fn luaL_argerror(
        L: *mut lua_State,
        narg: ::std::os::raw::c_int,
        extramsg: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn luaL_checkoption(
        L: *mut lua_State,
        narg: ::std::os::raw::c_int,
        def: *const ::std::os::raw::c_char,
        lst: *const *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn luaL_typerror(
        L: *mut lua_State,
        narg: ::std::os::raw::c_int,
        tname: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn luaL_checktype(L: *mut lua_State, narg: ::std::os::raw::c_int, t: ::std::os::raw::c_int);
}
extern "C" {
    pub fn luaL_checkany(L: *mut lua_State, narg: ::std::os::raw::c_int);
}
extern "C" {
    pub fn luaL_checklstring(
        L: *mut lua_State,
        narg: ::std::os::raw::c_int,
        len: *mut usize,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn luaL_optlstring(
        L: *mut lua_State,
        narg: ::std::os::raw::c_int,
        def: *const ::std::os::raw::c_char,
        len: *mut usize,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn luaL_checknumber(L: *mut lua_State, narg: ::std::os::raw::c_int) -> lua_Number;
}
extern "C" {
    pub fn luaL_optnumber(
        L: *mut lua_State,
        narg: ::std::os::raw::c_int,
        def: lua_Number,
    ) -> lua_Number;
}
extern "C" {
    pub fn luaL_checkinteger(L: *mut lua_State, narg: ::std::os::raw::c_int) -> lua_Integer;
}
extern "C" {
    pub fn luaL_optinteger(
        L: *mut lua_State,
        narg: ::std::os::raw::c_int,
        def: lua_Integer,
    ) -> lua_Integer;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __crt_locale_data {
    pub _address: u8,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __crt_multibyte_data {
    pub _address: u8,
}
pub type __builtin_va_list = *mut ::std::os::raw::c_char;
