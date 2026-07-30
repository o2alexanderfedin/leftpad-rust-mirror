use super::*;

///leftpad() - pad a string to a desired length
///
///Writes `padding` (or spaces if NULL or empty) and `str` (if not NULL) to
///`dest`, where `padding` is repeated or cut of as needed to make for a
///minimum final string length of `min_len`.
///
///If `dest` is NULL or `dest_sz` 0, nothing is written. Otherwise, up to
///`dest_sz` characters are written, including the \0 terminator, possibly
///truncating the output.
///
///Returns the length of the output string. If less than `dest_sz`, the output
///has not bene truncated.
pub(crate) extern "C" fn leftpad(
    str: *const i8,
    mut padding: *const i8,
    min_len: u64,
    dest: *mut i8,
    dest_sz: u64,
) -> u64 {
    let mut i: u64 = 0 as u64;
    let mut str_len: u64 = 0 as u64;
    let mut dest_len: u64 = 0 as u64;
    let mut npad: u64 = 0 as u64;
    while !(str).is_null() && unsafe { *str.add(str_len as usize) } != 0 {
        {
            let __old = str_len;
            str_len = str_len.wrapping_add(1);
            __old
        };
    }
    if (padding).is_null() as i32 != 0 || (unsafe { *padding.offset(0 as isize) } == 0) as i32 != 0
    {
        padding = c" ".as_ptr() as *mut i8 as *const i8;
    }
    if str_len < min_len {
        npad = min_len.wrapping_sub(str_len);
    }
    if (dest).is_null() as i32 != 0 || (dest_sz == 0) as i32 != 0 {
        return str_len.wrapping_add(npad);
    }
    while dest_len < npad && dest_len < dest_sz.wrapping_sub(1 as u64) {
        if ({
            let __v = unsafe {
                *padding.add({
                    let __old = i;
                    i = i.wrapping_add(1);
                    __old
                } as usize)
            } as i8;
            unsafe {
                *dest.add({
                    let __old = dest_len;
                    dest_len = dest_len.wrapping_add(1);
                    __old
                } as usize) = __v
            };
            __v
        } == 0) as i32
            != 0
        {
            unsafe {
                *dest.add(dest_len.wrapping_sub(1 as u64) as usize) =
                    unsafe { *padding.offset(0 as isize) } as i8
            };
            i = 1 as u64;
        }
    }
    {
        i = 0 as u64;
        '__b2: loop {
            if !(i < str_len && dest_len < dest_sz.wrapping_sub(1 as u64)) {
                break '__b2;
            }
            '__c2: loop {
                unsafe {
                    *dest.add({
                        let __old = dest_len;
                        dest_len = dest_len.wrapping_add(1);
                        __old
                    } as usize) = unsafe { *str.add(i as usize) } as i8
                };
                break '__c2;
            }
            i = i.wrapping_add(1);
        }
    }
    unsafe { *dest.add(dest_len as usize) = '\u{0}' as i32 as i8 };
    return dest_len;
}
