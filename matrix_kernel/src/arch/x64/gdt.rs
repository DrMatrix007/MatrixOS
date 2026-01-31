use lazy_static::lazy_static;
use x86_64::{
    VirtAddr,
    instructions::tables::load_tss,
    registers::segmentation::{CS, SS, Segment},
    structures::{
        gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector},
        tss::TaskStateSegment,
    },
};

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

struct Selectors {
    pub(self) code_selector: SegmentSelector,
    pub(self) data_selector: SegmentSelector,
    pub(self) tss_selector: SegmentSelector,
}

const STACK_SIZE: usize = 4096 * 512;
static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

lazy_static! {
    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            let stack_start = VirtAddr::from_ptr(&raw const STACK);
            stack_start + STACK_SIZE as u64
        };
        tss
    };
    static ref GDT: (GlobalDescriptorTable, Selectors) = {
        let mut gdt = GlobalDescriptorTable::new();
        let code_selector = gdt.append(Descriptor::kernel_code_segment());
        let data_selector = gdt.append(Descriptor::kernel_data_segment());
        let tss_selector = gdt.append(Descriptor::tss_segment(&TSS));

        (
            gdt,
            Selectors {
                code_selector,
                data_selector,
                tss_selector,
            },
        )
    };
}

pub fn init() {
    GDT.0.load();
    unsafe {
        CS::set_reg(GDT.1.code_selector);
        SS::set_reg(GDT.1.data_selector);

        load_tss(GDT.1.tss_selector);
    }
}
