#![no_std]

pub type MatrixEntryPoint = extern "C" fn() -> u64;

pub struct BootInfo {}
