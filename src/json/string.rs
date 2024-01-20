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
            let _ = simd_json::to_writer(&mut cur, self);
            cur.position()
        };

        unsafe { buf.set_len(beg + len as usize); }
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
            let _ = simd_json::to_writer(&mut cur, self);
            cur.position()
        };

        unsafe { buf.set_len(beg + len as usize); }
    }
}

impl Encode for String {
    #[inline]
    fn encode(&self, buf: &mut Vec<u8>) {
        self.as_str().encode(buf);
    }
}

#[test]
fn test() {
    {
        let mut buf = vec![];
        'a'.encode(&mut buf);
        'b'.encode(&mut buf);
        'c'.encode(&mut buf);
        assert_eq!(String::from_utf8_lossy(&buf), r#""a""b""c""#);
    }

    {
        let mut buf = vec![];
        "Hello".encode(&mut buf);
        "Bonjour".encode(&mut buf);
        "你好".encode(&mut buf);
        "こんにちは".encode(&mut buf);
        "สวัสดี".encode(&mut buf);
        assert_eq!(String::from_utf8_lossy(&buf), r#""Hello""Bonjour""你好""こんにちは""สวัสดี""#);
    }

    {
        let mut buf = vec![];
        String::from("\"\\/\n\r\t").encode(&mut buf);
        assert_eq!(String::from_utf8_lossy(&buf), r#""\"\\/\n\r\t""#);
    }
}