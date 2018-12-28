use crate::byte_serializable::*;
use crate::dns::{DnsHeader, DnsQuestion};

pub struct DnsRequest {
    pub header: DnsHeader,
    pub question: DnsQuestion
}

impl ByteSerializable for DnsRequest {
    fn to_bytes (&self) -> Vec<u8> {
        let mut byte_vec = Vec::new ();
        byte_vec.extend (self.header.to_bytes ());
        byte_vec.extend (self.question.to_bytes ());
        return byte_vec;
    }

    fn from_bytes (bytes: Vec<u8>) -> DnsRequest {
        let header_bytes = bytes[0..12].to_vec ();
        let question_bytes = bytes[12..bytes.len()].to_vec ();

        return DnsRequest {
            header: DnsHeader::from_bytes (header_bytes) as DnsHeader,
            question: DnsQuestion::from_bytes (question_bytes)
        }
    }
}