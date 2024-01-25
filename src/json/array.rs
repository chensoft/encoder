use super::Encode;
use indexmap::IndexSet;
use std::collections::HashSet;
use std::collections::BTreeSet;
use std::collections::VecDeque;

macro_rules! impl_array_obj {
    ($t:ty) => {
        impl<T: Encode> Encode for $t {
            #[inline]
            fn encode(&self, buf: &mut Vec<u8>) {
                impl_inner!(self, buf);
            }
        }
    };
}

macro_rules! impl_array_ref {
    ($t:ty) => {
        impl Encode for $t {
            #[inline]
            fn encode(&self, buf: &mut Vec<u8>) {
                impl_inner!(self, buf);
            }
        }
    };
}

macro_rules! impl_inner {
    ($self:expr, $buf:expr) => {
        $buf.extend_from_slice(b"[");

        for v in $self {
            v.encode($buf);
            $buf.extend_from_slice(b",");
        }

        match $buf.last_mut() {
            Some(val) if *val == b',' => *val = b']',
            _ => $buf.extend_from_slice(b"]"),
        }
    };
}

impl_array_obj!([T]);
impl_array_obj!(Vec<T>);
impl_array_obj!(VecDeque<T>);

impl_array_obj!(HashSet<T>);
impl_array_obj!(BTreeSet<T>);
impl_array_obj!(IndexSet<T>);

impl_array_ref!([&dyn Encode]);
impl_array_ref!(Vec<&dyn Encode>);
impl_array_ref!(VecDeque<&dyn Encode>);

impl_array_ref!(HashSet<&dyn Encode>);
impl_array_ref!(BTreeSet<&dyn Encode>);
impl_array_ref!(IndexSet<&dyn Encode>);

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
        let mut vec: Vec<&dyn Encode> = vec![];
        vec.push(&"hello");
        vec.push(&12345);
        vec.encode(&mut buf);
        assert_eq!(vec.stringify(), r#"["hello",12345]"#);
        assert_eq!(String::from_utf8_lossy(&buf), r#"["hello",12345]"#);
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