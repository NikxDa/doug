pub trait ToBytes {
    fn to_bytes (&self) -> Vec<u8>;
}

pub trait FromBytes {
    fn from_bytes (bytes: &[u8]) -> Self;
}