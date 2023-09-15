use lazy_static::lazy_static;
use x86_64::structures::gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector};
use x86_64::structures::tss::TaskStateSegment;
use x86_64::VirtAddr;

/// 双故障中断表索引
pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;
const STACK_SIZE: usize = 4096 * 5; // 20kib

#[allow(clippy::upper_case_acronyms)]
struct GDT {
    gdt: GlobalDescriptorTable,
    selectors: Selectors,
}

struct Selectors {
    code: SegmentSelector,
    tss: SegmentSelector,
}

lazy_static! {
    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();

        // 设置双故障中断栈
        // 由于x86架构中，栈是从高地址向低地址增长的，所以将结束地址设置为栈的起始地址
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

            let stack_start = VirtAddr::from_ptr(unsafe { &STACK });
            stack_start + STACK_SIZE
        };
        tss
    };
}

lazy_static! {

    static ref G: GDT = {
        let mut gdt = GlobalDescriptorTable::new();
        // 添加内核代码段
        let code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
        // 添加TSS
        let tss_selector = gdt.add_entry(Descriptor::tss_segment(&TSS));
        GDT {
            gdt,
            selectors: Selectors {
                code: code_selector,
                tss: tss_selector,
            },
        }
    };
}

pub fn init() {
    use x86_64::instructions::segmentation::{Segment, CS};
    use x86_64::instructions::tables::load_tss;

    G.gdt.load();
    unsafe {
        CS::set_reg(G.selectors.code);
        load_tss(G.selectors.tss);
    }
}
