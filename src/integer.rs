use super::Encode;
use std::mem::MaybeUninit;
use itoa::private::Sealed;

macro_rules! impl_integer {
    ($t:ident, $m:ident) => {
        impl Encode for $t {
            fn encode(&self, buf: &mut Vec<u8>) {
                buf.reserve(itoa::$m);

                unsafe {
                    let ptr = buf.as_mut_ptr().add(buf.len()) as *mut [MaybeUninit<u8>; itoa::$m];
                    let arr = &mut *(ptr as *mut [MaybeUninit<u8>; itoa::$m]);
                    let val = self.write(arr);
                    let len = val.len();
                    std::ptr::copy_nonoverlapping(val.as_ptr(), ptr as *mut u8, len);
                    buf.set_len(buf.len() + len);
                }
            }
        }
    };
}

impl Encode for bool {
    fn encode(&self, buf: &mut Vec<u8>) {
        match *self {
            true => buf.extend_from_slice("true".as_bytes()),
            false => buf.extend_from_slice("false".as_bytes()),
        }
    }
}

impl_integer!(i8, I8_MAX_LEN);
impl_integer!(u8, U8_MAX_LEN);
impl_integer!(i16, I16_MAX_LEN);
impl_integer!(u16, U16_MAX_LEN);
impl_integer!(i32, I32_MAX_LEN);
impl_integer!(u32, U32_MAX_LEN);
impl_integer!(i64, I64_MAX_LEN);
impl_integer!(u64, U64_MAX_LEN);
impl_integer!(i128, I128_MAX_LEN);
impl_integer!(u128, U128_MAX_LEN);

#[cfg(target_pointer_width = "8")]
impl_integer!(isize, I8_MAX_LEN);
#[cfg(target_pointer_width = "8")]
impl_integer!(usize, U8_MAX_LEN);

#[cfg(target_pointer_width = "16")]
impl_integer!(isize, I16_MAX_LEN);
#[cfg(target_pointer_width = "16")]
impl_integer!(usize, U16_MAX_LEN);

#[cfg(target_pointer_width = "32")]
impl_integer!(isize, I32_MAX_LEN);
#[cfg(target_pointer_width = "32")]
impl_integer!(usize, U32_MAX_LEN);

#[cfg(target_pointer_width = "64")]
impl_integer!(isize, I64_MAX_LEN);
#[cfg(target_pointer_width = "64")]
impl_integer!(usize, U64_MAX_LEN);

#[cfg(target_pointer_width = "128")]
impl_integer!(isize, I128_MAX_LEN);
#[cfg(target_pointer_width = "128")]
impl_integer!(usize, U128_MAX_LEN);

#[test]
fn test_encode() {
    fn assert(val: impl Encode, cmp: String) {
        let mut buf = vec![];
        val.encode(&mut buf);
        assert_eq!(String::from_utf8_lossy(&buf), cmp);
    }

    assert(0_i8, 0_i8.to_string());
    assert(i8::MIN, i8::MIN.to_string());
    assert(i8::MAX, i8::MAX.to_string());

    assert(0_i16, 0_i16.to_string());
    assert(i16::MIN, i16::MIN.to_string());
    assert(i16::MAX, i16::MAX.to_string());

    assert(0_i32, 0_i32.to_string());
    assert(i32::MIN, i32::MIN.to_string());
    assert(i32::MAX, i32::MAX.to_string());

    assert(0_i64, 0_i64.to_string());
    assert(i64::MIN, i64::MIN.to_string());
    assert(i64::MAX, i64::MAX.to_string());

    assert(0_i128, 0_i128.to_string());
    assert(i128::MIN, i128::MIN.to_string());
    assert(i128::MAX, i128::MAX.to_string());

    assert(0_isize, 0_isize.to_string());
    assert(isize::MIN, isize::MIN.to_string());
    assert(isize::MAX, isize::MAX.to_string());

    assert(u8::MIN, u8::MIN.to_string());
    assert(u8::MAX, u8::MAX.to_string());

    assert(u16::MIN, u16::MIN.to_string());
    assert(u16::MAX, u16::MAX.to_string());

    assert(u32::MIN, u32::MIN.to_string());
    assert(u32::MAX, u32::MAX.to_string());

    assert(u64::MIN, u64::MIN.to_string());
    assert(u64::MAX, u64::MAX.to_string());

    assert(u128::MIN, u128::MIN.to_string());
    assert(u128::MAX, u128::MAX.to_string());

    assert(usize::MIN, usize::MIN.to_string());
    assert(usize::MAX, usize::MAX.to_string());

    assert(true, true.to_string());
    assert(false, false.to_string());
}

#[test]
fn test_compose() {
    let mut buf = vec![];
    1_i32.encode(&mut buf);
    2_u32.encode(&mut buf);
    assert_eq!(String::from_utf8_lossy(&buf), r#"12"#);
}