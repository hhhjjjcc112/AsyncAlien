use crate::MacAddress;

// mac address
pub(crate) const BROADCAST_MAC: MacAddress = MacAddress::new([0xff, 0xff, 0xff, 0xff, 0xff, 0xff]);
// arp
pub(crate) const ARP_HRD_ETHER: u16 = 1;
pub(crate) const ARP_ETHADDR_LEN: usize = 6;
pub(crate) const ARP_OP_REQUEST: u16 = 1;
pub(crate) const ARP_OP_REPLY: u16 = 2;

pub(crate) const IP_HEADER_VHL: u8 = 4 << 4 | 20 >> 2;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u16)]
pub enum EthRtype {
    IP = 0x0008,
    ARP = 0x0608,
    Unknown,
}
impl Default for EthRtype {
    fn default() -> Self {
        EthRtype::Unknown
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub enum IpProtocal {
    ICMP = 1,
    IGMP = 2,
    TCP = 6,
    UDP = 17,
    Unknown,
}

impl Default for IpProtocal {
    fn default() -> Self {
        IpProtocal::Unknown
    }
}
