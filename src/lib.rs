pub trait Encode: Sync + Send {
    fn encode(&self, buf: &mut Vec<u8>);
}

pub mod float;
pub mod integer;