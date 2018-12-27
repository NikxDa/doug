use crate::byte_serializable::ByteSerializable;

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

    fn from_bytes (mut bytes: Vec<u8>) -> DnsHeader {
        return DnsHeader {
            id: ((bytes.remove (0) as u16) << 8) | (bytes.remove (0) as u16),
            options: ((bytes.remove (0) as u16) << 8) | (bytes.remove (0) as u16),
            question_count: ((bytes.remove (0) as u16) << 8) | (bytes.remove (0) as u16),
            answer_count: ((bytes.remove (0) as u16) << 8) | (bytes.remove (0) as u16),
            authority_count: ((bytes.remove (0) as u16) << 8) | (bytes.remove (0) as u16),
            additional_count: ((bytes.remove (0) as u16) << 8) | (bytes.remove (0) as u16)
        }
    }
}