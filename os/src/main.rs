#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
extern "C" fn _start() -> ! {
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}