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
    #[allow(clippy::empty_loop)]
    loop {}
}

pub fn main() {
    blog_os::init();
    println!("Welcome BlogOS ~");
    loop {}
}

#[cfg(not(test))]
#[panic_handler]
pub fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    println!("{info}");
    loop {}
}

#[cfg(test)]
#[panic_handler]
pub fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    blog_os::handle_test_panic(info)
}
