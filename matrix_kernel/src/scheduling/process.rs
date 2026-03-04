use crate::scheduling::trapframe::TrapFrame;

pub struct Process {
    pub trap_frame: TrapFrame,
    pub cr3: u64,
}
