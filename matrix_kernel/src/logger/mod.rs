use core::{cell::UnsafeCell, fmt::Write};

use lazy_static::lazy_static;
use log::Log;
use x86_64::instructions::port::Port;

struct BasicQemuWriter(Port<u8>);

impl BasicQemuWriter {
    pub fn new() -> Self {
        BasicQemuWriter(Port::new(0x3f8))
    }
}

impl Write for BasicQemuWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for byte in s.as_bytes() {
            unsafe { self.0.write(*byte) };
        }
        Ok(())
    }
}

pub struct BasicQemuLogger(UnsafeCell<BasicQemuWriter>);

unsafe impl Sync for BasicQemuLogger {}

impl Log for BasicQemuLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::Level::Info
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            let _ = writeln!(
                unsafe { &mut *self.0.get() },
                "[ {}]: {}@{}: {}",
                record.level(),
                record.file().unwrap_or("NO_FILE"),
                record.line().unwrap_or(0),
                record.args()
            );
        }
    }

    fn flush(&self) {}
}

lazy_static! {
    static ref LOGGER: BasicQemuLogger = BasicQemuLogger(UnsafeCell::new(BasicQemuWriter::new()));
}

pub fn init_basic_logger() {
    log::set_logger(&*LOGGER).expect("failed to init logger");
    log::set_max_level(log::LevelFilter::Info);
}
