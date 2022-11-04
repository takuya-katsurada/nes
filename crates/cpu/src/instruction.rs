#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum Opcode {
    ADC,
    AND,
    ASL,
    BCC,
    BCS,
    BEQ,
    BIT,
    BMI,
    BNE,
    BPL,
    BRK,
    BVC,
    BVS,
    CLC,
    CLD,
    CLI,
    CLV,
    CMP,
    CPX,
    DEC,
    DEX,
    DEY,
    EOR,
    INC,
    INX,
    INY,
    ORA,
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
            0x00 => Instruction(Opcode::BRK, AddressingMode::Implied),
            0x01 => Instruction(Opcode::ORA, AddressingMode::IndirectX),
            0x05 => Instruction(Opcode::ORA, AddressingMode::ZeroPage),
            0x06 => Instruction(Opcode::ASL, AddressingMode::ZeroPage),
            0x09 => Instruction(Opcode::ORA, AddressingMode::Immediate),
            0x0d => Instruction(Opcode::ORA, AddressingMode::Absolute),
            0x11 => Instruction(Opcode::ORA, AddressingMode::IndirectY),
            0x15 => Instruction(Opcode::ORA, AddressingMode::ZeroPageX),
            0x19 => Instruction(Opcode::ORA, AddressingMode::AbsoluteY),
            0x0a => Instruction(Opcode::ASL, AddressingMode::Accumulator),
            0x0e => Instruction(Opcode::ASL, AddressingMode::Absolute),
            0x10 => Instruction(Opcode::BPL, AddressingMode::Relative),
            0x16 => Instruction(Opcode::ASL, AddressingMode::ZeroPageX),
            0x18 => Instruction(Opcode::CLC, AddressingMode::Implied),
            0x1d => Instruction(Opcode::ORA, AddressingMode::AbsoluteX),
            0x1e => Instruction(Opcode::ASL, AddressingMode::AbsoluteX),
            0x21 => Instruction(Opcode::AND, AddressingMode::IndirectX),
            0x24 => Instruction(Opcode::BIT, AddressingMode::ZeroPage),
            0x25 => Instruction(Opcode::AND, AddressingMode::ZeroPage),
            0x29 => Instruction(Opcode::AND, AddressingMode::Immediate),
            0x2c => Instruction(Opcode::BIT, AddressingMode::Absolute),
            0x2d => Instruction(Opcode::AND, AddressingMode::Absolute),
            0x30 => Instruction(Opcode::BMI, AddressingMode::Relative),
            0x31 => Instruction(Opcode::AND, AddressingMode::IndirectY),
            0x35 => Instruction(Opcode::AND, AddressingMode::ZeroPageX),
            0x39 => Instruction(Opcode::AND, AddressingMode::AbsoluteY),
            0x3d => Instruction(Opcode::AND, AddressingMode::AbsoluteX),
            0x41 => Instruction(Opcode::EOR, AddressingMode::IndirectX),
            0x45 => Instruction(Opcode::EOR, AddressingMode::ZeroPage),
            0x49 => Instruction(Opcode::EOR, AddressingMode::Immediate),
            0x4d => Instruction(Opcode::EOR, AddressingMode::Absolute),
            0x50 => Instruction(Opcode::BVC, AddressingMode::Relative),
            0x51 => Instruction(Opcode::EOR, AddressingMode::IndirectY),
            0x55 => Instruction(Opcode::EOR, AddressingMode::ZeroPageX),
            0x58 => Instruction(Opcode::CLI, AddressingMode::Implied),
            0x59 => Instruction(Opcode::EOR, AddressingMode::AbsoluteY),
            0x5d => Instruction(Opcode::EOR, AddressingMode::AbsoluteX),
            0x61 => Instruction(Opcode::ADC, AddressingMode::IndirectX),
            0x65 => Instruction(Opcode::ADC, AddressingMode::ZeroPage),
            0x69 => Instruction(Opcode::ADC, AddressingMode::Immediate),
            0x6d => Instruction(Opcode::ADC, AddressingMode::Absolute),
            0x70 => Instruction(Opcode::BVS, AddressingMode::Relative),
            0x71 => Instruction(Opcode::ADC, AddressingMode::IndirectY),
            0x75 => Instruction(Opcode::ADC, AddressingMode::ZeroPageX),
            0x79 => Instruction(Opcode::ADC, AddressingMode::AbsoluteY),
            0x7d => Instruction(Opcode::ADC, AddressingMode::AbsoluteX),
            0x88 => Instruction(Opcode::DEY, AddressingMode::Implied),
            0x90 => Instruction(Opcode::BCC, AddressingMode::Relative),
            0xb0 => Instruction(Opcode::BCS, AddressingMode::Relative),
            0xb8 => Instruction(Opcode::CLV, AddressingMode::Implied),
            0xc1 => Instruction(Opcode::CMP, AddressingMode::IndirectX),
            0xc5 => Instruction(Opcode::CMP, AddressingMode::ZeroPage),
            0xc6 => Instruction(Opcode::DEC, AddressingMode::ZeroPage),
            0xc8 => Instruction(Opcode::INY, AddressingMode::Implied),
            0xc9 => Instruction(Opcode::CMP, AddressingMode::Immediate),
            0xca => Instruction(Opcode::DEX, AddressingMode::Implied),
            0xcd => Instruction(Opcode::CMP, AddressingMode::Absolute),
            0xce => Instruction(Opcode::DEC, AddressingMode::Absolute),
            0xd0 => Instruction(Opcode::BNE, AddressingMode::Relative),
            0xd1 => Instruction(Opcode::CMP, AddressingMode::IndirectY),
            0xd5 => Instruction(Opcode::CMP, AddressingMode::ZeroPageX),
            0xd6 => Instruction(Opcode::DEC, AddressingMode::ZeroPageX),
            0xd8 => Instruction(Opcode::CLD, AddressingMode::Implied),
            0xd9 => Instruction(Opcode::CMP, AddressingMode::AbsoluteY),
            0xdd => Instruction(Opcode::CMP, AddressingMode::AbsoluteX),
            0xde => Instruction(Opcode::DEC, AddressingMode::AbsoluteX),
            0xe0 => Instruction(Opcode::CPX, AddressingMode::Immediate),
            0xe4 => Instruction(Opcode::CPX, AddressingMode::ZeroPage),
            0xe6 => Instruction(Opcode::INC, AddressingMode::ZeroPage),
            0xe8 => Instruction(Opcode::INX, AddressingMode::Implied),
            0xec => Instruction(Opcode::CPX, AddressingMode::Absolute),
            0xee => Instruction(Opcode::INC, AddressingMode::Absolute),
            0xf0 => Instruction(Opcode::BEQ, AddressingMode::Relative),
            0xf6 => Instruction(Opcode::INC, AddressingMode::ZeroPageX),
            0xfe => Instruction(Opcode::INC, AddressingMode::AbsoluteX),

            _ => panic!("unsupported CPU instruction:{:08x}", opcode),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::instruction::{Instruction, Opcode, AddressingMode};

    #[test]
    fn whether_adc_instruction_was_created_from_opcode() {
        let opcodes = [0x61u8,0x65u8,0x69u8,0x6du8,0x71u8,0x75u8,0x79u8,0x7du8];
        for op in opcodes {
            let instruction = Instruction::from(op);
            assert_eq!(instruction.0, Opcode::ADC);
            assert_eq!(instruction.1, match op {
                0x69 => AddressingMode::Immediate,
                0x65 => AddressingMode::ZeroPage,
                0x75 => AddressingMode::ZeroPageX,
                0x6d => AddressingMode::Absolute,
                0x7d => AddressingMode::AbsoluteX,
                0x79 => AddressingMode::AbsoluteY,
                0x61 => AddressingMode::IndirectX,
                0x71 => AddressingMode::IndirectY,
                _ => panic!("invalid opcode has been specified")
            });
        }
    }

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
    fn whether_asl_instruction_was_created_from_opcode() {
        let opcodes = [0x06u8,0x0au8,0x0eu8,0x16u8,0x1eu8];
        for op in opcodes {
            let instruction = Instruction::from(op);
            assert_eq!(instruction.0, Opcode::ASL);
            assert_eq!(instruction.1, match op {
                0x0a => AddressingMode::Accumulator,
                0x06 => AddressingMode::ZeroPage,
                0x16 => AddressingMode::ZeroPageX,
                0x0e => AddressingMode::Absolute,
                0x1e => AddressingMode::AbsoluteX,
                _ => panic!("invalid opcode has been specified")
            });
        }
    }

    #[test]
    fn whether_bcc_instruction_was_created_from_opcode() {
        let instruction = Instruction::from(0x90u8);
        assert_eq!(instruction.0, Opcode::BCC);
        assert_eq!(instruction.1, AddressingMode::Relative);
    }

    #[test]
    fn whether_bcs_instruction_was_created_from_opcode() {
        let instruction = Instruction::from(0xb0u8);
        assert_eq!(instruction.0, Opcode::BCS);
        assert_eq!(instruction.1, AddressingMode::Relative);
    }

    #[test]
    fn whether_beq_instruction_was_created_from_opcode() {
        let instruction = Instruction::from(0xf0u8);
        assert_eq!(instruction.0, Opcode::BEQ);
        assert_eq!(instruction.1, AddressingMode::Relative);
    }

    #[test]
    fn whether_bit_instruction_was_created_from_opcode() {
        let opcodes = [0x24u8,0x2cu8];
        for op in opcodes {
            let instruction = Instruction::from(op);
            assert_eq!(instruction.0, Opcode::BIT);
            assert_eq!(instruction.1, match op {
                0x24 => AddressingMode::ZeroPage,
                0x2c => AddressingMode::Absolute,
                _ => panic!("invalid opcode has been specified")
            });
        }
    }

    #[test]
    fn whether_bmi_instruction_was_created_from_opcode() {
        let instruction = Instruction::from(0x30u8);
        assert_eq!(instruction.0, Opcode::BMI);
        assert_eq!(instruction.1, AddressingMode::Relative);
    }

    #[test]
    fn whether_bne_instruction_was_created_from_opcode() {
        let instruction = Instruction::from(0xd0u8);
        assert_eq!(instruction.0, Opcode::BNE);
        assert_eq!(instruction.1, AddressingMode::Relative);
    }

    #[test]
    fn whether_bpl_instruction_was_created_from_opcode() {
        let instruction = Instruction::from(0x10u8);
        assert_eq!(instruction.0, Opcode::BPL);
        assert_eq!(instruction.1, AddressingMode::Relative);
    }

    #[test]
    fn whether_brk_instruction_was_created_from_opcode() {
        let instruction = Instruction::from(0x00u8);
        assert_eq!(instruction.0, Opcode::BRK);
        assert_eq!(instruction.1, AddressingMode::Implied);
    }

    #[test]
    fn whether_bvc_instruction_was_created_from_opcode() {
        let instruction = Instruction::from(0x50u8);
        assert_eq!(instruction.0, Opcode::BVC);
        assert_eq!(instruction.1, AddressingMode::Relative);
    }

    #[test]
    fn whether_bvs_instruction_was_created_from_opcode() {
        let instruction = Instruction::from(0x70u8);
        assert_eq!(instruction.0, Opcode::BVS);
        assert_eq!(instruction.1, AddressingMode::Relative);
    }

    #[test]
    fn whether_clc_instruction_was_created_from_opcode() {
        let instruction = Instruction::from(0x18u8);
        assert_eq!(instruction.0, Opcode::CLC);
        assert_eq!(instruction.1, AddressingMode::Implied);
    }

    #[test]
    fn whether_cld_instruction_was_created_from_opcode() {
        let instruction = Instruction::from(0xd8u8);
        assert_eq!(instruction.0, Opcode::CLD);
        assert_eq!(instruction.1, AddressingMode::Implied);
    }

    #[test]
    fn whether_cli_instruction_was_created_from_opcode() {
        let instruction = Instruction::from(0x58u8);
        assert_eq!(instruction.0, Opcode::CLI);
        assert_eq!(instruction.1, AddressingMode::Implied);
    }

    #[test]
    fn whether_clv_instruction_was_created_from_opcode() {
        let instruction = Instruction::from(0xb8u8);
        assert_eq!(instruction.0, Opcode::CLV);
        assert_eq!(instruction.1, AddressingMode::Implied);
    }

    #[test]
    fn whether_cmp_instruction_was_created_from_opcode() {
        let opcodes = [0xc1u8,0xc5u8,0xc9u8,0xcdu8,0xd1u8,0xd5u8,0xd9u8,0xddu8];
        for op in opcodes {
            let instruction = Instruction::from(op);
            assert_eq!(instruction.0, Opcode::CMP);
            assert_eq!(instruction.1, match op {
                0xc9 => AddressingMode::Immediate,
                0xc5 => AddressingMode::ZeroPage,
                0xd5 => AddressingMode::ZeroPageX,
                0xcd => AddressingMode::Absolute,
                0xdd => AddressingMode::AbsoluteX,
                0xd9 => AddressingMode::AbsoluteY,
                0xc1 => AddressingMode::IndirectX,
                0xd1 => AddressingMode::IndirectY,
                _ => panic!("invalid opcode has been specified")
            });
        }
    }

    #[test]
    fn whether_cpx_instruction_was_created_from_opcode() {
        let opcodes = [0xe0u8,0xe4u8,0xecu8];
        for op in opcodes {
            let instruction = Instruction::from(op);
            assert_eq!(instruction.0, Opcode::CPX);
            assert_eq!(instruction.1, match op {
                0xe0 => AddressingMode::Immediate,
                0xe4 => AddressingMode::ZeroPage,
                0xec => AddressingMode::Absolute,
                _ => panic!("invalid opcode has been specified")
            });
        }
    }

    #[test]
    fn whether_dec_instruction_was_created_from_opcode() {
        let opcodes = [0xc6u8,0xceu8,0xd6u8,0xdeu8];
        for op in opcodes {
            let instruction = Instruction::from(op);
            assert_eq!(instruction.0, Opcode::DEC);
            assert_eq!(instruction.1, match op {
                0xc6 => AddressingMode::ZeroPage,
                0xd6 => AddressingMode::ZeroPageX,
                0xce => AddressingMode::Absolute,
                0xde => AddressingMode::AbsoluteX,
                _ => panic!("invalid opcode has been specified")
            });
        }
    }

    #[test]
    fn whether_dex_instruction_was_created_from_opcode() {
        let instruction = Instruction::from(0xcau8);
        assert_eq!(instruction.0, Opcode::DEX);
        assert_eq!(instruction.1, AddressingMode::Implied);
    }

    #[test]
    fn whether_dey_instruction_was_created_from_opcode() {
        let instruction = Instruction::from(0x88u8);
        assert_eq!(instruction.0, Opcode::DEY);
        assert_eq!(instruction.1, AddressingMode::Implied);
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

    #[test]
    fn whether_inc_instruction_was_created_from_opcode() {
        let opcodes = [0xe6u8,0xeeu8,0xf6u8,0xfeu8];
        for op in opcodes {
            let instruction = Instruction::from(op);
            assert_eq!(instruction.0, Opcode::INC);
            assert_eq!(instruction.1, match op {
                0xe6 => AddressingMode::ZeroPage,
                0xf6 => AddressingMode::ZeroPageX,
                0xee => AddressingMode::Absolute,
                0xfe => AddressingMode::AbsoluteX,
                _ => panic!("invalid opcode has been specified")
            });
        }
    }

    #[test]
    fn whether_inx_instruction_was_created_from_opcode() {
        let instruction = Instruction::from(0xe8u8);
        assert_eq!(instruction.0, Opcode::INX);
        assert_eq!(instruction.1, AddressingMode::Implied);
    }

    #[test]
    fn whether_iny_instruction_was_created_from_opcode() {
        let instruction = Instruction::from(0xc8u8);
        assert_eq!(instruction.0, Opcode::INY);
        assert_eq!(instruction.1, AddressingMode::Implied);
    }

    #[test]
    fn whether_ora_instruction_was_created_from_opcode() {
        let opcodes = [0x01u8,0x05u8,0x09u8,0x0du8,0x11u8,0x15u8,0x19u8,0x1du8];
        for op in opcodes {
            let instruction = Instruction::from(op);
            assert_eq!(instruction.0, Opcode::ORA);
            assert_eq!(instruction.1, match op {
                0x09 => AddressingMode::Immediate,
                0x05 => AddressingMode::ZeroPage,
                0x15 => AddressingMode::ZeroPageX,
                0x0d => AddressingMode::Absolute,
                0x1d => AddressingMode::AbsoluteX,
                0x19 => AddressingMode::AbsoluteY,
                0x01 => AddressingMode::IndirectX,
                0x11 => AddressingMode::IndirectY,
                _ => panic!("invalid opcode has been specified")
            });
        }
    }
}