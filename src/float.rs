use super::Encode;
use ryu::buffer::Sealed;

macro_rules! impl_float {
    ($t:ident) => {
        impl Encode for $t {
            fn encode(&self, buf: &mut Vec<u8>) {
                match !self.is_nonfinite() {
                    true => {
                        buf.reserve(24);

                        unsafe {
                            let len = self.write_to_ryu_buffer(buf.as_mut_ptr().add(buf.len()));
                            buf.set_len(buf.len() + len);
                        }
                    }
                    false => {
                        buf.extend_from_slice(self.format_nonfinite().as_bytes());
                    }
                }
            }
        }
    };
}

impl_float!(f32);
impl_float!(f64);

#[test]
fn test_encode() {
    fn assert(val: impl Encode, cmp: &str) {
        let mut buf = vec![];
        val.encode(&mut buf);
        assert_eq!(String::from_utf8_lossy(&buf), cmp);
    }

    assert(0_f32, "0.0");
    assert(1.23456_f32, "1.23456");
    assert(-1.23456_f32, "-1.23456");
    
    assert(f32::NAN, "NaN");
    assert(f32::INFINITY, "inf");
    assert(f32::NEG_INFINITY, "-inf");

    assert(0_f64, "0.0");
    assert(1.23456789012345_f64, "1.23456789012345");
    assert(-1.23456789012345_f64, "-1.23456789012345");

    assert(f64::NAN, "NaN");
    assert(f64::INFINITY, "inf");
    assert(f64::NEG_INFINITY, "-inf");
}

#[test]
fn test_compose() {
    let mut buf = vec![];
    1_f32.encode(&mut buf);
    2_f32.encode(&mut buf);
    3_f32.encode(&mut buf);
    1_f64.encode(&mut buf);
    2_f64.encode(&mut buf);
    3_f64.encode(&mut buf);
    assert_eq!(String::from_utf8_lossy(&buf), r#"1.02.03.01.02.03.0"#);
}