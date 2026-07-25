use super::*;
use crate::src::leftpad::leftpad;
use crate::{__stderrp, fputs, malloc, puts, strtol};

pub(crate) extern "C" fn __main_inner(argc: i32, argv: *const *mut i8) -> Result<(), i32> {
    unsafe {
        let mut min_len: i64 = 0 as i64;
        let mut padding: *const i8 = c" ".as_ptr() as *mut i8 as *const i8;
        let mut end: *mut i8 = core::ptr::null_mut();
        let mut buf: *mut i8 = core::ptr::null_mut();
        let mut buf_len: u64 = 0 as u64;
        if argc == 4 {
            padding = unsafe { *argv.offset(3 as isize) } as *const i8;
        } else if argc != 3 {
            unsafe {
                fputs(
                    c"usage: leftpad string length [padding]\n".as_ptr() as *mut i8 as *const i8,
                    __stderrp,
                )
            };
            return Err(1);
        }
        min_len = unsafe {
            strtol(
                unsafe { *argv.offset(2 as isize) } as *const i8,
                &mut end,
                10,
            )
        };
        if (unsafe { *unsafe { (*argv.offset(2 as isize)).offset(0 as isize) } } == 0) as i32 != 0
            || unsafe { *end.offset(0 as isize) } != 0
            || min_len < 0 as i64
        {
            unsafe {
                fputs(
                    c"leftpad: invalid length".as_ptr() as *mut i8 as *const i8,
                    __stderrp,
                )
            };
            return Err(1);
        }
        buf_len = leftpad(
            unsafe { *argv.offset(1 as isize) } as *const i8,
            padding,
            min_len as u64,
            0 as *mut () as *mut i8,
            0 as u64,
        );
        if ({
            buf = unsafe { malloc(buf_len) } as *mut i8;
            buf
        })
        .is_null() as i32
            != 0
        {
            unsafe {
                fputs(
                    c"leftpad: out of memory".as_ptr() as *mut i8 as *const i8,
                    __stderrp,
                )
            };
            return Err(1);
        }
        leftpad(
            unsafe { *argv.offset(1 as isize) } as *const i8,
            padding,
            min_len as u64,
            buf,
            buf_len.wrapping_add(1 as u64),
        );
        unsafe { puts(buf as *const i8) };
        return Ok(());
    }
}
