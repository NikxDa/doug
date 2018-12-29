use crate::dns::{DnsRecordType, DnsClass, DnsUtils};
use crate::byte_serializable::*;

use num_traits::{FromPrimitive};

pub struct DnsQuestion {
    pub name: String,
    pub r#type: DnsRecordType,
    pub class: DnsClass
}

impl ByteSerializable for DnsQuestion {
    fn to_bytes (&self) -> Vec<u8> {
        let mut name_vec = self.name.split (".").fold (Vec::new(), |mut bytes, itm| {
            bytes.push (itm.chars ().count () as u8);
            bytes.extend (itm.bytes());
            return bytes;
        });

        let byte_vec = vec![
            0 as u8,
            (self.r#type as u16 >> 8) as u8,
            self.r#type as u8,
            (self.class as u16 >> 8) as u8,
            self.class as u8
        ];

        name_vec.extend (byte_vec);
        return name_vec;
    }

    fn from_bytes (bytes: &[u8]) -> DnsQuestion {
        let question_name_result = DnsUtils::read_name (&bytes, 0);

        let question_name = question_name_result.0;
        let mut byte_offset = question_name_result.1;

        let question_type = DnsUtils::bytes_to_u16 (&bytes[byte_offset..(byte_offset+2)]);
        byte_offset += 2;

        let question_class = DnsUtils::bytes_to_u16 (&bytes[byte_offset..(byte_offset+2)]);

        return DnsQuestion {
            name: question_name,
            r#type: DnsRecordType::from_u16(question_type).expect ("Invalid question type"),
            class: DnsClass::from_u16(question_class).expect ("Invalid question class")
        }
    }
}