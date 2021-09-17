#![feature(asm)]
#![no_std]
#![no_main]

mod logo;
mod vga_buffer;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    logo::show();
    loop {}
}

use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
