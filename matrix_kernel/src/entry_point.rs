use crate::kernel_entry;

#[unsafe(no_mangle)]
extern "C" fn _start() -> u64 {
    kernel_entry()
}
