extern crate alloc;

use alloc::format;
use core::cmp::min;

use corelib::AlienResult;
use interface::DomainType;
use pconst::{
    io::PollEvents,
    net::{Domain, SocketType},
};
use shared_heap::DVec;

use crate::{domain_helper, error::AlienError};

#[inline]
fn mark_pass(prefix: &str, name: &str) {
    platform::println!("[{}] pass: {}", prefix, name);
}

#[inline]
fn mark_skip(prefix: &str, name: &str, reason: &str) {
    platform::println!("[{}] skip: {} ({})", prefix, name, reason);
}

#[inline]
fn mark_fail(prefix: &str, name: &str, reason: &str) {
    platform::println!("[{}] fail: {} ({})", prefix, name, reason);
}

#[inline]
fn mark_step(prefix: &str, step: &str, detail: &str) {
    platform::println!("[{}] step: {} ({})", prefix, step, detail);
}

fn expect_domain(prefix: &str, name: &str) -> AlienResult<DomainType> {
    domain_helper::query_domain(name).ok_or_else(|| {
        mark_fail(prefix, name, "domain not found");
        AlienError::EINVAL
    })
}

fn find_domain<'a>(candidates: &'a [&'a str]) -> Option<(&'a str, DomainType)> {
    for &name in candidates {
        if let Some(domain) = domain_helper::query_domain(name) {
            return Some((name, domain));
        }
    }
    None
}

fn is_nonfatal_probe_err(err: AlienError) -> bool {
    matches!(err, AlienError::ENOSYS | AlienError::EINVAL | AlienError::EBLOCKING)
}

#[inline]
fn is_empty_domain_id(domain_id: u64) -> bool {
    domain_id == u64::MAX
}

#[cfg(feature = "domain_syscall_test")]
fn test_syscall() -> AlienResult<()> {
    let prefix = "domain_syscall_test";
    mark_step(prefix, "resolve", "query syscall domain");
    let syscall = match expect_domain(prefix, "syscall")? {
        DomainType::SysCallDomain(syscall) => syscall,
        _ => {
            mark_fail(prefix, "syscall", "domain type mismatch");
            return Err(AlienError::EINVAL);
        }
    };

    let domain_id = syscall.domain_id();
    if domain_id == 0 {
        mark_fail(prefix, "syscall", "invalid domain id");
        return Err(AlienError::EINVAL);
    }
    mark_step(
        prefix,
        "probe",
        &format!("syscall domain_id={}", domain_id),
    );

    let ret = syscall.call(2003, [0; 6])?;
    mark_step(prefix, "probe", &format!("call#1 ret={}", ret));
    if ret != 0 {
        mark_fail(prefix, "syscall", "unexpected return value(call#1)");
        return Err(AlienError::EINVAL);
    }

    let ret2 = syscall.call(2003, [1, 2, 3, 4, 5, 6])?;
    mark_step(prefix, "probe", &format!("call#2 ret={}", ret2));
    if ret2 != 0 {
        mark_fail(prefix, "syscall", "unexpected return value(call#2)");
        return Err(AlienError::EINVAL);
    }

    mark_pass(prefix, "syscall");
    Ok(())
}

