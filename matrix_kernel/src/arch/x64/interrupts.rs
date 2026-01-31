use lazy_static::lazy_static;
use log::{error, info};
use pic8259::ChainedPics;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

use crate::arch::x64::gdt;

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
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

#[derive(Debug, Clone, Copy, Default)]
pub struct InterruptNotInRagne;

pub fn enable_irq(pics: &mut ChainedPics, irq: InterruptIndex) -> Result<(), InterruptNotInRagne> {
    let [mut primary, mut secondary] = unsafe { pics.read_masks() };

    let irq = irq as u8 - PIC_1_OFFSET;

    match irq {
        0..=7 => {
            primary &= !(1 << irq);
        }
        8..=15 => {
            secondary &= !(1 << (irq - 8));
        }
        _ => return Err(InterruptNotInRagne),
    }

    unsafe { pics.write_masks(primary, secondary) };
    Ok(())
}

pub fn init_idt() {
    IDT.load();

    unsafe {
        let mut pics = PICS.lock();

        pics.initialize();

        enable_irq(&mut pics, InterruptIndex::Timer).unwrap();
        info!("PIC masks after enabling IRQ0: {:?}", pics.read_masks());
    };

    info!("finish init idt!");
}

extern "x86-interrupt" fn time_interrupt_handler(_: InterruptStackFrame) {
    info!("time int!");

    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer as u8)
    };
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
        "EXCEPTION: GENERAL_PROTECTION_FAULT code: {}; {:?}",
        error_code, stack_frame
    );
}
