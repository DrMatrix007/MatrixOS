use core::{cell::UnsafeCell, fmt::Write};

use log::Log;
use x86_64::instructions::port::Port;

struct BasicQemuWriter(Port<u8>);

impl BasicQemuWriter {
    pub const fn new() -> Self {
        BasicQemuWriter(Port::<u8>::new(0x3f8))
    }
}

impl Write for BasicQemuWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        unsafe {
            for byte in s.as_bytes() {
                self.0.write(*byte);
            }
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
                "{} - {}",
                record.level(),
                record.args()
            );
        }
    }

    fn flush(&self) {}
}

static LOGGER: BasicQemuLogger = BasicQemuLogger(UnsafeCell::new(BasicQemuWriter::new()));

pub fn init_basic_logger() {
    log::set_logger(&LOGGER).expect("failed to init logger");
}
