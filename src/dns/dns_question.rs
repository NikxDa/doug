use crate::dns::{DnsRecordType, DnsClass, DnsUtils};
use crate::byte_serializable::*;

pub struct DnsQuestion {
    pub name: String,
    pub r#type: DnsRecordType,
    pub class: DnsClass
}

impl ToBytes for DnsQuestion {
    fn to_bytes (&self) -> Vec<u8> {
        let mut name_vec = self.name.split (".").fold (Vec::new(), |mut bytes, itm| {
            bytes.push (itm.chars ().count () as u8);
            bytes.extend (itm.bytes());
            bytes
        });

        let byte_vec = vec![
            0 as u8,
            (self.r#type.as_u16 () >> 8) as u8,
            self.r#type.as_u16 () as u8,
            (self.class.as_u16 () >> 8) as u8,
            self.class.as_u16 () as u8
        ];

        name_vec.extend (byte_vec);
        name_vec
    }
}

impl FromBytes for DnsQuestion {
    fn from_bytes (bytes: &[u8]) -> DnsQuestion {
        let question_name_result = DnsUtils::read_domain_name (&bytes, 0);

        let question_name = question_name_result.0;
        let mut byte_offset = question_name_result.1;

        let question_type = DnsUtils::bytes_to_u16 (&bytes[byte_offset..(byte_offset+2)]);
        byte_offset += 2;

        let question_class = DnsUtils::bytes_to_u16 (&bytes[byte_offset..(byte_offset+2)]);

        DnsQuestion {
            name: question_name,
            r#type: DnsRecordType::from_u16(question_type),
            class: DnsClass::from_u16(question_class)
        }
    }
}