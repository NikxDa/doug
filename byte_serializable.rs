pub trait ByteSerializable {
    fn to_bytes (&self) -> Vec<u8>;
    fn from_bytes (bytes: Vec<u8>) -> Self;
}