#[cfg(feature = "domain_task_test")]
fn test_task() -> AlienResult<()> {
    let prefix = "domain_task_test";
    mark_step(prefix, "resolve", "query task domain");
    let task = match expect_domain(prefix, "task")? {
        DomainType::TaskDomain(task) => task,
        _ => {
            mark_fail(prefix, "task", "domain type mismatch");
            return Err(AlienError::EINVAL);
        }
    };

    let domain_id_1 = task.domain_id();
    let domain_id_2 = task.domain_id();
    if domain_id_1 == 0 || domain_id_1 != domain_id_2 {
        mark_fail(prefix, "task", "domain id check failed");
        return Err(AlienError::EINVAL);
    }
    mark_step(
        prefix,
        "probe",
        &format!("task domain_id={}", domain_id_1),
    );

    match task.page_table_token_with_trap_frame_virt_addr() {
        Ok((token, trap_frame_va)) => {
            if token == 0 || trap_frame_va == 0 {
                mark_fail(prefix, "task", "invalid page table token/trap frame va");
                return Err(AlienError::EINVAL);
            }
            mark_step(
                prefix,
                "probe",
                &format!("token={:#x}, trap_frame_va={:#x}", token, trap_frame_va),
            );
        }
        Err(AlienError::EINVAL) => {
            mark_step(prefix, "probe", "task not initialized yet, skip token check");
        }
        Err(err) => {
            mark_fail(prefix, "task", "page_table_token probe failed");
            return Err(err);
        }
    }

    match task.trap_frame_phy_addr() {
        Ok(trap_frame_pa) => {
            if trap_frame_pa == 0 {
                mark_fail(prefix, "task", "invalid trap frame pa");
                return Err(AlienError::EINVAL);
            }
            mark_step(
                prefix,
                "probe",
                &format!("trap_frame_pa={:#x}", trap_frame_pa),
            );
        }
        Err(AlienError::EINVAL) => {
            mark_step(prefix, "probe", "task not initialized yet, skip trap frame pa check");
        }
        Err(err) => {
            mark_fail(prefix, "task", "trap_frame_pa probe failed");
            return Err(err);
        }
    }

    mark_pass(prefix, "task");
    Ok(())
}

#[cfg(feature = "domain_apic_test")]
fn test_apic() -> AlienResult<()> {
    let prefix = "domain_apic_test";
    mark_step(prefix, "resolve", "query apic domain");
    let apic = match expect_domain(prefix, "apic")? {
        DomainType::APICDomain(apic) => apic,
        _ => {
            mark_fail(prefix, "apic", "domain type mismatch");
            return Err(AlienError::EINVAL);
        }
    };

    let domain_id = apic.domain_id();
    if domain_id == 0 {
        mark_fail(prefix, "apic", "invalid domain id");
        return Err(AlienError::EINVAL);
    }
    mark_step(
        prefix,
        "probe",
        &format!("apic domain_id={}", domain_id),
    );

    let info = apic.irq_info(DVec::new(0, 512))?;
    if info.is_empty() {
        mark_fail(prefix, "apic", "irq_info is empty");
        return Err(AlienError::EINVAL);
    }
    let preview_len = min(32, info.len());
    let preview = &info.as_slice()[..preview_len];
    mark_step(
        prefix,
        "probe",
        &format!("irq_info_len={}, preview={:?}", info.len(), preview),
    );

    mark_pass(prefix, "apic");
    Ok(())
}

#[cfg(feature = "domain_uart_test")]
fn test_uart() -> AlienResult<()> {
    let prefix = "domain_uart_test";
    mark_step(prefix, "resolve", "query buf_uart domain");
    let uart = match expect_domain(prefix, "buf_uart")? {
        DomainType::BufUartDomain(uart) => uart,
        _ => {
            mark_fail(prefix, "uart", "domain type mismatch");
            return Err(AlienError::EINVAL);
        }
    };

    let domain_id = uart.domain_id();
    if domain_id == 0 {
        mark_fail(prefix, "uart", "invalid domain id");
        return Err(AlienError::EINVAL);
    }
    mark_step(
        prefix,
        "probe",
        &format!("uart domain_id={}", domain_id),
    );

    let has_space = uart.have_space_to_put()?;
    mark_step(prefix, "probe", &format!("have_space_to_put={}", has_space));
    if !has_space {
        mark_fail(prefix, "uart", "no space to put");
        return Err(AlienError::EINVAL);
    }

    uart.putc(b'\n')?;
    mark_step(prefix, "probe", "putc success");

    let wrote = uart.put_bytes(&DVec::from_slice(b"[domain_uart_test] uart smoke\n"))?;
    mark_step(prefix, "probe", &format!("put_bytes wrote={}", wrote));
    if wrote == 0 {
        mark_fail(prefix, "uart", "no bytes written");
        return Err(AlienError::EINVAL);
    }

    let has_data = uart.have_data_to_get()?;
    mark_step(prefix, "probe", &format!("have_data_to_get={}", has_data));

    if has_data {
        match uart.getc() {
            Ok(Some(ch)) => mark_step(prefix, "probe", &format!("getc got={:#x}", ch)),
            Ok(None) => mark_step(prefix, "probe", "getc returned none"),
            Err(err) => mark_step(prefix, "probe", &format!("getc err={:?}", err)),
        }
    } else {
        mark_step(prefix, "probe", "skip getc because no input data");
    }

    mark_pass(prefix, "uart");
    Ok(())
}

