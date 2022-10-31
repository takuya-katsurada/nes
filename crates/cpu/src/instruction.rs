#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum Opcode {
    AND
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
            _ => panic!("unsupported CPU instruction:{:08x}", opcode),
        }
    }
}