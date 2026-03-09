use anyhow::{Context, Result, anyhow};
use x86_64::{
    VirtAddr,
    structures::paging::{
        FrameAllocator, Mapper, Page, PageTable, PageTableFlags, Size4KiB, mapper::CleanUp,
    },
};

use crate::{
    memory::{PAGE_TABLE, allocator::FRAME_ALLOCATOR},
    memory_locations::PROCESS_CREATION_PAGE_MAP_BASE,
    scheduler::process_memory_manager::ProcessMemoryManager,
    scheduler::trapframe::TrapFrame,
};

pub struct Process {
    pub trap_frame: TrapFrame,
    pub memory_manager: ProcessMemoryManager,
    pub rsp: u64,
}

impl Process {
    pub fn new(f: fn()) -> Result<Self> {
        let mut res = {
            let mut frame_allocator = FRAME_ALLOCATOR.lock();
            let mut current_page_table = PAGE_TABLE.lock();

            let new_page_table_frame = frame_allocator
                .allocate_frame()
                .context("allocating frame for the new process's pagetable")?;

            let new_page_table_page =
                Page::<Size4KiB>::containing_address(VirtAddr::new(PROCESS_CREATION_PAGE_MAP_BASE));

            let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;

            unsafe {
                current_page_table
                    .map_to(
                        new_page_table_page,
                        new_page_table_frame,
                        flags,
                        &mut *frame_allocator,
                    )
                    .unwrap()
                    .flush();
            };

            let new_page_table = unsafe {
                &mut *(new_page_table_page.start_address().as_mut_ptr() as *mut PageTable)
            };

            *new_page_table = PageTable::new();

            current_page_table
                .unmap(new_page_table_page)
                .map_err(|x| anyhow!("{:?}", x))
                .context("unmapping the temp page table")?
                .1
                .flush();

            unsafe {
                current_page_table
                    .inner_mut()
                    .clean_up(&mut *frame_allocator)
            };

            Self {
                rsp: 0,
                memory_manager: ProcessMemoryManager::new(new_page_table_frame),
                trap_frame: TrapFrame::default(),
            }
        };

        let _ = res
            .memory_manager
            .allocate_memory(VirtAddr::new(0x2000000), 64 * 0x1000)
            .map_err(|x| anyhow!("{:?}", x))
            .context("allocating stack")?;

        Ok(res)
    }
}
