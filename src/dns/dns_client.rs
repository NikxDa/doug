use std::net::{UdpSocket, SocketAddr, IpAddr, Ipv4Addr};
use std::fs;
use regex::Regex;
use rand::Rng;

use crate::dns::{DnsHeader, DnsQuestion, DnsResourceRecord, DnsRequest, DnsResponse, DnsClass, DnsRecordType};
use crate::byte_serializable::ByteSerializable;

pub struct DnsClient {
    udp_socket: UdpSocket,
    pub local_addr: SocketAddr,
    pub dns_addr: SocketAddr
}

impl DnsClient {
    pub fn new () -> DnsClient {
        let dns_addr: Ipv4Addr = Self::get_local_dns_addr ();
        let local_addr: Ipv4Addr = "0.0.0.0".parse ().expect ("Could not parse local DNS address.");

        let local_port = rand::thread_rng ().gen_range (50000, 60000);

        let dns_sock_addr = SocketAddr::new(IpAddr::V4 (dns_addr), 53);
        let local_sock_addr = SocketAddr::new(IpAddr::V4 (local_addr), local_port);

        return DnsClient {
            dns_addr: dns_sock_addr,
            local_addr: local_sock_addr,
            udp_socket: UdpSocket::bind(local_sock_addr).expect("Failed to bind socket")
        }
    }

    pub fn set_dns_address (&mut self, dns_addr: String) -> &DnsClient {
        let new_dns_addr: Ipv4Addr = match dns_addr.parse () {
            Ok(itm) => itm,
            Err(_) => {
                let records: Vec<DnsResourceRecord> = self.query (dns_addr, DnsRecordType::A).resource_records;
                if records.len () > 0 {
                    let ip_string: String = records
                        .first ()
                        .unwrap ()
                        .data
                        .iter ()
                        .map (|itm| itm.to_string ())
                        .collect::<Vec<String>>().join (".");

                    match ip_string.parse () {
                        Ok(itm) => itm,
                        Err (_) => return self
                    }
                } else {
                    return self;
                }
            }
        };

        self.dns_addr = SocketAddr::new(IpAddr::V4 (new_dns_addr), 53);
        return self;
    }

    pub fn query (&self, url: String, record_type: DnsRecordType) -> DnsResponse {
        let dns_header = DnsHeader {
            id: Self::get_request_id (),
            options: 0b_0000000100000000,
            question_count: 1,
            answer_count: 0,
            authority_count: 0,
            additional_count: 0
        };

        let dns_question = DnsQuestion {
            name: url,
            r#type: record_type,
            class: DnsClass::Internet
        };

        let dns_request = DnsRequest {
            header: dns_header,
            question: dns_question
        };

        let mut buf = [0u8; 512];
        self.udp_socket.send_to (&dns_request.to_bytes (), self.dns_addr).expect ("Failed to send DNS request.");
        let result = self.udp_socket.recv(&mut buf);

        match result {
            Ok(length) => {
                let response = DnsResponse::from_bytes (&buf[0..length]);
                return response;
            }
            Err(_) => {
                panic!("Did not receive correct data.");
            }
        }
    }

    fn get_request_id () -> u16 {
        return rand::thread_rng().gen_range(0, 65535);
    }

    fn get_local_dns_addr () -> Ipv4Addr {
        let fs_result = fs::read_to_string ("/etc/resolv.conf");

        match fs_result {
            Ok(data) => {
                let dns_regex = Regex::new (r"nameserver\s(?P<dns>(?:[0-9]{1,3}\.?){4})")
                    .expect ("Failed to construct regular expression.");

                let captures = dns_regex.captures (&data as &str)
                    .expect ("No capture groups found.");

                let default_dns: String = String::from (&captures ["dns"]);
                return default_dns.parse ().expect ("Could not parse default DNS address.");
            }
            Err(_) => {
                // Fallback to Google DNS
                let fallback_dns = "8.8.8.8";
                return fallback_dns.parse ().expect ("Could not parse fallback DNS address.");
            }
        }
    }
}