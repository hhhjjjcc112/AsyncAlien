use core::fmt::{Debug, Formatter};
use core::{mem::size_of, net::Ipv4Addr};

use crate::{
    consts::{EthRtype, IpProtocal},
    MacAddress,
};

#[derive(Debug)]
#[repr(C, packed)]
pub struct Eth {
    pub(crate) dhost: MacAddress, // destination host
    pub(crate) shost: MacAddress, // source host
    pub(crate) rtype: EthRtype,   // packet type, arp or ip
}

#[repr(C, packed)]
#[derive(Debug, Clone)]
pub struct Arp {
    pub(crate) httype: u16,      // Hardware type
    pub(crate) pttype: EthRtype, // Protocol type, For IPv4, this has the value 0x0800.
    pub(crate) hlen: u8,         // Hardware length: Ethernet address length is 6.
    pub(crate) plen: u8,         // Protocol length: IPv4 address length is 4.
    pub(crate) op: u16,          // Operation: 1 for request, 2 for reply.
    pub(crate) sha: MacAddress,  // Sender hardware address
    pub(crate) spa: Ipv4Addr,    // Sender protocol address
    pub(crate) tha: MacAddress,  // Target hardware address
    pub(crate) tpa: Ipv4Addr,    // Target protocol address
}

#[allow(dead_code)]
#[repr(C, packed)]
/// See https://github.com/rust-lang/rust-clippy/issues/13375
///
/// 在Rust中，#[repr(packed)]默认情况下是#[repr(Rust, packed)]，而不是#[repr(C, packed)]。这意味着：
/// 1. #[repr(packed)]的作用：
///     - 它会取消结构体字段之间的所有填充字节（padding），使字段紧密排列。
///     - 但它不会改变Rust的默认字段排序规则（即repr(Rust)的规则）。
/// 2. Rust的默认字段排序规则（repr(Rust)）：
///     - Rust编译器可能会对字段进行重排序以优化内存布局（例如，将大小相近的字段放在一起以减少填充字节）。
///     - 这种重排序是未指定的（unspecified），因此不能依赖具体的字段顺序。
/// 3. #[repr(C, packed)]的作用：
///     - 如果你希望同时满足C语言的内存布局规则（字段顺序与定义顺序一致）并且取消填充字节，可以显式地使用#[repr(C, packed)]。
///     - 这会强制字段按照定义顺序排列，并且紧密排列
#[derive(Debug, Clone)]
pub struct Ip {
    pub(crate) vhl: u8,         // version << 4 | header length >> 2
    pub(crate) tos: u8,         // type of service
    pub(crate) len: u16,        // total length, packet length
    pub(crate) id: u16,         // identification, can combine all packets
    pub(crate) off: u16,        // fragment offset field, packet from
    pub(crate) ttl: u8,         // time to live
    pub(crate) pro: IpProtocal, // protocol， ICMP(1)、IGMP(2)、TCP(6)、UDP(17)
    pub(crate) sum: u16,        // checksum,
    pub(crate) src: Ipv4Addr,   // souce ip
    pub(crate) dst: Ipv4Addr,   // destination ip
}

#[allow(dead_code)]
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct UDP {
    pub(crate) sport: u16, // souce port
    pub(crate) dport: u16, // destination port
    pub(crate) ulen: u16,  // length, including udp header, not including IP header
    pub(crate) sum: u16,   // checksum
}

impl Debug for UDP {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("UDP")
            .field("sport", &self.sport.to_be())
            .field("dport", &self.dport.to_be())
            .field("ulen", &self.ulen.to_be())
            .field("sum", &self.sum.to_be())
            .finish()
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct TcpFlags: u8 {
        const NONE = 0;
        const F = 0b00000001;
        const S = 0b00000010;
        const R = 0b00000100;
        const P = 0b00001000;
        const A = 0b00010000;
        const U = 0b00100000;
    }
}

#[allow(dead_code)]
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TCP {
    pub(crate) sport: u16,      // souce port
    pub(crate) dport: u16,      // destination port
    pub(crate) seq: u32,        // sequence number
    pub(crate) ack: u32,        // acknowledgement number
    pub(crate) offset: u8,      // offset, first 4 bytes are tcp header length
    pub(crate) flags: TcpFlags, // flags, last 6 are flags(U, A, P, R, S, F)
    pub(crate) win: u16,        // window size
    pub(crate) sum: u16,        // checksum
    pub(crate) urg: u16,        // urgent pointer
}

#[allow(dead_code)]
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct ICMP {
    pub(crate) type_: u8,
    pub(crate) code: u8,
    pub(crate) checksum: u16,
    pub(crate) id: u16,
    pub(crate) seq: u16,
}

pub(crate) const ETH_LEN: usize = size_of::<Eth>();
pub(crate) const ARP_LEN: usize = size_of::<Arp>();
pub(crate) const IP_LEN: usize = size_of::<Ip>();
pub(crate) const UDP_LEN: usize = size_of::<UDP>();
pub(crate) const TCP_LEN: usize = size_of::<TCP>();
