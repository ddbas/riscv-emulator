use super::MMIODevice;

pub struct Rom {}

impl Rom {
    /// Pre-loads data into the ROM device.
    pub fn load(&mut self /* data */) {
        todo!()
    }
}

impl MMIODevice for Rom {
    fn read(&self, _address: usize) -> usize {
        todo!()
    }

    fn write(&mut self, _address: usize, _value: usize) {
        // do nothing
    }
}
