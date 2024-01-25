use super::Encode;

impl<T: Encode> Encode for Option<T> {
    #[inline]
    fn encode(&self, buf: &mut Vec<u8>) {
        match self {
            None => buf.extend_from_slice(b"null"),
            Some(val) => { val.encode(buf); }
        }
    }
}

impl Encode for Option<&dyn Encode> {
    #[inline]
    fn encode(&self, buf: &mut Vec<u8>) {
        match self {
            None => buf.extend_from_slice(b"null"),
            Some(val) => { val.encode(buf); }
        }
    }
}

#[test]
fn test() {
    {
        let mut buf = vec![];
        let mut opt = Some(1);
        opt.encode(&mut buf);
        opt = None;
        opt.encode(&mut buf);
        assert_eq!(opt.stringify(), r#"null"#);
        assert_eq!(String::from_utf8_lossy(&buf), r#"1null"#);
    }

    {
        let mut buf = vec![];
        let mut opt: Option<&dyn Encode> = Some(&1);
        opt.encode(&mut buf);
        opt = None;
        opt.encode(&mut buf);
        assert_eq!(opt.stringify(), r#"null"#);
        assert_eq!(String::from_utf8_lossy(&buf), r#"1null"#);
    }
}