use core::fmt;

#[derive(Debug)]
pub enum Instruction {
    I {
        kind: IKind,
        immediate: i32,
        source: u8,
        destination: u8,
    },
}

#[derive(Debug)]
pub enum IKind {
    ADDI,
    SLTI,
}

pub fn decode(encoded_instruction: u32) -> Result<Instruction, InvalidInstruction> {
    if encoded_instruction & 0x7F == 0b0010011 {
        let opcode = (encoded_instruction >> 12) as u8 & 0x07;
        let kind = match opcode {
            0b000_u8 => IKind::ADDI,
            0b010_u8 => IKind::SLTI,
            _ => {
                return Err(InvalidInstruction {
                    instruction: encoded_instruction,
                })
            }
        };
        // Note: << 20 >> 20 ensures that the sign is extended across the 20 most
        // significant bits of i32.
        // E.g.,
        // "100000000001" => -2047 in 12 bits
        // 0b00000000_00000000_00001000_00000001 => "as i32"
        // 0b11111111_11111111_11111000_00000001 => i32 << 20 >> 20 => -2047 in 32 bits
        let immediate = ((encoded_instruction >> 20) as i32) << 20 >> 20;
        return Ok(Instruction::I {
            kind,
            immediate,
            source: (encoded_instruction >> 15) as u8 & 0x1f,
            destination: (encoded_instruction >> 7) as u8 & 0x1f,
        });
    }

    Err(InvalidInstruction {
        instruction: encoded_instruction,
    })
}

#[derive(Debug)]
pub struct InvalidInstruction {
    instruction: u32,
}

impl fmt::Display for InvalidInstruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid instruction: {:#032b}", self.instruction)
    }
}

impl std::error::Error for InvalidInstruction {}
