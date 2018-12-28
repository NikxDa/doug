use std::net::{Ipv4Addr, Ipv6Addr};

#[derive(Debug, Clone)]
pub enum DnsRecordData {
    A       { ip_addr: Ipv4Addr },
    AAAA    { ip_addr: Ipv6Addr },
    MX      { priority: u16, name: String },
    TXT     { text: String },
    CNAME   { name: String },
    HINFO   { cpu: String, os: String },
    NS      { name: String },
    PTR     { name: String },
}   