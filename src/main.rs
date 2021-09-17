#![feature(asm)]
#![no_std]
#![no_main]

mod vga_buffer;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("                         @@@@@@@&&@@@@@@@@*");
    println!("                     @@@@/,,,,,,,,,,****//@@@@");
    println!("                   @@@.....,,,,,,,,,*****///(@@@");
    println!("                 @@@,,,.....,,,,,*****///////((@@@");
    println!("                @@@//***,,....,,***@@@@@@/((((((&@@");
    println!("               @@@  (//**,,,,,,,@@@@///(@@@@(((((@@@");
    println!("               @@     #(/****,@@@*//(((((((@@#(((#@@");
    println!("              (@@      .(///**@@@//((((((((@@@####@@");
    println!("              @@@        (////@@@/((((((###@@####%@@");
    println!("             @@@          (////(@@@@&##@@@@@###%%@@@");
    println!("            @@@           (////(((##@@@@#####%%%@@@");
    println!("         .@@              (///(((##%@@#######@@@");
    println!("          @@@@@@            (//((##%@@#####@@@");
    println!("                (@@                *##%@@*    @@@");
    println!("                (@@                   %@@     @@@");
    println!("                 @@             (@@@@@@@      @@@");
    println!("                 ,@@@@@@@@@@@@@@@/            @@@");
    println!("                                               @");
    println!("");
    println!("");
    println!("");
    println!("               YOU ARE IN magi OS. THERE IS NO WAY OUT__");
    loop {}
}

use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
