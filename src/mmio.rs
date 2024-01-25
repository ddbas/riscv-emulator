use crate::device::MMIODevice;

pub struct MemoryMapping {
    pub device: Box<dyn MMIODevice>,
    pub start: usize,
    pub end: usize,
}

pub struct SystemInterface {
    pub mappings: Vec<MemoryMapping>,
    pub size: usize,
}

impl MMIODevice for SystemInterface {
    fn read(&self, address: usize) -> u32 {
        let address = address % self.size;
        let mapping = self
            .mappings
            .iter()
            .find(|mapping| mapping.start <= address && mapping.end >= address);
        if let Some(mapping) = mapping {
            return mapping.device.read(address);
        }

        0
    }

    fn write(&mut self, _address: usize, _value: u32) {
        todo!()
    }
}
