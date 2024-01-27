use core::fmt;

#[derive(Debug)]
pub enum Instruction {
    I {
        kind: IKind,
        immediate: u16,
        source: u8,
        destination: u8,
    },
}

#[derive(Debug)]
pub enum IKind {
    ADDI,
}

pub fn decode(encoded_instruction: u32) -> Result<Instruction, InvalidInstruction> {
    if encoded_instruction & 0x7F == 0b0010011 {
        let opcode = (encoded_instruction >> 12) as u8 & 0x07;
        let kind = match opcode {
            0 => IKind::ADDI,
            _ => {
                return Err(InvalidInstruction {
                    instruction: encoded_instruction,
                })
            }
        };
        return Ok(Instruction::I {
            kind,
            immediate: (encoded_instruction >> 20) as u16,
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
