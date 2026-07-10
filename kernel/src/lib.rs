
use crate::vga;
// No std library
#![no_std]
// Disable main init
#![no_main]
use core::panic::PanicInfo;

// Called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Entry point, since the linker look for the function _start
static HELLO: &[u8] = b"Hello World!";

#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() -> ! {



    loop {}
}