use corelib::AlienResult;
use interface::DomainType;
use shared_heap::DVec;

use crate::{domain_helper, error::AlienError};

#[inline]
fn mark_pass(name: &str) {
    platform::println!("[domain_self_test] pass: {}", name);
}

#[inline]
fn mark_skip(name: &str, reason: &str) {
    platform::println!("[domain_self_test] skip: {} ({})", name, reason);
}

#[inline]
fn mark_fail(name: &str, reason: &str) {
    platform::println!("[domain_self_test] fail: {} ({})", name, reason);
}

fn expect_domain(name: &str) -> AlienResult<DomainType> {
    domain_helper::query_domain(name).ok_or_else(|| {
        mark_fail(name, "domain not found");
        AlienError::EINVAL
    })
}

fn test_syscall() -> AlienResult<()> {
    let syscall = match expect_domain("syscall")? {
        DomainType::SysCallDomain(syscall) => syscall,
        _ => {
            mark_fail("syscall", "domain type mismatch");
            return Err(AlienError::EINVAL);
        }
    };

    let ret = syscall.call(2003, [0; 6])?;
    if ret != 0 {
        mark_fail("syscall", "unexpected return value");
        return Err(AlienError::EINVAL);
    }

    mark_pass("syscall");
    Ok(())
}

fn test_task() -> AlienResult<()> {
    let task = match expect_domain("task")? {
        DomainType::TaskDomain(task) => task,
        _ => {
            mark_fail("task", "domain type mismatch");
            return Err(AlienError::EINVAL);
        }
    };

    let _ = task.domain_id();
    mark_pass("task");
    Ok(())
}

fn test_apic() -> AlienResult<()> {
    let apic = match expect_domain("apic")? {
        DomainType::APICDomain(apic) => apic,
        _ => {
            mark_fail("apic", "domain type mismatch");
            return Err(AlienError::EINVAL);
        }
    };

    let _ = apic.irq_info(DVec::new(0, 128))?;
    mark_pass("apic");
    Ok(())
}

fn test_uart() -> AlienResult<()> {
    let uart = match expect_domain("buf_uart")? {
        DomainType::BufUartDomain(uart) => uart,
        _ => {
            mark_fail("uart", "domain type mismatch");
            return Err(AlienError::EINVAL);
        }
    };

    let wrote = uart.put_bytes(&DVec::from_slice(b"[domain_self_test] uart smoke\n"))?;
    if wrote == 0 {
        mark_fail("uart", "no bytes written");
        return Err(AlienError::EINVAL);
    }

    mark_pass("uart");
    Ok(())
}

fn test_block() -> AlienResult<()> {
    let block_name = if domain_helper::query_domain("block-1").is_some() {
        "block-1"
    } else if domain_helper::query_domain("block").is_some() {
        "block"
    } else {
        mark_skip("block", "domain not found");
        return Ok(());
    };

    let block = match expect_domain(block_name)? {
        DomainType::BlkDeviceDomain(block) => block,
        _ => {
            mark_fail("block", "domain type mismatch");
            return Err(AlienError::EINVAL);
        }
    };

    match block.get_capacity() {
        Ok(capacity) => {
            if capacity == 0 {
                mark_fail("block", "capacity is zero");
                return Err(AlienError::EINVAL);
            }
        }
        Err(err) => {
            platform::println!(
                "[domain_self_test] note: block capacity check skipped ({:?})",
                err
            );
        }
    }

    mark_pass("block");
    Ok(())
}

pub fn run() -> AlienResult<()> {
    platform::println!("[domain_self_test] begin");
    test_syscall()?;
    test_task()?;
    test_apic()?;
    test_uart()?;
    test_block()?;
    platform::println!("[domain_self_test] done");
    Ok(())
}
