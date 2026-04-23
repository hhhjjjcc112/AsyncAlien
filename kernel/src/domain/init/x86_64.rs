use interface::DomainTypeRaw;

pub(super) const ARCH_INIT_DOMAIN_LIST: &[(&str, DomainTypeRaw)] = &[
    ("uart16550", DomainTypeRaw::UartDomain),
    ("virtio_blk", DomainTypeRaw::BlkDeviceDomain),
    ("apic", DomainTypeRaw::APICDomain),
    ("cmos_rtc", DomainTypeRaw::RtcDomain),
    ("virtio_net", DomainTypeRaw::NetDeviceDomain),
    ("virtio_input", DomainTypeRaw::InputDomain),
    ("virtio_gpu", DomainTypeRaw::GpuDomain),
    ("loopback", DomainTypeRaw::NetDeviceDomain),
];
