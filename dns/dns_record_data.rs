// Primitive Enum Macro
use enum_primitive_derive::Primitive;

#[derive(Debug, Copy, Clone, Primitive, Display, EnumString)]
pub enum DnsRecordData {
    A       (IpAddr)
}