use super::Encode;
use ryu::raw::format32;
use ryu::raw::format64;

/// Format non-finite float, copied from ryu
#[cold]
#[inline]
fn format_nonfinite32(f: f32) -> &'static str {
    const MANTISSA_MASK: u32 = 0x007fffff;
    const SIGN_MASK: u32 = 0x80000000;
    let bits = f.to_bits();
    if bits & MANTISSA_MASK != 0 {
        "NaN"
    } else if bits & SIGN_MASK != 0 {
        "-inf"
    } else {
        "inf"
    }
}

/// Format non-finite float, copied from ryu
#[cold]
#[inline]
fn format_nonfinite64(f: f64) -> &'static str {
    const MANTISSA_MASK: u64 = 0x000fffffffffffff;
    const SIGN_MASK: u64 = 0x8000000000000000;
    let bits = f.to_bits();
    if bits & MANTISSA_MASK != 0 {
        "NaN"
    } else if bits & SIGN_MASK != 0 {
        "-inf"
    } else {
        "inf"
    }
}

/// Encode Float
///
/// # Encode
///
/// ```
/// use encoder::number::Encode;
///
/// fn assert(val: impl Encode, cmp: &str) {
///     let mut buf = vec![];
///     val.encode(&mut buf);
///     assert_eq!(String::from_utf8_lossy(&buf), cmp);
/// }
///
/// assert(0_f32, "0.0");
/// assert(1.23456_f32, "1.23456");
/// assert(-1.23456_f32, "-1.23456");
///
/// assert(f32::NAN, "NaN");
/// assert(f32::INFINITY, "inf");
/// assert(f32::NEG_INFINITY, "-inf");
///
/// assert(0_f64, "0.0");
/// assert(1.23456789012345_f64, "1.23456789012345");
/// assert(-1.23456789012345_f64, "-1.23456789012345");
///
/// assert(f64::NAN, "NaN");
/// assert(f64::INFINITY, "inf");
/// assert(f64::NEG_INFINITY, "-inf");
/// ```
///
/// # Compose
///
/// ```
/// use encoder::number::Encode;
///
/// let mut buf = vec![];
/// 1_f32.encode(&mut buf);
/// 2_f32.encode(&mut buf);
/// 3_f32.encode(&mut buf);
/// 1_f64.encode(&mut buf);
/// 2_f64.encode(&mut buf);
/// 3_f64.encode(&mut buf);
/// assert_eq!(String::from_utf8_lossy(&buf), r#"1.02.03.01.02.03.0"#);
///
/// assert_eq!(1_f32.stringify(), "1.0");
/// assert_eq!(2_f32.stringify(), "2.0");
/// assert_eq!(3_f32.stringify(), "3.0");
/// ```
macro_rules! impl_float {
    ($t:ident, $f:ident, $n:ident) => {
        impl Encode for $t {
            #[inline]
            fn encode(&self, buf: &mut Vec<u8>) {
                match self.is_finite() {
                    true => {
                        buf.reserve(24);

                        unsafe {
                            let len = ($f)(*self, buf.as_mut_ptr().add(buf.len()));
                            buf.set_len(buf.len() + len);
                        }
                    }
                    false => {
                        buf.extend_from_slice(($n)(*self).as_bytes());
                    }
                }
            }
        }
    };
}

impl_float!(f32, format32, format_nonfinite32);
impl_float!(f64, format64, format_nonfinite64);