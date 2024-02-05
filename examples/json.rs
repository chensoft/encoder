use encoder::json::Encode;

fn main() {
    let mut buf = vec![];
    "Hello, 世界! 👋".encode(&mut buf);
    assert_eq!(String::from_utf8_lossy(&buf), r#""Hello, 世界! 👋""#);
    assert_eq!("Hello, 世界! 👋".stringify(), r#""Hello, 世界! 👋""#);
}