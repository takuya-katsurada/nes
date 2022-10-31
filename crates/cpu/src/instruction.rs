#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum Opcode {
    AND,
    EOR,
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum AddressingMode {
    Implied,
    Accumulator,
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IndirectX,
    IndirectY,
    Relative,
}

#[derive(Copy, Clone, Debug)]
pub struct Instruction(Opcode, AddressingMode);

impl Instruction {
    pub fn from(opcode: u8) -> Instruction {
        match opcode {
            0x21 => Instruction(Opcode::AND, AddressingMode::IndirectX),
            0x25 => Instruction(Opcode::AND, AddressingMode::ZeroPage),
            0x29 => Instruction(Opcode::AND, AddressingMode::Immediate),
            0x2d => Instruction(Opcode::AND, AddressingMode::Absolute),
            0x31 => Instruction(Opcode::AND, AddressingMode::IndirectY),
            0x35 => Instruction(Opcode::AND, AddressingMode::ZeroPageX),
            0x39 => Instruction(Opcode::AND, AddressingMode::AbsoluteY),
            0x3d => Instruction(Opcode::AND, AddressingMode::AbsoluteX),
            0x41 => Instruction(Opcode::EOR, AddressingMode::IndirectX),
            0x45 => Instruction(Opcode::EOR, AddressingMode::ZeroPage),
            0x49 => Instruction(Opcode::EOR, AddressingMode::Immediate),
            0x4d => Instruction(Opcode::EOR, AddressingMode::Absolute),
            0x51 => Instruction(Opcode::EOR, AddressingMode::IndirectY),
            0x55 => Instruction(Opcode::EOR, AddressingMode::ZeroPageX),
            0x59 => Instruction(Opcode::EOR, AddressingMode::AbsoluteY),
            0x5d => Instruction(Opcode::EOR, AddressingMode::AbsoluteX),

            _ => panic!("unsupported CPU instruction:{:08x}", opcode),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::instruction::{Instruction, Opcode, AddressingMode};

    #[test]
    fn whether_and_instruction_was_created_from_opcode() {
        let opcodes = [0x21u8,0x25u8,0x29u8,0x2du8,0x31u8,0x35u8,0x39u8,0x3du8];
        for op in opcodes {
            let instruction = Instruction::from(op);
            assert_eq!(instruction.0, Opcode::AND);
            assert_eq!(instruction.1, match op {
                0x29 => AddressingMode::Immediate,
                0x25 => AddressingMode::ZeroPage,
                0x35 => AddressingMode::ZeroPageX,
                0x2d => AddressingMode::Absolute,
                0x3d => AddressingMode::AbsoluteX,
                0x39 => AddressingMode::AbsoluteY,
                0x21 => AddressingMode::IndirectX,
                0x31 => AddressingMode::IndirectY,
                _ => panic!("invalid opcode has been specified")
            });
        }
    }

    #[test]
    fn whether_eor_instruction_was_created_from_opcode() {
        let opcodes = [0x41u8,0x45u8,0x49u8,0x4du8,0x51u8,0x55u8,0x59u8,0x5du8];
        for op in opcodes {
            let instruction = Instruction::from(op);
            assert_eq!(instruction.0, Opcode::EOR);
            assert_eq!(instruction.1, match op {
                0x49 => AddressingMode::Immediate,
                0x45 => AddressingMode::ZeroPage,
                0x55 => AddressingMode::ZeroPageX,
                0x4d => AddressingMode::Absolute,
                0x5d => AddressingMode::AbsoluteX,
                0x59 => AddressingMode::AbsoluteY,
                0x41 => AddressingMode::IndirectX,
                0x51 => AddressingMode::IndirectY,
                _ => panic!("invalid opcode has been specified")
            });
        }
    }
}