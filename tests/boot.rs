#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    #[allow(clippy::empty_loop)]
    loop {}
}

#[panic_handler]
pub fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    blog_os::handle_test_panic(info)
}
