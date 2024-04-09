#![no_std]
#![no_main]

#[panic_handler]
fn panic_handle(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
