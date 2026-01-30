pub mod interrupts;
pub mod gdt;


pub fn init_x64() {
    gdt::init();
    interrupts::init_idt();

    x86_64::instructions::interrupts::enable();

}