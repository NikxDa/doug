use crate::dns::{DnsRecordType};

pub struct DnsUtils;

impl DnsUtils {
    pub fn read_name (bytes: &Vec<u8>, mut offset: usize) -> (String, usize) {
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
                    let pointer_offset = (DnsUtils::bytes_to_u16(bytes[offset..(offset+2)].to_vec ()) ^ 0b1100_0000_0000_0000) as usize;
                    name.push_str (&DnsUtils::read_name (&bytes, pointer_offset).0);
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

    pub fn read_resource_record_data (record_type: DnsRecordType, bytes: Vec<u8>) -> String {
        return String::new ();
    }

    pub fn bytes_to_u16 (mut bytes: Vec<u8>) -> u16 {
        return 
              (bytes.remove (0) as u16) << 8 
            | (bytes.remove (0) as u16);
    }

    pub fn bytes_to_u32 (mut bytes: Vec<u8>) -> u32 {
        return 
              (bytes.remove (0) as u32) << 24 
            | (bytes.remove (0) as u32) << 16 
            | (bytes.remove (0) as u32) << 8 
            | (bytes.remove (0) as u32);
    }
}