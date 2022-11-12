#![no_std]
#![no_main]

use core::panic::PanicInfo;

/// Don't mangle the entry point, as we want this to remain `_start`.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    #[allow(clippy::empty_loop)]
    loop {}
}

/// This is our handler for any panics that occur.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    #[allow(clippy::empty_loop)]
    loop {}
}
