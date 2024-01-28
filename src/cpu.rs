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
    registers: [i32; 33],
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
                    // Note: << 20 >> 20 ensures that the sign is extended across the 20 most
                    // significant bits of i32.
                    // E.g.,
                    // "100000000001" => -2047 in 12 bits
                    // 0b00001000_00000001 => "as i16"
                    // 0b11111111_11111111_11111000_00000001 => i16 << 20 >> 20 => -2047 in 32 bits
                    self.registers[destination as usize] = self.registers[source as usize]
                        .wrapping_add((immediate as i32) << 20 >> 20);
                }
            },
        }
    }

    pub fn inspect_register(&self, register: u8) -> i32 {
        self.registers[register as usize]
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
