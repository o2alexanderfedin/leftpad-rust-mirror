#![allow(unused_imports, dead_code)]

mod bin;
mod src;
use crate::bin::main::__main_inner;

pub(crate) type DarwinSizeT = u64;

#[unsafe(no_mangle)]
pub extern "C" fn main(argc: i32, argv: *const *mut i8) -> i32 {
    let __r: Result<(), i32> = __main_inner(argc, argv);
    if __r.is_ok() {
        return 0;
    }
    return __r.unwrap_err();
}

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) struct SFILE {
    pub(crate) _opaque: [u8; 0],
}

pub(crate) type FILE = SFILE;

extern "C" {
    fn __transpiler_isa(child: i32, ancestor: i32) -> bool;
    fn fputs(_: *const i8, _: *mut FILE) -> i32;
    fn strtol(__str: *const i8, __endptr: *mut *mut i8, __base: i32) -> i64;
    fn malloc(__size: u64) -> *mut ();
    fn puts(_: *const i8) -> i32;
    static mut __stderrp: *mut FILE;
}
