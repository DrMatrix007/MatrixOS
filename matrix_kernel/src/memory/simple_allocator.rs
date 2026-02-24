use core::alloc::GlobalAlloc;

pub struct Dummy;

unsafe impl GlobalAlloc for Dummy {
    unsafe fn alloc(&self, _: core::alloc::Layout) -> *mut u8 {
        // panic!("damn");
        core::ptr::null_mut()
    }

    unsafe fn dealloc(&self, _: *mut u8, _: core::alloc::Layout) {
        panic!("damn");
    }
}

// #[global_allocator]
// static ALLOCATOR: Dummy = Dummy;
