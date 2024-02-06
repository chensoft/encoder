#![allow(clippy::uninit_vec)]

use super::Encode;

/// Encode Integer
/// 
/// # Encode
/// 
/// ```
/// use encoder::number::Encode;
/// 
/// fn assert(val: impl Encode, cmp: String) {
///     let mut buf = vec![];
///     val.encode(&mut buf);
///     assert_eq!(String::from_utf8_lossy(&buf), cmp);
/// }
/// 
/// assert(0_i8, 0_i8.to_string());
/// assert(i8::MIN, i8::MIN.to_string());
/// assert(i8::MAX, i8::MAX.to_string());
/// 
/// assert(0_i16, 0_i16.to_string());
/// assert(i16::MIN, i16::MIN.to_string());
/// assert(i16::MAX, i16::MAX.to_string());
/// 
/// assert(0_i32, 0_i32.to_string());
/// assert(i32::MIN, i32::MIN.to_string());
/// assert(i32::MAX, i32::MAX.to_string());
/// 
/// assert(0_i64, 0_i64.to_string());
/// assert(i64::MIN, i64::MIN.to_string());
/// assert(i64::MAX, i64::MAX.to_string());
/// 
/// assert(0_i128, 0_i128.to_string());
/// assert(i128::MIN, i128::MIN.to_string());
/// assert(i128::MAX, i128::MAX.to_string());
/// 
/// assert(0_isize, 0_isize.to_string());
/// assert(isize::MIN, isize::MIN.to_string());
/// assert(isize::MAX, isize::MAX.to_string());
/// 
/// assert(u8::MIN, u8::MIN.to_string());
/// assert(u8::MAX, u8::MAX.to_string());
/// 
/// assert(u16::MIN, u16::MIN.to_string());
/// assert(u16::MAX, u16::MAX.to_string());
/// 
/// assert(u32::MIN, u32::MIN.to_string());
/// assert(u32::MAX, u32::MAX.to_string());
/// 
/// assert(u64::MIN, u64::MIN.to_string());
/// assert(u64::MAX, u64::MAX.to_string());
/// 
/// assert(u128::MIN, u128::MIN.to_string());
/// assert(u128::MAX, u128::MAX.to_string());
/// 
/// assert(usize::MIN, usize::MIN.to_string());
/// assert(usize::MAX, usize::MAX.to_string());
/// 
/// assert(true, true.to_string());
/// assert(false, false.to_string());
/// ```
/// 
/// # Compose
/// 
/// ```
/// use encoder::number::Encode;
/// 
/// let mut buf = vec![];
/// 1_i32.encode(&mut buf);
/// 2_u32.encode(&mut buf);
/// assert_eq!(String::from_utf8_lossy(&buf), r#"12"#);
/// 
/// assert_eq!(1_i32.stringify(), "1");
/// assert_eq!(2_u32.stringify(), "2");
/// ```
macro_rules! impl_integer {
    ($t:ident, $m:expr) => {
        impl Encode for $t {
            #[inline]
            fn encode(&self, buf: &mut Vec<u8>) {
                let beg = buf.len();

                unsafe {
                    buf.reserve($m);
                    buf.set_len(beg + $m);
                }

                let len = {
                    let mut cur = std::io::Cursor::new(&mut buf[beg..]);
                    let _ = simd_json::to_writer(&mut cur, self);
                    cur.position()
                };

                unsafe { buf.set_len(beg + len as usize); }
            }
        }
    };
}

impl_integer!(i8, 4);
impl_integer!(u8, 3);
impl_integer!(i16, 6);
impl_integer!(u16, 5);
impl_integer!(i32, 11);
impl_integer!(u32, 10);
impl_integer!(i64, 20);
impl_integer!(u64, 20);
impl_integer!(i128, 40);
impl_integer!(u128, 39);

#[cfg(target_pointer_width = "8")]
impl_integer!(isize, 4);
#[cfg(target_pointer_width = "8")]
impl_integer!(usize, 3);

#[cfg(target_pointer_width = "16")]
impl_integer!(isize, 6);
#[cfg(target_pointer_width = "16")]
impl_integer!(usize, 5);

#[cfg(target_pointer_width = "32")]
impl_integer!(isize, 11);
#[cfg(target_pointer_width = "32")]
impl_integer!(usize, 10);

#[cfg(target_pointer_width = "64")]
impl_integer!(isize, 20);
#[cfg(target_pointer_width = "64")]
impl_integer!(usize, 20);

#[cfg(target_pointer_width = "128")]
impl_integer!(isize, 40);
#[cfg(target_pointer_width = "128")]
impl_integer!(usize, 39);

impl_integer!(bool, 5);