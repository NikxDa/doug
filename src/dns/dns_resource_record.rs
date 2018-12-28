use crate::dns::{DnsRecordData, DnsRecordType, DnsClass};

pub struct DnsResourceRecord {
    pub name: String,
    pub r#type: DnsRecordType,
    pub class: DnsClass,
    pub ttl: u32,
    pub length: u16,
    pub data: Vec<u8>,
    pub parsed_data: DnsRecordData
}