#[derive(Debug, Copy, Clone, EnumString)]
pub enum DnsClass {
    Internet, 
    CSNet,    
    Chaos,    
    Hesiod,   

    Custom (u16)
}

impl DnsClass {
    pub fn as_u16 (self) -> u16 {
        match self {
            DnsClass::Internet      => 1,
            DnsClass::CSNet         => 2,
            DnsClass::Chaos         => 3,
            DnsClass::Hesiod        => 4,
            DnsClass::Custom (data) => data
        }
    }

    pub fn from_u16 (data: u16) -> Self {
        match data {
            1 => DnsClass::Internet,
            2 => DnsClass::CSNet,
            3 => DnsClass::Chaos,
            4 => DnsClass::Hesiod,
            _ => DnsClass::Custom (data)
        }
    }
}