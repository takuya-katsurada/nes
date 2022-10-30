#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum Opcode {
    AND
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum AddressingMode {
    Immediate
}

#[derive(Copy, Clone, Debug)]
pub struct Instruction(Opcode, AddressingMode);

impl Instruction {
    pub fn from(opcode: u8) -> Instruction {
        match opcode {
            0x29 => Instruction(Opcode::AND, AddressingMode::Immediate),
            _ => panic!("unsupported CPU instruction:{:08x}", opcode),
        }
    }
}