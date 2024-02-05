use super::Encode;

/// Encode Option 1
/// 
/// ```
/// use encoder::json::Encode;
/// 
/// let mut buf = vec![];
/// let mut opt = Some(1);
/// opt.encode(&mut buf);
/// opt = None;
/// opt.encode(&mut buf);
/// assert_eq!(opt.stringify(), r#"null"#);
/// assert_eq!(String::from_utf8_lossy(&buf), r#"1null"#);
/// ```
impl<T: Encode> Encode for Option<T> {
    #[inline]
    fn encode(&self, buf: &mut Vec<u8>) {
        match self {
            None => buf.extend_from_slice(b"null"),
            Some(val) => { val.encode(buf); }
        }
    }
}

/// Encode Option 2
/// 
/// ```
/// use encoder::json::Encode;
/// 
/// let mut buf = vec![];
/// let mut opt: Option<&dyn Encode> = Some(&1);
/// opt.encode(&mut buf);
/// opt = None;
/// opt.encode(&mut buf);
/// assert_eq!(opt.stringify(), r#"null"#);
/// assert_eq!(String::from_utf8_lossy(&buf), r#"1null"#);
/// ```
impl Encode for Option<&dyn Encode> {
    #[inline]
    fn encode(&self, buf: &mut Vec<u8>) {
        match self {
            None => buf.extend_from_slice(b"null"),
            Some(val) => { val.encode(buf); }
        }
    }
}