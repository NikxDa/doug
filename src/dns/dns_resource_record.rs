use crate::dns::{DnsRecordData, DnsRecordType, DnsClass, DnsUtils};
use crate::byte_serializable::{ToBytes};

pub struct DnsResourceRecord {
    pub name: String,
    pub r#type: DnsRecordType,
    pub class: DnsClass,
    pub ttl: u32,
    pub length: u16,
    pub data: Vec<u8>,
    pub parsed_data: DnsRecordData
}

impl ToBytes for DnsResourceRecord {
    fn to_bytes (&self) -> Vec<u8> {
        let mut bytes = Vec::new ();
        bytes.extend (DnsUtils::domain_name_to_bytes (&self.name));
        bytes.extend (vec![
            (self.r#type.as_u16 () >> 8) as u8,
            self.r#type.as_u16 () as u8,
            (self.class.as_u16 () >> 8) as u8,
            self.class.as_u16 () as u8,
            (self.ttl as u32 >> 24) as u8,
            (self.ttl as u32 >> 16) as u8,
            (self.ttl as u32 >> 8) as u8,
            self.ttl as u8,
            (self.length as u16 >> 8) as u8,
            self.length as u8,
        ]);
        bytes.extend (&self.data);
        bytes
    }
}