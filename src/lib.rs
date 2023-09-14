#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

pub mod serial;
pub mod vga_buffer;

#[cfg(test)]
#[panic_handler]
pub fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    handle_test_panic(info)
}

pub fn handle_test_panic(info: &core::panic::PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

/// 测试入口
#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    #[allow(clippy::empty_loop)]
    loop {}
}

pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}

/// 测试特征，用于测试函数
pub trait Testable {
    fn run(&self);
}

/// 实现测试特征，T必须是Fn()类型
impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        // 打印测试函数名称
        serial_print!("{}...\t", core::any::type_name::<T>());
        // 调用测试函数
        self();
        // 打印结果
        serial_println!("[ok]");
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}
