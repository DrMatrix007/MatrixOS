use crate::kernel_entry;

#[unsafe(no_mangle)]
pub fn _start() {
    kernel_entry()
}