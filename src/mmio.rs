use crate::device::MMIODevice;

pub struct MemoryMapping {
    pub device: Box<dyn MMIODevice>,
    pub start: usize,
    pub end: usize,
}

pub struct SystemInterface {
    pub mappings: Vec<MemoryMapping>,
}

impl MMIODevice for SystemInterface {
    fn read(&self, _address: usize) -> usize {
        todo!()
    }

    fn write(&mut self, _address: usize, _value: usize) {
        todo!()
    }
}
