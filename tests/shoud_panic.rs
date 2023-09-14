#![no_std]
#![no_main]

use blog_os::{exit_qemu, serial_print, serial_println, QemuExitCode};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    serial_print!("should_panic... ");
    assert_eq!(1, 2);
    serial_println!("[failed]");
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

#[panic_handler]
pub fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}
