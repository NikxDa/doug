// Primitive Enum Macro

#[derive(Debug, Copy, Clone, Display, EnumString, PartialEq)]
pub enum DnsRecordType {
    A, 	   
    NS, 	   
    MD, 	   
    MF, 	   
    CNAME, 
    SOA, 
    MB, 	   
    MG, 	   
    MR, 	   
    NULL, 	
    WKS, 	
    PTR, 	
    HINFO, 	
    MINFO, 	
    MX, 	    
    TXT, 	
    AAAA,    
    OPT,     
    AXFR, 	
    MAILB, 	
    MAILA, 	
    ALL, 	

    Custom (u16)
}

impl DnsRecordType {
    pub fn as_u16 (self) -> u16 {
        match self {
            DnsRecordType::A                => 1,
            DnsRecordType::NS               => 2,
            DnsRecordType::MD               => 3,
            DnsRecordType::MF               => 4,
            DnsRecordType::CNAME            => 5,
            DnsRecordType::SOA              => 6,
            DnsRecordType::MB               => 7,
            DnsRecordType::MG               => 8,
            DnsRecordType::MR               => 9,
            DnsRecordType::NULL             => 10,
            DnsRecordType::WKS              => 11,
            DnsRecordType::PTR              => 12,
            DnsRecordType::HINFO            => 13,
            DnsRecordType::MINFO            => 14,
            DnsRecordType::MX               => 15,
            DnsRecordType::TXT              => 16,
            DnsRecordType::AAAA             => 28,
            DnsRecordType::OPT              => 41,
            DnsRecordType::AXFR             => 252,
            DnsRecordType::MAILB            => 253,
            DnsRecordType::MAILA            => 254,
            DnsRecordType::ALL              => 255,
            DnsRecordType::Custom (data)    => data
        }
    }

    pub fn from_u16 (data: u16) -> Self {
        match data {
            1       => DnsRecordType::A,
            2       => DnsRecordType::NS,
            3       => DnsRecordType::MD,
            4       => DnsRecordType::MF,
            5       => DnsRecordType::CNAME,
            6       => DnsRecordType::SOA,
            7       => DnsRecordType::MB,
            8       => DnsRecordType::MG,
            9       => DnsRecordType::MR,
            10      => DnsRecordType::NULL,
            11      => DnsRecordType::WKS,
            12      => DnsRecordType::PTR,
            13      => DnsRecordType::HINFO,
            14      => DnsRecordType::MINFO,
            15      => DnsRecordType::MX,
            16      => DnsRecordType::TXT,
            28      => DnsRecordType::AAAA,
            41      => DnsRecordType::OPT,
            252     => DnsRecordType::AXFR,
            253     => DnsRecordType::MAILB,
            254     => DnsRecordType::MAILA,
            255     => DnsRecordType::ALL,
            _       => DnsRecordType::Custom (data)
        }
    }
}