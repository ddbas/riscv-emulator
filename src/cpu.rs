use std::fmt;

use crate::{
    device::MMIODevice,
    instruction::{self, decode, Instruction},
    mmio::SystemInterface,
};

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

    pub fn cycle(&mut self, bus: &mut SystemInterface) {
        let encoded_instruction = self.fetch(bus);
        let instruction = decode(encoded_instruction).unwrap_or_else(|err| panic!("Err: {}", err));
        self.execute(instruction, bus);
    }

    fn fetch(&mut self, bus: &SystemInterface) -> u32 {
        let pc = self.registers[32];
        let value = bus.read(pc as usize);
        self.registers[32] = pc + 4;
        value
    }

    fn execute(&mut self, instruction: Instruction, _bus: &mut SystemInterface) {
        match instruction {
            Instruction::I {
                kind,
                immediate,
                source,
                destination,
            } => match kind {
                instruction::IKind::ADDI => {
                    self.registers[destination as usize] =
                        self.registers[source as usize] + immediate as u32;
                }
            },
        }
    }
}

impl Default for Cpu {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (index, value) in self.registers.iter().enumerate() {
            let register_name = match index {
                32 => String::from("pc"),
                _ => format!("x{}", index),
            };
            write!(f, "{:>3}: {:#010x}", register_name, value)?;

            if index < self.registers.len() - 1 {
                write!(f, ",")?;
            }

            if index % 4 == 3 {
                writeln!(f)?;
            } else {
                write!(f, " ")?;
            }
        }
        Ok(())
    }
}
