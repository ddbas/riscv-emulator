use crate::{device::MMIODevice, mmio::SystemInterface, instruction::decode};

pub struct Cpu {
    /// x0     -> zero register
    /// x1-x31 -> general purpose registers
    /// pc     -> program counter
    registers: [u32; 33],
}

impl Cpu {
    pub fn new() -> Self {
        Cpu { registers: [0; 33] }
    }

    pub fn execute(&mut self, bus: &mut SystemInterface) {
        let encoded_instruction = self.fetch(bus);
        let instruction = decode(encoded_instruction);
        println!("Execute instruction: {:?}", instruction);
        todo!();
    }

    fn fetch(&mut self, bus: &SystemInterface) -> u32 {
        let pc = self.registers[32];
        println!("Fetch: {:08x}", pc);
        let value = bus.read(pc as usize);
        self.registers[32] = pc + 4;
        value
    }
}
