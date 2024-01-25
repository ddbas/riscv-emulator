mod ram;
mod rom;

pub use crate::device::ram::Ram;
pub use crate::device::rom::Rom;

/// Trait for memory-mapped I/O devices.
pub trait MMIODevice {
    fn read(&self, address: usize) -> u32;
    fn write(&mut self, address: usize, value: u32);
}
