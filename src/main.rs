#![no_std]
#![no_main]

mod vga_buffer;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    for i in 1..=30 {
        println!("L{}", i);
    }
    loop {}
}
