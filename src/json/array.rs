use super::Encode;
use std::collections::VecDeque;

macro_rules! impl_array {
    ($t:ty) => {
        impl<T: Encode> Encode for $t {
            #[inline]
            fn encode(&self, buf: &mut Vec<u8>) {
                buf.extend_from_slice(b"[");
        
                for v in self {
                    v.encode(buf);
                    buf.extend_from_slice(b",");
                }
        
                match buf.last_mut() {
                    Some(val) if *val == b',' => *val = b']',
                    _ => buf.extend_from_slice(b"]"),
                }
            }
        }
    };
}

impl_array!([T]);
impl_array!(Vec<T>);
impl_array!(VecDeque<T>);

#[test]
fn test() {
    {
        let mut buf = vec![];
        let arr = ["hello", "world"];
        arr.encode(&mut buf);
        assert_eq!(String::from_utf8_lossy(&buf), r#"["hello","world"]"#);
    }

    {
        let mut buf = vec![];
        let mut vec = vec![];
        vec.push("hello");
        vec.push("world");
        vec.encode(&mut buf);
        assert_eq!(String::from_utf8_lossy(&buf), r#"["hello","world"]"#);
    }

    {
        let mut buf = vec![];
        let mut vec = VecDeque::new();
        vec.push_back("world");
        vec.push_front("hello");
        vec.encode(&mut buf);
        assert_eq!(String::from_utf8_lossy(&buf), r#"["hello","world"]"#);
    }
}