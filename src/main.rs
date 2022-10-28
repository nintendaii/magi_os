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
    use x86_64::{structures::paging::Page, VirtAddr};
    use magi_os::memory::BootInfoFrameAllocator;


    logo::show();
    //optional color setting
    vga_buffer::WRITER.lock().color_code = vga_buffer::ColorCode::new(vga_buffer::Color::White, vga_buffer::Color::Black);
    magi_os::init();

    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };


    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = memory::EmptyFrameAllocator;

    // map an unused page
    let page = Page::containing_address(VirtAddr::new(0));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    // write the string `New!` to the screen through the new mapping
    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e)};


    
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
