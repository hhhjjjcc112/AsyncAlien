use crate::error;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    error!("Kernel Panic: {}", _info);
    loop {
        core::hint::spin_loop();
    }
}