use super::Encode;

/// Encode Number
/// 
/// # Encode
/// 
/// ```
/// use encoder::json::Encode;
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
/// 
/// assert(0_f32, "0.0".to_string());
/// assert(1.23456_f32, "1.23456".to_string());
/// assert(-1.23456_f32, "-1.23456".to_string());
/// 
/// assert(0_f64, "0.0".to_string());
/// assert(1.23456789012345_f64, "1.23456789012345".to_string());
/// assert(-1.23456789012345_f64, "-1.23456789012345".to_string());
/// ```
/// 
/// # Compose
/// 
/// ```
/// use encoder::json::Encode;
/// 
/// let mut buf = vec![];
/// 1_i32.encode(&mut buf);
/// 2_u32.encode(&mut buf);
/// assert_eq!(String::from_utf8_lossy(&buf), r#"12"#);
/// ```
macro_rules! impl_number {
    ($t:ident) => {
        impl Encode for $t {
            #[inline]
            fn encode(&self, buf: &mut Vec<u8>) {
                $crate::number::Encode::encode(self, buf)
            }
        }
    };
}

impl_number!(i8);
impl_number!(u8);
impl_number!(i16);
impl_number!(u16);
impl_number!(i32);
impl_number!(u32);
impl_number!(i64);
impl_number!(u64);
impl_number!(i128);
impl_number!(u128);

impl_number!(isize);
impl_number!(usize);

impl_number!(bool);

impl_number!(f32);
impl_number!(f64);