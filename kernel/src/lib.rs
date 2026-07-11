
// No std library
#![no_std]
// Disable main init
#![no_main]

pub mod vga;
use core::panic::PanicInfo;

// Called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Entry point, since the linker look for the function _start 
// -> kernel_main
#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() -> ! {
    vga::print_hello_world("Hello les guys !");
    loop {}
}