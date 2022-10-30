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