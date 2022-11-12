#![no_std]
#![no_main]

use core::intrinsics::offset;
use core::panic::PanicInfo;

/// Don't mangle the entry point, as we want this to remain `_start`.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    #[allow(clippy::empty_loop)]
    loop {}
}

/// This is our handler for any panics that occur.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    #[allow(clippy::empty_loop)]
    loop {}
}