#[cfg(feature = "domain_block_test")]
fn test_block() -> AlienResult<()> {
    let prefix = "domain_block_test";
    mark_step(prefix, "resolve", "query block domain");
    let block_name = if domain_helper::query_domain("block-1").is_some() {
        "block-1"
    } else if domain_helper::query_domain("block").is_some() {
        "block"
    } else {
        mark_skip(prefix, "block", "domain not found");
        return Ok(());
    };

    let block = match expect_domain(prefix, block_name)? {
        DomainType::BlkDeviceDomain(block) => block,
        _ => {
            mark_fail(prefix, "block", "domain type mismatch");
            return Err(AlienError::EINVAL);
        }
    };

    let domain_id = block.domain_id();
    if domain_id == 0 {
        mark_fail(prefix, "block", "invalid domain id");
        return Err(AlienError::EINVAL);
    }
    mark_step(
        prefix,
        "probe",
        &format!("block domain_id={}, name={}", domain_id, block_name),
    );

    let mut probe_ok = 0usize;

    match block.get_capacity() {
        Ok(capacity) => {
            mark_step(prefix, "probe", &format!("capacity={} bytes", capacity));
            if capacity == 0 {
                mark_fail(prefix, "block", "capacity is zero");
                return Err(AlienError::EINVAL);
            }
            probe_ok += 1;
        }
        Err(err) => {
            platform::println!(
                "[domain_block_test] note: block capacity check skipped ({:?})",
                err
            );
        }
    }

    match block.read_block(0, DVec::new(0u8, 512)) {
        Ok(buf) => {
            let checksum = buf
                .as_slice()
                .iter()
                .fold(0u32, |acc, byte| acc.wrapping_add(*byte as u32));
            mark_step(
                prefix,
                "probe",
                &format!("read_block#0 len={}, checksum={}", buf.len(), checksum),
            );
            probe_ok += 1;
        }
        Err(err) => {
            platform::println!("[domain_block_test] note: read_block skipped ({:?})", err);
        }
    }

    match block.flush() {
        Ok(()) => {
            mark_step(prefix, "probe", "flush success");
            probe_ok += 1;
        }
        Err(err) => {
            platform::println!("[domain_block_test] note: flush skipped ({:?})", err);
        }
    }

    if probe_ok == 0 {
        mark_fail(prefix, "block", "no block probe succeeded");
        return Err(AlienError::EINVAL);
    }

    mark_pass(prefix, "block");
    Ok(())
}

