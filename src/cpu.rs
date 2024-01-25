use crate::{device::MMIODevice, mmio::SystemInterface};

pub struct Cpu {
    next_instruction: Option<u32>,
    /// x0     -> zero register
    /// x1-x31 -> general purpose registers
    /// pc     -> program counter
    registers: [u32; 33],
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            next_instruction: None,
            registers: [0; 33],
        }
    }

    pub fn fetch(&mut self, bus: &SystemInterface) {
        let pc = self.registers[32];
        self.next_instruction = Some(bus.read(pc as usize));
    }

    pub fn execute(&mut self, _bus: &mut SystemInterface) {
        todo!();
    }
}
