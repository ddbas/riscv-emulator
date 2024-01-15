use super::MMIODevice;

pub struct Rom {}

impl MMIODevice for Rom {
    fn read(&self, _address: usize) -> usize {
        todo!()
    }

    fn write(&mut self, _address: usize, _value: usize) {
        // do nothing
    }
}
