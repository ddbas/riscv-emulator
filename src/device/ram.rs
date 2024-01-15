use super::MMIODevice;

pub struct Ram {}

impl MMIODevice for Ram {
    fn read(&self, _address: usize) -> usize {
        todo!()
    }

    fn write(&mut self, _address: usize, _value: usize) {
        todo!()
    }
}
