use crate::gdt;
use crate::keyborad;
use crate::{print, println};
use lazy_static::lazy_static;
use pc_keyboard::DecodedKey;
use pic8259::ChainedPics;
use spin;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        // 设置断点异常的中断处理函数
        idt.breakpoint.set_handler_fn(breakpoint_handler);

        // 设置双重故障异常的中断处理函数
        unsafe {
            idt.double_fault.set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }

        // 时钟中断
        idt[InterruptIndex::Timer.into()].set_handler_fn(timer_interrupt_handler);
        // 键盘中断
        idt[InterruptIndex::Keyboard.into()].set_handler_fn(keyboard_interrupt_handler);

        idt
    };
}

/// 初始化中断描述符表
pub fn init_idt() {
    IDT.load();
}

/// 双重故障异常
extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}

/// 处理断点异常
extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

#[test_case]
fn test_breakpoint_exception() {
    // 触发断点异常
    x86_64::instructions::interrupts::int3();
}

// =========== PICs ===========

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    /// 时钟中断
    Timer = PIC_1_OFFSET,
    /// 键盘输入中断
    Keyboard,
}

impl From<InterruptIndex> for u8 {
    fn from(value: InterruptIndex) -> Self {
        value as u8
    }
}

impl From<InterruptIndex> for usize {
    fn from(value: InterruptIndex) -> Self {
        value as usize
    }
}

/// 时钟中断处理器
extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    notify_end_of_interrupt(InterruptIndex::Timer);
}

/// 时钟中断处理器
extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    use x86_64::instructions::port::Port;

    // PS2 port
    let mut port = Port::new(0x60);
    let sc: u8 = unsafe { port.read() };
    let mut kb = keyborad::KEYBOARD.lock();
    if let Ok(Some(key_event)) = kb.add_byte(sc) {
        if let Some(key) = kb.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode(character) => print!("{}", character),
                DecodedKey::RawKey(key) => print!("{:?}", key),
            }
        }
    }

    notify_end_of_interrupt(InterruptIndex::Keyboard);
}

fn notify_end_of_interrupt(ii: InterruptIndex) {
    unsafe { PICS.lock().notify_end_of_interrupt(ii.into()) }
}
