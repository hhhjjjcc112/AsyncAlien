#[cfg(target_arch = "riscv64")]
mod riscv64;
#[cfg(target_arch = "x86_64")]
mod x86_64;

use alloc::{collections::BTreeMap, string::ToString, vec};

use core2::io::Read;
use interface::DomainTypeRaw;
use log::warn;

#[cfg(target_arch = "riscv64")]
use self::riscv64::ARCH_INIT_DOMAIN_LIST;
#[cfg(target_arch = "x86_64")]
use self::x86_64::ARCH_INIT_DOMAIN_LIST;
use crate::{
    domain_loader::creator::register_domain_elf,
    error::{AlienError, AlienResult},
};

const COMMON_INIT_DOMAIN_LIST: &[(&str, DomainTypeRaw)] = &[
    ("buf_uart", DomainTypeRaw::BufUartDomain),
    ("buf_input", DomainTypeRaw::BufInputDomain),
    ("cache_blk", DomainTypeRaw::CacheBlkDeviceDomain),
    ("devfs", DomainTypeRaw::DevFsDomain),
    ("fatfs", DomainTypeRaw::FsDomain),
    ("null", DomainTypeRaw::EmptyDeviceDomain),
    ("pipefs", DomainTypeRaw::FsDomain),
    ("procfs", DomainTypeRaw::FsDomain),
    ("ramfs", DomainTypeRaw::FsDomain),
    ("random", DomainTypeRaw::EmptyDeviceDomain),
    ("shadow_blk", DomainTypeRaw::ShadowBlockDomain),
    ("syscall", DomainTypeRaw::SysCallDomain),
    ("sysfs", DomainTypeRaw::FsDomain),
    ("fifo_scheduler", DomainTypeRaw::SchedulerDomain),
    ("task", DomainTypeRaw::TaskDomain),
    ("vfs", DomainTypeRaw::VfsDomain),
    ("net_stack", DomainTypeRaw::NetDomain),
    ("logger", DomainTypeRaw::LogDomain),
    ("domainfs", DomainTypeRaw::FsDomain),
    #[cfg(feature = "bench")]
    ("mem_block", DomainTypeRaw::BlkDeviceDomain),
];

pub fn init_domains() -> AlienResult<()> {
    let initrd = mem::INITRD_DATA.lock();
    let data = initrd.as_ref().ok_or_else(|| {
        log::error!("Initrd data is not initialized");
        AlienError::EINVAL
    })?;
    let mut decoder = libflate::gzip::Decoder::new(data.as_slice()).map_err(|err| {
        log::error!("failed to decode initrd gzip: {:?}", err);
        AlienError::EINVAL
    })?;
    let mut buf = vec![];
    decoder.read_to_end(&mut buf).map_err(|err| {
        log::error!("failed to read initrd archive: {:?}", err);
        AlienError::EINVAL
    })?;

    let mut map = BTreeMap::new();
    for entry in cpio_reader::iter_files(&buf) {
        let _mode = entry.mode();
        let name = entry.name();
        if name.starts_with('g') {
            let data = entry.file();
            let domain_name = name.split_once('g').unwrap().1;
            map.insert(domain_name.to_string(), data.to_vec());
        }
    }

    let mut register = |domain_file_name: &str, domain: DomainTypeRaw| {
        if let Some(elf) = map.remove(domain_file_name) {
            register_domain_elf(domain_file_name, elf, domain);
        } else {
            warn!(
                "initrd missing domain elf: {}, skip pre-register",
                domain_file_name
            );
        }
    };

    for (domain_file_name, domain) in COMMON_INIT_DOMAIN_LIST {
        register(domain_file_name, *domain);
    }

    for (domain_file_name, domain) in ARCH_INIT_DOMAIN_LIST {
        register(domain_file_name, *domain);
    }

    Ok(())
}
