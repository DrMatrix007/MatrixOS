use crate::kernel_entry;

#[unsafe(no_mangle)]
pub fn _start() -> u64 {
    kernel_entry()
}
