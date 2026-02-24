use log::info;

pub mod gdt;
pub mod interrupts;


/// # Safety
///
/// init some x64 stuff. should be called once in the boot of the kernel 
/// 
pub unsafe fn init_x64() {
    gdt::init();
    interrupts::init_idt();

    info!("before enable interrutps");
    x86_64::instructions::interrupts::enable();
    info!("after enable interrutps");
}
