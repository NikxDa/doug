use crate::byte_serializable::*;
use crate::dns::{DnsHeader, DnsQuestion, DnsClass, DnsRecordType, DnsResourceRecord, DnsUtils};

pub struct DnsMessage {
    pub header: DnsHeader,
    pub question: DnsQuestion,
    pub resource_records: Vec<DnsResourceRecord>
}

impl ToBytes for DnsMessage {
    fn to_bytes (&self) -> Vec<u8> {
        let mut byte_vec = Vec::new ();
        byte_vec.extend (self.header.to_bytes ());
        byte_vec.extend (self.question.to_bytes ());
        for i in 0..self.resource_records.len () {
            byte_vec.extend (self.resource_records[i].to_bytes ());
        }
        byte_vec
    }
}

impl FromBytes for DnsMessage {
    fn from_bytes (bytes: &[u8]) -> DnsMessage {
        // Define offset
        let mut byte_offset: usize = 0;

        // Read Header bytes
        let header_bytes = &bytes[0..12];
        byte_offset += header_bytes.len ();

        // Read Question bytes
        let mut question_bytes: Vec<u8> = vec![];
        while bytes[byte_offset] != 0b00000000 {
            question_bytes.push(bytes[byte_offset]);
            byte_offset += 1;
        }
        question_bytes.extend (&bytes[byte_offset..(byte_offset+5)].to_vec ());
        byte_offset += 5;

        // Read Resource Records
        let mut resource_records: Vec<DnsResourceRecord> = Vec::new ();

        while byte_offset < bytes.len () {
            let (record_name, bytes_read) = DnsUtils::read_domain_name (&bytes, byte_offset);
            byte_offset += bytes_read;

            let record_type = DnsUtils::bytes_to_u16 (&bytes[byte_offset..(byte_offset+2)]);
            byte_offset += 2;

            let record_class = DnsUtils::bytes_to_u16 (&bytes[byte_offset..(byte_offset+2)]);
            byte_offset += 2;

            let record_ttl = DnsUtils::bytes_to_u32 (&bytes[byte_offset..(byte_offset+4)]);
            byte_offset += 4;

            let record_data_length = DnsUtils::bytes_to_u16 (&bytes[byte_offset..(byte_offset+2)]);
            byte_offset += 2;

            let record_data = bytes[byte_offset..(byte_offset+(record_data_length as usize))].to_vec ();
            byte_offset += record_data_length as usize;

            let parsed_data = DnsUtils::read_resource_record_data (
                &bytes,
                DnsRecordType::from_u16(record_type),
                byte_offset - record_data_length as usize,
                record_data_length as usize
            );

            resource_records.push (DnsResourceRecord {
                name: record_name,
                r#type: DnsRecordType::from_u16 (record_type),
                class: DnsClass::from_u16 (record_class),
                ttl: record_ttl,
                length: record_data_length,
                data: record_data,
                parsed_data: parsed_data
            });
        }

        DnsMessage {
            header: DnsHeader::from_bytes (header_bytes) as DnsHeader,
            question: DnsQuestion::from_bytes (&question_bytes),
            resource_records: resource_records
        }
    }
}