use anyhow::{Context, Result, anyhow};
use x86_64::{VirtAddr, structures::paging::PageTableFlags};

use crate::{
    scheduler::process_memory_manager::ProcessMemoryManager, scheduler::trapframe::TrapFrame,
};

// TODO: find a better fucking way to store the loader
// #[cfg(debug_assertions)]
// static LOADER: &[u8] = include_bytes!("../../../target/debug/esp/loader.mat");
// #[cfg(not(debug_assertions))]
// static LOADER: &[u8] = include_bytes!("../../../target/release/esp/loader.mat");

pub struct Process {
    pub trap_frame: TrapFrame,
    pub memory_manager: ProcessMemoryManager,
    pub rsp: u64,
}

impl Process {
    pub fn new() -> Result<Self> {
        let mut res = {
            Self {
                rsp: 0,
                memory_manager: ProcessMemoryManager::new()
                    .context("creating the process memory manager for the process")?,
                trap_frame: TrapFrame::default(),
            }
        };

        let stack = res
            .memory_manager
            .allocate_memory(
                VirtAddr::new(0x2000000),
                64 * 0x1000,
                PageTableFlags::USER_ACCESSIBLE
                    | PageTableFlags::PRESENT
                    | PageTableFlags::WRITABLE,
            )
            .map_err(|x| anyhow!("{:?}", x))
            .context("allocating stack")?;

        res.rsp = ((stack.pages.end + 1).start_address()).as_u64();

        Ok(res)
    }
}
