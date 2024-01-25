use super::MMIODevice;

pub struct Rom {
    rom: Vec<u8>,
}

impl Rom {
    /// Create a ROM of size in bytes.
    pub fn new(size: usize) -> Self {
        Self { rom: vec![0; size] }
    }

    /// Pre-loads data into the ROM device.
    pub fn load(&mut self /* data */) {
        todo!()
    }
}

impl MMIODevice for Rom {
    fn read(&self, address: usize) -> u32 {
        let address = address % self.rom.capacity();
        ((self.rom[address] as u32) << 24)
            | ((self.rom[address + 1] as u32) << 16)
            | ((self.rom[address + 2] as u32) << 8)
            | (self.rom[address + 3] as u32)
    }

    fn write(&mut self, address: usize, value: u32) {
        let address = address % self.rom.capacity();
        self.rom[address] = (value >> 24) as u8;
        self.rom[address] = (value >> 16) as u8;
        self.rom[address] = (value >> 8) as u8;
        self.rom[address] = value as u8;
    }
}
