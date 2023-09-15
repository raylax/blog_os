#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use blog_os::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    #[cfg(not(test))]
    main();
    #[cfg(test)]
    test_main();
    blog_os::hlt_loop();
}

pub fn main() {
    blog_os::init();
    println!("Welcome BlogOS ~");
}

#[cfg(not(test))]
#[panic_handler]
pub fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    println!("{info}");
    blog_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
pub fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    blog_os::handle_test_panic(info)
}
