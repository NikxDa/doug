use crate::byte_serializable::ByteSerializable;
use crate::dns::DnsUtils;

pub struct DnsHeader {
    pub id: u16,
    pub options: u16,
    pub question_count: u16,
    pub answer_count: u16,
    pub authority_count: u16,
    pub additional_count: u16
}

impl ByteSerializable for DnsHeader {
    fn to_bytes (&self) -> Vec<u8> {
        return vec![
            (self.id >> 8) as u8,
            self.id as u8,
            (self.options >> 8) as u8,
            self.options as u8,
            (self.question_count >> 8) as u8,
            self.question_count as u8,
            (self.answer_count >> 8) as u8,
            self.answer_count as u8,
            (self.authority_count >> 8) as u8,
            self.authority_count as u8,
            (self.additional_count >> 8) as u8,
            self.additional_count as u8,
        ]
    }

    fn from_bytes (bytes: &[u8]) -> DnsHeader {
        return DnsHeader {
            id: DnsUtils::bytes_to_u16 (&bytes[0..2]),
            options: DnsUtils::bytes_to_u16 (&bytes[2..4]),
            question_count: DnsUtils::bytes_to_u16 (&bytes[4..6]),
            answer_count: DnsUtils::bytes_to_u16 (&bytes[6..8]),
            authority_count: DnsUtils::bytes_to_u16 (&bytes[8..10]),
            additional_count: DnsUtils::bytes_to_u16 (&bytes[10..12])
        }
    }
}