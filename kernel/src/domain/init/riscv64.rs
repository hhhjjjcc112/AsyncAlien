use interface::DomainTypeRaw;

pub(super) const ARCH_INIT_DOMAIN_LIST: &[(&str, DomainTypeRaw)] = &[
    ("goldfish", DomainTypeRaw::RtcDomain),
    ("plic", DomainTypeRaw::PLICDomain),
    #[cfg(all(target_arch = "riscv64", plat_qemu_riscv))]
    ("uart16550", DomainTypeRaw::UartDomain),
    #[cfg(all(target_arch = "riscv64", plat_qemu_riscv))]
    ("virtio_blk", DomainTypeRaw::BlkDeviceDomain),
    #[cfg(all(target_arch = "riscv64", plat_qemu_riscv))]
    ("virtio_net", DomainTypeRaw::NetDeviceDomain),
    #[cfg(all(target_arch = "riscv64", plat_vf2, not(plat_vf2_sd)))]
    ("mem_block", DomainTypeRaw::BlkDeviceDomain),
    #[cfg(all(target_arch = "riscv64", plat_vf2))]
    ("uart8250", DomainTypeRaw::UartDomain),
    #[cfg(all(target_arch = "riscv64", plat_vf2, plat_vf2_sd))]
    ("vf2_sd", DomainTypeRaw::BlkDeviceDomain),
];
