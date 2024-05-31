use super::Encode;
use indexmap::IndexMap;
use std::collections::HashMap;
use std::collections::BTreeMap;

/// Encode Object
///
/// # Examples
///
/// ```
/// use encoder::json::Encode;
/// use indexmap::IndexMap;
/// use std::collections::HashMap;
/// use std::collections::BTreeMap;
///
/// {
///     let mut buf = vec![];
///     let mut map = HashMap::new();
///     map.insert("hello", "world\n");
///     map.insert("aloha", "honua\n");
///     map.encode(&mut buf);
///
///     assert_eq!(matches!(serde_json::from_str::<HashMap<String, String>>(map.stringify().as_str()), Ok(map)), true);
///     assert_eq!(matches!(serde_json::from_slice::<HashMap<String, String>>(buf.as_slice()), Ok(map)), true);
/// }
///
/// {
///     let mut buf = vec![];
///     let mut map = HashMap::new();
///     map.insert(123, "world\n");
///     map.insert(456, "honua\n");
///     map.encode(&mut buf);
///
///     assert_eq!(matches!(serde_json::from_str::<HashMap<i32, String>>(map.stringify().as_str()), Ok(map)), true);
///     assert_eq!(matches!(serde_json::from_slice::<HashMap<i32, String>>(buf.as_slice()), Ok(map)), true);
/// }
///
/// {
///     let mut buf = vec![];
///     let mut map = BTreeMap::new();
///     map.insert("hello", "world\n");
///     map.insert("aloha", "honua\n");
///     map.encode(&mut buf);
///
///     assert_eq!(map.stringify(), r#"{"aloha":"honua\n","hello":"world\n"}"#);
///     assert_eq!(String::from_utf8_lossy(&buf), r#"{"aloha":"honua\n","hello":"world\n"}"#);
/// }
///
/// {
///     let mut buf = vec![];
///     let mut map = IndexMap::new();
///     map.insert("aloha", "honua\n");
///     map.insert("hello", "world\n");
///     map.encode(&mut buf);
///
///     assert_eq!(map.stringify(), r#"{"aloha":"honua\n","hello":"world\n"}"#);
///     assert_eq!(String::from_utf8_lossy(&buf), r#"{"aloha":"honua\n","hello":"world\n"}"#);
/// }
///
/// {
///     let mut buf = vec![];
///     let mut map: IndexMap<&str, &dyn Encode> = IndexMap::new();
///     map.insert("string", &"world");
///     map.insert("number", &12345);
///     map.encode(&mut buf);
///
///     assert_eq!(map.stringify(), r#"{"string":"world","number":12345}"#);
///     assert_eq!(String::from_utf8_lossy(&buf), r#"{"string":"world","number":12345}"#);
/// }
/// ```
macro_rules! impl_object {
    ($t:ident) => {
        impl<K: Encode, V: Encode> Encode for $t<K, V> {
            #[inline]
            fn encode(&self, buf: &mut Vec<u8>) {
                impl_inner!(self, buf);
            }
        }

        impl<K: Encode> Encode for $t<K, &dyn Encode> {
            #[inline]
            fn encode(&self, buf: &mut Vec<u8>) {
                impl_inner!(self, buf);
            }
        }
    };
}

macro_rules! impl_inner {
    ($self:expr, $buf:expr) => {
        $buf.extend_from_slice(b"{");

        for (k, v) in $self {
            // key
            let beg = $buf.len();
            k.encode($buf);

            match $buf.last() {
                Some(val) if *val == b'"' => $buf.extend_from_slice(b":"),
                _ => {
                    $buf.insert(beg, b'"');
                    $buf.extend_from_slice(b"\":");
                }
            }

            // val
            v.encode($buf);
            $buf.extend_from_slice(b",");
        }

        match $buf.last_mut() {
            Some(val) if *val == b',' => *val = b'}',
            _ => $buf.extend_from_slice(b"}"),
        }
    };
}

impl_object!(HashMap);
impl_object!(BTreeMap);
impl_object!(IndexMap);