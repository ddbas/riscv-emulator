pub struct ExecutionEnvironment {
    pub cpu: Cpu,
    pub system_interface: SystemInterface,
}

pub struct Cpu {}

/// Trait for memory-mapped I/O devices.
pub trait MMIODevice {
    fn read(&self, address: usize) -> usize;
    fn write(&mut self, address: usize, value: usize);
}

pub struct MemoryMapping {
    pub device: Box<dyn MMIODevice>,
    pub start: usize,
    pub end: usize,
}

pub struct SystemInterface {
    pub memory_mappings: Vec<MemoryMapping>,
}

impl MMIODevice for SystemInterface {
    fn read(&self, _address: usize) -> usize {
        todo!()
    }

    fn write(&mut self, _address: usize, _value: usize) {
        todo!()
    }
}

pub struct RomDevice {}

impl MMIODevice for RomDevice {
    fn read(&self, _address: usize) -> usize {
        todo!()
    }

    fn write(&mut self, _address: usize, _value: usize) {
        // do nothing
    }
}

pub struct RamDevice {}

impl MMIODevice for RamDevice {
    fn read(&self, _address: usize) -> usize {
        todo!()
    }

    fn write(&mut self, _address: usize, _value: usize) {
        todo!()
    }
}
