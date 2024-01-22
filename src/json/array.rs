use super::Encode;
use indexmap::IndexSet;
use std::collections::HashSet;
use std::collections::BTreeSet;
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

impl_array!(HashSet<T>);
impl_array!(BTreeSet<T>);
impl_array!(IndexSet<T>);

#[test]
fn test() {
    {
        let mut buf = vec![];
        let arr = ["hello", "world"];
        arr.encode(&mut buf);
        assert_eq!(arr.stringify(), r#"["hello","world"]"#);
        assert_eq!(String::from_utf8_lossy(&buf), r#"["hello","world"]"#);
    }

    {
        let mut buf = vec![];
        let mut vec = vec![];
        vec.push("hello");
        vec.push("world");
        vec.encode(&mut buf);
        assert_eq!(vec.stringify(), r#"["hello","world"]"#);
        assert_eq!(String::from_utf8_lossy(&buf), r#"["hello","world"]"#);
    }

    {
        let mut buf = vec![];
        let mut vec = VecDeque::new();
        vec.push_back("world");
        vec.push_front("hello");
        vec.encode(&mut buf);
        assert_eq!(vec.stringify(), r#"["hello","world"]"#);
        assert_eq!(String::from_utf8_lossy(&buf), r#"["hello","world"]"#);
    }

    {
        let mut buf = vec![];
        let mut set = HashSet::new();
        set.insert("world");
        set.insert("hello");
        set.encode(&mut buf);
        assert_eq!(unsafe { simd_json::from_str::<HashSet<&str>>(set.stringify().as_mut_str()).unwrap() }, set);
        assert_eq!(simd_json::from_slice::<HashSet<&str>>(buf.as_mut_slice()).unwrap(), set);
    }

    {
        let mut buf = vec![];
        let mut set = BTreeSet::new();
        set.insert("world");
        set.insert("hello");
        set.encode(&mut buf);
        assert_eq!(set.stringify(), r#"["hello","world"]"#);
        assert_eq!(String::from_utf8_lossy(&buf), r#"["hello","world"]"#);
    }

    {
        let mut buf = vec![];
        let mut set = IndexSet::new();
        set.insert("world");
        set.insert("hello");
        set.encode(&mut buf);
        assert_eq!(set.stringify(), r#"["world","hello"]"#);
        assert_eq!(String::from_utf8_lossy(&buf), r#"["world","hello"]"#);
    }
}