use super::Encode;

impl Encode for char {
    fn encode(&self, buf: &mut Vec<u8>) {
        let beg = buf.len();
        let max = 4 + 2; // max utf8 len + two double quotes

        unsafe {
            buf.reserve(max);
            buf.set_len(beg + max);
        }

        let len = {
            let mut cur = std::io::Cursor::new(&mut buf[beg..]);
            serde_json::to_writer(&mut cur, self).expect("encode string error");
            cur.position()
        } as usize;

        unsafe { buf.set_len(beg + len); }
    }
}

impl Encode for &str {
    #[inline]
    fn encode(&self, buf: &mut Vec<u8>) {
        let beg = buf.len();
        let max = self.len() * 2 + 2; // max bytes len + two double quotes

        unsafe {
            buf.reserve(max);
            buf.set_len(beg + max);
        }

        let len = {
            let mut cur = std::io::Cursor::new(&mut buf[beg..]);
            serde_json::to_writer(&mut cur, self).expect("encode string error");
            cur.position()
        } as usize;

        unsafe { buf.set_len(beg + len); }
    }
}

impl Encode for String {
    #[inline]
    fn encode(&self, buf: &mut Vec<u8>) {
        self.as_str().encode(buf)
    }
}

#[test]
fn test() {
    {
        assert_eq!('a'.stringify(), r#""a""#);
        assert_eq!('b'.stringify(), r#""b""#);
        assert_eq!('c'.stringify(), r#""c""#);

        let mut buf = vec![];
        'a'.encode(&mut buf);
        'b'.encode(&mut buf);
        'c'.encode(&mut buf);
        assert_eq!(String::from_utf8_lossy(&buf), r#""a""b""c""#);
    }

    {
        assert_eq!("Hello".stringify(), r#""Hello""#);
        assert_eq!("Bonjour".stringify(), r#""Bonjour""#);
        assert_eq!("你好".stringify(), r#""你好""#);
        assert_eq!("こんにちは".stringify(), r#""こんにちは""#);
        assert_eq!("สวัสดี".stringify(), r#""สวัสดี""#);

        let mut buf = vec![];
        "Hello".encode(&mut buf);
        "Bonjour".encode(&mut buf);
        "你好".encode(&mut buf);
        "こんにちは".encode(&mut buf);
        "สวัสดี".encode(&mut buf);
        assert_eq!(String::from_utf8_lossy(&buf), r#""Hello""Bonjour""你好""こんにちは""สวัสดี""#);
    }

    {
        assert_eq!(String::from("\"\\/\n\r\t").stringify(), r#""\"\\/\n\r\t""#);

        let mut buf = vec![];
        String::from("\"\\/\n\r\t").encode(&mut buf);
        assert_eq!(String::from_utf8_lossy(&buf), r#""\"\\/\n\r\t""#);
    }
}