#[cfg(feature = "domain_net_test")]
fn test_net() -> AlienResult<()> {
    let prefix = "domain_net_test";
    mark_step(prefix, "resolve", "query net_stack domain");
    let net = match expect_domain(prefix, "net_stack")? {
        DomainType::NetDomain(net) => net,
        _ => {
            mark_fail(prefix, "net_stack", "domain type mismatch");
            return Err(AlienError::EINVAL);
        }
    };

    let net_domain_id = net.domain_id();
    if net_domain_id == 0 {
        mark_fail(prefix, "net_stack", "invalid domain id");
        return Err(AlienError::EINVAL);
    }
    mark_step(
        prefix,
        "probe",
        &format!("net_stack domain_id={}", net_domain_id),
    );

    let (sock_tx, sock_rx) = net.socket_pair(Domain::AF_UNIX, SocketType::SOCK_STREAM)?;
    mark_step(
        prefix,
        "probe",
        &format!("socket_pair tx={}, rx={}", sock_tx, sock_rx),
    );

    let poll_out = net.poll(sock_tx, PollEvents::EPOLLOUT)?;
    mark_step(prefix, "probe", &format!("poll_out={:?}", poll_out));
    if !poll_out.contains(PollEvents::EPOLLOUT) {
        mark_fail(prefix, "net_stack", "socket not writable");
        return Err(AlienError::EINVAL);
    }

    let payload = DVec::from_slice(b"domain_net_test_payload");
    let wrote = net.write_at(sock_tx, 0, &payload)?;
    mark_step(prefix, "probe", &format!("write_at wrote={}", wrote));
    if wrote != payload.len() {
        mark_fail(prefix, "net_stack", "write length mismatch");
        return Err(AlienError::EINVAL);
    }

    let poll_in = net.poll(sock_rx, PollEvents::EPOLLIN)?;
    mark_step(prefix, "probe", &format!("poll_in={:?}", poll_in));
    if !poll_in.contains(PollEvents::EPOLLIN) {
        mark_fail(prefix, "net_stack", "socket not readable after write");
        return Err(AlienError::EINVAL);
    }

    let (read_buf, read_len) = net.read_at(sock_rx, 0, DVec::new(0u8, payload.len()))?;
    mark_step(prefix, "probe", &format!("read_at len={}", read_len));
    if read_len != payload.len() {
        mark_fail(prefix, "net_stack", "read length mismatch");
        return Err(AlienError::EINVAL);
    }
    if &read_buf.as_slice()[..read_len] != payload.as_slice() {
        mark_fail(prefix, "net_stack", "payload mismatch");
        return Err(AlienError::EINVAL);
    }

    let poll_after_read = net.poll(sock_rx, PollEvents::EPOLLIN)?;
    mark_step(
        prefix,
        "probe",
        &format!("poll_after_read={:?}", poll_after_read),
    );

    net.remove_socket(sock_tx)?;
    mark_step(prefix, "probe", "remove_socket success");

    let nic_candidates = ["nic-1", "nic", "virtio_net-1", "virtio_net"];
    if let Some((nic_name, nic_domain)) = find_domain(&nic_candidates) {
        let nic = match nic_domain {
            DomainType::NetDeviceDomain(nic) => nic,
            _ => {
                mark_fail(prefix, nic_name, "net device domain type mismatch");
                return Err(AlienError::EINVAL);
            }
        };

        let nic_domain_id = nic.domain_id();
        if nic_domain_id == 0 {
            mark_fail(prefix, nic_name, "invalid domain id");
            return Err(AlienError::EINVAL);
        }
        mark_step(
            prefix,
            "probe",
            &format!("nic={} domain_id={}", nic_name, nic_domain_id),
        );

        if is_empty_domain_id(nic_domain_id) {
            mark_skip(
                prefix,
                nic_name,
                "empty-domain proxy detected, skip strict nic io probes",
            );
        } else {
            let mut nic_probe_ok = 0usize;

            match nic.mac_address() {
                Ok(mac) => {
                    mark_step(prefix, "probe", &format!("nic mac={:02x?}", mac));
                    nic_probe_ok += 1;
                }
                Err(err) if is_nonfatal_probe_err(err) => {
                    mark_step(prefix, "probe", &format!("nic mac skipped err={:?}", err));
                }
                Err(err) => {
                    mark_fail(prefix, nic_name, "mac probe failed");
                    return Err(err);
                }
            }

            match nic.tx_queue_size() {
                Ok(size) => {
                    mark_step(prefix, "probe", &format!("nic tx_queue_size={}", size));
                    if size > 0 {
                        nic_probe_ok += 1;
                    }
                }
                Err(err) if is_nonfatal_probe_err(err) => {
                    mark_step(prefix, "probe", &format!("tx_queue_size skipped err={:?}", err));
                }
                Err(err) => {
                    mark_fail(prefix, nic_name, "tx_queue_size probe failed");
                    return Err(err);
                }
            }

            match nic.rx_queue_size() {
                Ok(size) => {
                    mark_step(prefix, "probe", &format!("nic rx_queue_size={}", size));
                    if size > 0 {
                        nic_probe_ok += 1;
                    }
                }
                Err(err) if is_nonfatal_probe_err(err) => {
                    mark_step(prefix, "probe", &format!("rx_queue_size skipped err={:?}", err));
                }
                Err(err) => {
                    mark_fail(prefix, nic_name, "rx_queue_size probe failed");
                    return Err(err);
                }
            }

            let can_transmit = match nic.can_transmit() {
                Ok(can) => {
                    mark_step(prefix, "probe", &format!("nic can_transmit={}", can));
                    nic_probe_ok += 1;
                    can
                }
                Err(err) => {
                    mark_fail(prefix, nic_name, "can_transmit probe failed on non-empty nic");
                    return Err(err);
                }
            };
            if !can_transmit {
                mark_fail(prefix, nic_name, "can_transmit=false on non-empty nic");
                return Err(AlienError::EINVAL);
            }

            let can_receive = match nic.can_receive() {
                Ok(can) => {
                    mark_step(prefix, "probe", &format!("nic can_receive={}", can));
                    nic_probe_ok += 1;
                    can
                }
                Err(err) => {
                    mark_fail(prefix, nic_name, "can_receive probe failed on non-empty nic");
                    return Err(err);
                }
            };

            if nic_probe_ok == 0 {
                mark_fail(prefix, nic_name, "no nic probe succeeded on non-empty nic");
                return Err(AlienError::EINVAL);
            }
            mark_step(
                prefix,
                "probe",
                &format!("nic probe success count={}", nic_probe_ok),
            );

            let mut tx_frame = [0u8; 64];
            tx_frame[0..6].copy_from_slice(&[0xff; 6]);
            tx_frame[6..12].copy_from_slice(&[0x52, 0x54, 0x00, 0x12, 0x34, 0x56]);
            tx_frame[12..14].copy_from_slice(&[0x08, 0x00]);
            for (index, byte) in tx_frame[14..].iter_mut().enumerate() {
                *byte = (index as u8).wrapping_mul(3).wrapping_add(1);
            }

            if let Err(err) = nic.transmit(&DVec::from_slice(&tx_frame)) {
                mark_fail(prefix, nic_name, "nic transmit failed on non-empty nic");
                return Err(err);
            }
            mark_step(
                prefix,
                "probe",
                &format!("nic transmit ok len={}", tx_frame.len()),
            );

            if can_receive {
                let rx_capacity = 2048usize;
                let (rx_buf, rx_len) = match nic.receive(DVec::new(0u8, rx_capacity)) {
                    Ok(ret) => ret,
                    Err(err) => {
                        mark_fail(prefix, nic_name, "nic receive failed with can_receive=true");
                        return Err(err);
                    }
                };
                mark_step(prefix, "probe", &format!("nic receive len={}", rx_len));
                if rx_len == 0 || rx_len > rx_buf.len() {
                    mark_fail(prefix, nic_name, "invalid receive length on non-empty nic");
                    return Err(AlienError::EINVAL);
                }
            } else {
                mark_step(prefix, "probe", "skip strict receive because can_receive=false");
            }
        }
    } else {
        mark_skip(prefix, "nic", "domain not found");
    }

    mark_pass(prefix, "net");
    Ok(())
}

pub fn run() -> AlienResult<()> {
    platform::println!("[domain_test] begin");
    #[cfg(feature = "domain_syscall_test")]
    test_syscall()?;
    #[cfg(feature = "domain_task_test")]
    test_task()?;
    #[cfg(feature = "domain_apic_test")]
    test_apic()?;
    #[cfg(feature = "domain_uart_test")]
    test_uart()?;
    #[cfg(feature = "domain_block_test")]
    test_block()?;
    #[cfg(feature = "domain_net_test")]
    test_net()?;
    platform::println!("[domain_test] done");
    Ok(())
}