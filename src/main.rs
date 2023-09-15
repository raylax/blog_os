#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use blog_os::println;
use bootloader::{entry_point, BootInfo};
use x86_64::structures::paging::Translate;
use x86_64::VirtAddr;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    #[cfg(not(test))]
    main(boot_info);
    #[cfg(test)]
    test_main();
    blog_os::hlt_loop();
}

pub fn main(boot_info: &'static BootInfo) {
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
