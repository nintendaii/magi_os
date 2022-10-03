#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(magi_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    magi_os::test_panic_handler(info)
}

use magi_os::println;

#[test_case]
fn test_println() {
    println!("test_println output");
}

#[test_case]
fn test_panic() {
    panic!("This should panic");
}