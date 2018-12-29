use crate::dns::{DnsRecordType, DnsRecordData};
use std::net::{Ipv4Addr, Ipv6Addr};

pub struct DnsUtils;

impl DnsUtils {
    pub fn read_domain_name (bytes: &[u8], mut offset: usize) -> (String, usize) {
        let mut name = String::new ();
        let mut bytes_read: usize = 0;

        loop {
            let current_byte = bytes [offset];

            match current_byte >> 6 {
                0b00000000 => {
                    if current_byte == 0b00000000 {
                        // Null-Terminator, remove last '.'
                        name.pop ();
                        bytes_read += 1;
                        break;
                    } else {
                        let label_size = current_byte as usize;
                        let label_bytes = &bytes[(offset+1)..=(offset+label_size)];
                        for label_byte in label_bytes {
                            name.push (label_byte.clone () as char);
                        }
                        name.push ('.');
                        offset += 1 + label_size;
                        bytes_read += 1 + label_size;
                    }
                }
                0b00000011 => {
                    let pointer_offset = (DnsUtils::bytes_to_u16(&bytes[offset..(offset+2)]) ^ 0b1100_0000_0000_0000) as usize;
                    name.push_str (&DnsUtils::read_domain_name (&bytes, pointer_offset).0);
                    bytes_read += 2;
                    break;
                }
                _ => {
                    panic!("Encountered unexpected byte in name: {}", current_byte);
                }
            }
        }

        (name, bytes_read)
    }

    pub fn read_character_string (bytes: &[u8], offset: usize) -> (String, usize) {
        let mut text = String::new ();

        let string_length = bytes[offset];
        for i in 1..=string_length {
            text.push (bytes[offset + (i as usize)] as char);
        }

        return (text, string_length as usize);
    }

    pub fn read_resource_record_data (bytes: &[u8], record_type: DnsRecordType, record_data_offset: usize, record_data_length: usize) -> DnsRecordData {
        let record_data: Vec<u8> = bytes[record_data_offset..(record_data_offset + record_data_length)].to_vec ();
        
        match record_type {
            DnsRecordType::A => {
                let ip_addr: Ipv4Addr = record_data
                    .iter ()
                    .map (|itm| itm.to_string ())
                    .collect::<Vec<String>>()
                    .join (".")
                    .parse ()
                    .expect ("Could not parse A record IP address");
                return DnsRecordData::A { ip_addr: ip_addr };
            },
            DnsRecordType::AAAA => {
                let ip_parts: Vec<u16> = record_data
                    .chunks (2)
                    .map (|itm| (itm[0] as u16) << 8 | (itm[1] as u16))
                    .collect::<Vec<u16>>();
                
                let ip_addr = Ipv6Addr::new (
                    ip_parts [0], ip_parts [1], ip_parts [2], ip_parts [3],
                    ip_parts [4], ip_parts [5], ip_parts [6], ip_parts [7]
                );
                return DnsRecordData::AAAA { ip_addr: ip_addr };
            },
            DnsRecordType::NS => {
                let name = DnsUtils::read_domain_name (&bytes, record_data_offset).0;
                return DnsRecordData::NS { name: name };
            },
            DnsRecordType::CNAME => {
                let name = DnsUtils::read_domain_name (&bytes, record_data_offset).0;
                return DnsRecordData::CNAME { name: name };
            },
            DnsRecordType::PTR => {
                let name = DnsUtils::read_domain_name (&bytes, record_data_offset).0;
                return DnsRecordData::PTR { name: name };
            },
            DnsRecordType::MX => {
                let priority = DnsUtils::bytes_to_u16 (&record_data[0..2]);
                let name = DnsUtils::read_domain_name (&bytes, record_data_offset + 2).0;
                return DnsRecordData::MX { priority: priority, name: name };
            },
            DnsRecordType::TXT => {
                let text = DnsUtils::read_character_string (&bytes, record_data_offset).0;
                return DnsRecordData::TXT { text: text };
            },
            _ => {
                return DnsRecordData::None;
            }
        }
    }

    pub fn domain_name_to_bytes (name: &String) -> Vec<u8> {
        name
            .split (".")
            .map (|itm| -> Vec<u8> {
                let mut vec = vec![itm.chars ().count () as u8];
                vec.extend (itm.chars ().map (|ch| ch as u8));
                vec
            })
            .fold (Vec::new (), |mut vec, itm| {
                vec.extend (itm);
                vec
            })
    }

    pub fn bytes_to_u16 (bytes: &[u8]) -> u16 {
        return 
              (bytes[0] as u16) << 8 
            | (bytes[1] as u16);
    }

    pub fn bytes_to_u32 (bytes: &[u8]) -> u32 {
        return 
              (bytes[0] as u32) << 24 
            | (bytes[1] as u32) << 16 
            | (bytes[2] as u32) << 8 
            | (bytes[3] as u32);
    }
}