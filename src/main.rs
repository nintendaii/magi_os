#![no_std]
#![no_main]
#![reexport_test_harness_main = "test_main"]
#![test_runner(magi_os::test_runner)]
#![feature(custom_test_frameworks)]

mod logo;
mod vga_buffer;
mod serial;

use bootloader::{BootInfo, entry_point};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use magi_os::memory;
    use x86_64::{structures::paging::Translate, VirtAddr};

    logo::show();
    //optional color setting
    vga_buffer::WRITER.lock().color_code = vga_buffer::ColorCode::new(vga_buffer::Color::White, vga_buffer::Color::Black);
    magi_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mapper = unsafe { memory::init(phys_mem_offset) };

    let addresses = [
        // the identity-mapped vga buffer page
        0xb8000,
        // some code page
        0x201008,
        // some stack page
        0x0100_0020_1a10,
        // virtual address mapped to physical address 0
        boot_info.physical_memory_offset,
    ];

    for &address in &addresses {
        let virt = VirtAddr::new(address);
        let phys = mapper.translate_addr(virt);
        println!("{:?} -> {:?}", virt, phys);
    }


    
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
