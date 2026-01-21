use crate::println;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    println!("Kernel panic: {}", _info);
    loop {
        core::hint::spin_loop();
    }
}