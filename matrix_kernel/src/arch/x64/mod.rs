use log::info;
use x86_64::VirtAddr;

pub mod gdt;
pub mod interrupts;

/// # Safety
///
/// init some x64 stuff. should be called once in the boot of the kernel
///
pub unsafe fn init_x64(phys_offset: VirtAddr) {
    gdt::init();
    interrupts::init_idt(phys_offset);

    info!("before enable interrutps");
    x86_64::instructions::interrupts::enable();
    info!("after enable interrutps");
}
