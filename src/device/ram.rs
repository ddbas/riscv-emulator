use super::MMIODevice;

pub struct Ram {
    ram: Vec<u8>,
}

impl Ram {
    /// Create a RAM of size in bytes.
    pub fn new(size: usize) -> Self {
        Self { ram: vec![0; size] }
    }
}

impl MMIODevice for Ram {
    fn read(&self, address: usize) -> u32 {
        let address = address % self.ram.capacity();
        ((self.ram[address] as u32) << 24)
            | ((self.ram[address + 1] as u32) << 16)
            | ((self.ram[address + 2] as u32) << 8)
            | (self.ram[address + 3] as u32)
    }

    fn write(&mut self, address: usize, value: u32) {
        let address = address % self.ram.capacity();
        self.ram[address] = (value >> 24) as u8;
        self.ram[address] = (value >> 16) as u8;
        self.ram[address] = (value >> 8) as u8;
        self.ram[address] = value as u8;
    }
}
