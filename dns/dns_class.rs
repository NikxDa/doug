// Primitive Enum Macro
use enum_primitive_derive::Primitive;

#[derive(Debug, Copy, Clone, Primitive, EnumString)]
pub enum DnsClass {
    Internet    = 1,
    CSNet       = 2,
    Chaos       = 3,
    Hesiod      = 4,
}