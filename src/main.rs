#![no_std]
#![no_main]
#![reexport_test_harness_main = "test_main"]
#![test_runner(magi_os::test_runner)]
#![feature(custom_test_frameworks)]

mod logo;
mod vga_buffer;
mod serial;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    logo::show();
    //optional color setting
    vga_buffer::WRITER.lock().color_code = vga_buffer::ColorCode::new(vga_buffer::Color::White, vga_buffer::Color::Black);
    magi_os::init();
    
    #[cfg(test)]
    test_main();
    println!("It did not crash!");
    magi_os::hlt_loop();
}


use core::panic::PanicInfo;

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    magi_os::hlt_loop();
}
// our panic handler in test mode
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    magi_os::test_panic_handler(info)
}
