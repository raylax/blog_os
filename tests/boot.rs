#![no_std]
#![no_main]

use blog_os::{exit_qemu, serial_println, QemuExitCode};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    serial_println!("boot... [ok]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}

#[panic_handler]
pub fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    serial_println!("[failed]");
    exit_qemu(QemuExitCode::Failed);
    loop {}
}
