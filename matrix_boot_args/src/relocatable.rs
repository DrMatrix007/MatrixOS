pub trait Relocatable {
    /// # Safety
    ///
    /// this changes the pointers in `Self` 
    /// 
    unsafe fn relocated(&self, relocate_addr: u64) -> Self;
}