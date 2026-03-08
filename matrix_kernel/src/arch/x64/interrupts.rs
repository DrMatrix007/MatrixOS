use lazy_static::lazy_static;
use log::{error, info};
use x2apic::lapic::{LocalApicBuilder, TimerDivide};
use x86_64::{
    VirtAddr,
    structures::idt::{InterruptDescriptorTable, InterruptStackFrame},
};

use crate::{
    arch::x64::gdt, memory::apic_mapping::init_apic_mappings, memory_locations::APIC_PAGE,
};

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Error,
    Spurious,
}

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.general_protection_fault
            .set_handler_fn(general_protection_fault_handler);

        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX)
        };

        idt[InterruptIndex::Timer as u8].set_handler_fn(time_interrupt_handler);

        idt
    };
}

pub fn init_idt(phys_offset: VirtAddr) {
    init_apic_mappings(phys_offset);

    IDT.load();

    let mut lapic = LocalApicBuilder::new()
        .timer_vector(InterruptIndex::Timer as usize)
        .error_vector(InterruptIndex::Error as usize)
        .spurious_vector(InterruptIndex::Spurious as usize)
        .set_xapic_base(APIC_PAGE.start_address().as_u64())
        .build()
        .expect("we need lapic");

    unsafe {
        lapic.set_timer_initial(6250000);
        lapic.set_timer_divide(TimerDivide::Div16)
    };

    unsafe { lapic.enable() };

    info!("finish init idt!");
}

extern "x86-interrupt" fn time_interrupt_handler(_: InterruptStackFrame) {
    let mut lapic = LocalApicBuilder::new()
        .timer_vector(InterruptIndex::Timer as usize)
        .error_vector(InterruptIndex::Error as usize)
        .spurious_vector(InterruptIndex::Spurious as usize)
        .set_xapic_base(APIC_PAGE.start_address().as_u64())
        .build()
        .unwrap_or_else(|err| panic!("{}", err));
    unsafe { lapic.end_of_interrupt() };
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT {:?}", stack_frame);
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    error!("EXCEPTION: BREAKPOINT {:?}", stack_frame);
}

extern "x86-interrupt" fn general_protection_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: u64,
) {
    panic!(
        "EXCEPTION: GENERAL_PROTECTION_FAULT code: {}; {:#?}",
        error_code, stack_frame
    );
}
