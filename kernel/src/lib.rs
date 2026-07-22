
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
    vga::WRITER.lock().clear_screen();
    for row in 1..28  {
        if row % 2 == 0 {
            vga::WRITER.lock().set_color(vga::Color::Red,  vga::Color::Yellow);
        }
        else {
            vga::WRITER.lock().set_color(vga::Color::Cyan,  vga::Color::DarkGray);
        }
        println!("Screen Column n-{}", row);
    }
    print!("Hello {}", 42);
    loop {}
}