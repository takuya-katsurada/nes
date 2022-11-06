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
    CPY,
    DEC,
    DEX,
    DEY,
    EOR,
    INC,
    INX,
    INY,
    JMP,
    JSR,
    LDA,
    LDX,
    LDY,
    LSR,
    NOP,
    ORA,
    PHA,
    PHP,
    PLA,
    PLP,
    ROL,
    ROR,
    RTI,
    RTS,
    SBC,
    SEC,
    SED,
    SEI,
    STA,
    STX,
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
pub struct Instruction(pub Opcode, pub AddressingMode);

impl Instruction {
    pub fn from(opcode: u8) -> Instruction {
        match opcode {
            0x00 => Instruction(Opcode::BRK, AddressingMode::Implied),
            0x01 => Instruction(Opcode::ORA, AddressingMode::IndirectX),
            0x05 => Instruction(Opcode::ORA, AddressingMode::ZeroPage),
            0x06 => Instruction(Opcode::ASL, AddressingMode::ZeroPage),
            0x08 => Instruction(Opcode::PHP, AddressingMode::Implied),
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
            0x20 => Instruction(Opcode::JSR, AddressingMode::Absolute),
            0x21 => Instruction(Opcode::AND, AddressingMode::IndirectX),
            0x24 => Instruction(Opcode::BIT, AddressingMode::ZeroPage),
            0x25 => Instruction(Opcode::AND, AddressingMode::ZeroPage),
            0x26 => Instruction(Opcode::ROL, AddressingMode::ZeroPage),
            0x28 => Instruction(Opcode::PLP, AddressingMode::Implied),
            0x29 => Instruction(Opcode::AND, AddressingMode::Immediate),
            0x2a => Instruction(Opcode::ROL, AddressingMode::Accumulator),
            0x2c => Instruction(Opcode::BIT, AddressingMode::Absolute),
            0x2d => Instruction(Opcode::AND, AddressingMode::Absolute),
            0x2e => Instruction(Opcode::ROL, AddressingMode::Absolute),
            0x30 => Instruction(Opcode::BMI, AddressingMode::Relative),
            0x31 => Instruction(Opcode::AND, AddressingMode::IndirectY),
            0x35 => Instruction(Opcode::AND, AddressingMode::ZeroPageX),
            0x36 => Instruction(Opcode::ROL, AddressingMode::ZeroPageX),
            0x38 => Instruction(Opcode::SEC, AddressingMode::Implied),
            0x39 => Instruction(Opcode::AND, AddressingMode::AbsoluteY),
            0x3d => Instruction(Opcode::AND, AddressingMode::AbsoluteX),
            0x3e => Instruction(Opcode::ROL, AddressingMode::AbsoluteX),
            0x40 => Instruction(Opcode::RTI, AddressingMode::Implied),
            0x41 => Instruction(Opcode::EOR, AddressingMode::IndirectX),
            0x45 => Instruction(Opcode::EOR, AddressingMode::ZeroPage),
            0x46 => Instruction(Opcode::LSR, AddressingMode::ZeroPage),
            0x48 => Instruction(Opcode::PHA, AddressingMode::Implied),
            0x49 => Instruction(Opcode::EOR, AddressingMode::Immediate),
            0x4a => Instruction(Opcode::LSR, AddressingMode::Accumulator),
            0x4c => Instruction(Opcode::JMP, AddressingMode::Absolute),
            0x4d => Instruction(Opcode::EOR, AddressingMode::Absolute),
            0x4e => Instruction(Opcode::LSR, AddressingMode::Absolute),
            0x50 => Instruction(Opcode::BVC, AddressingMode::Relative),
            0x51 => Instruction(Opcode::EOR, AddressingMode::IndirectY),
            0x55 => Instruction(Opcode::EOR, AddressingMode::ZeroPageX),
            0x56 => Instruction(Opcode::LSR, AddressingMode::ZeroPageX),
            0x58 => Instruction(Opcode::CLI, AddressingMode::Implied),
            0x59 => Instruction(Opcode::EOR, AddressingMode::AbsoluteY),
            0x5d => Instruction(Opcode::EOR, AddressingMode::AbsoluteX),
            0x5e => Instruction(Opcode::LSR, AddressingMode::AbsoluteX),
            0x60 => Instruction(Opcode::RTS, AddressingMode::Implied),
            0x61 => Instruction(Opcode::ADC, AddressingMode::IndirectX),
            0x65 => Instruction(Opcode::ADC, AddressingMode::ZeroPage),
            0x66 => Instruction(Opcode::ROR, AddressingMode::ZeroPage),
            0x68 => Instruction(Opcode::PLA, AddressingMode::Implied),
            0x69 => Instruction(Opcode::ADC, AddressingMode::Immediate),
            0x6a => Instruction(Opcode::ROR, AddressingMode::Accumulator),
            0x6c => Instruction(Opcode::JMP, AddressingMode::Indirect),
            0x6d => Instruction(Opcode::ADC, AddressingMode::Absolute),
            0x6e => Instruction(Opcode::ROR, AddressingMode::Absolute),
            0x70 => Instruction(Opcode::BVS, AddressingMode::Relative),
            0x71 => Instruction(Opcode::ADC, AddressingMode::IndirectY),
            0x75 => Instruction(Opcode::ADC, AddressingMode::ZeroPageX),
            0x76 => Instruction(Opcode::ROR, AddressingMode::ZeroPageX),
            0x78 => Instruction(Opcode::SEI, AddressingMode::Implied),
            0x79 => Instruction(Opcode::ADC, AddressingMode::AbsoluteY),
            0x7d => Instruction(Opcode::ADC, AddressingMode::AbsoluteX),
            0x7e => Instruction(Opcode::ROR, AddressingMode::AbsoluteX),
            0x81 => Instruction(Opcode::STA, AddressingMode::IndirectX),
            0x85 => Instruction(Opcode::STA, AddressingMode::ZeroPage),
            0x86 => Instruction(Opcode::STX, AddressingMode::ZeroPage),
            0x88 => Instruction(Opcode::DEY, AddressingMode::Implied),
            0x8d => Instruction(Opcode::STA, AddressingMode::Absolute),
            0x8e => Instruction(Opcode::STX, AddressingMode::Absolute),
            0x90 => Instruction(Opcode::BCC, AddressingMode::Relative),
            0x91 => Instruction(Opcode::STA, AddressingMode::IndirectY),
            0x95 => Instruction(Opcode::STA, AddressingMode::ZeroPageX),
            0x96 => Instruction(Opcode::STX, AddressingMode::ZeroPageY),
            0x99 => Instruction(Opcode::STA, AddressingMode::AbsoluteY),
            0x9d => Instruction(Opcode::STA, AddressingMode::AbsoluteX),
            0xa0 => Instruction(Opcode::LDY, AddressingMode::Immediate),
            0xa1 => Instruction(Opcode::LDA, AddressingMode::IndirectX),
            0xa2 => Instruction(Opcode::LDX, AddressingMode::Immediate),
            0xa4 => Instruction(Opcode::LDY, AddressingMode::ZeroPage),
            0xa5 => Instruction(Opcode::LDA, AddressingMode::ZeroPage),
            0xa6 => Instruction(Opcode::LDX, AddressingMode::ZeroPage),
            0xa9 => Instruction(Opcode::LDA, AddressingMode::Immediate),
            0xac => Instruction(Opcode::LDY, AddressingMode::Absolute),
            0xad => Instruction(Opcode::LDA, AddressingMode::Absolute),
            0xae => Instruction(Opcode::LDX, AddressingMode::Absolute),
            0xb0 => Instruction(Opcode::BCS, AddressingMode::Relative),
            0xb1 => Instruction(Opcode::LDA, AddressingMode::IndirectY),
            0xb4 => Instruction(Opcode::LDY, AddressingMode::ZeroPageX),
            0xb5 => Instruction(Opcode::LDA, AddressingMode::ZeroPageX),
            0xb6 => Instruction(Opcode::LDX, AddressingMode::ZeroPageY),
            0xb8 => Instruction(Opcode::CLV, AddressingMode::Implied),
            0xb9 => Instruction(Opcode::LDA, AddressingMode::AbsoluteY),
            0xbc => Instruction(Opcode::LDY, AddressingMode::AbsoluteX),
            0xbd => Instruction(Opcode::LDA, AddressingMode::AbsoluteX),
            0xbe => Instruction(Opcode::LDX, AddressingMode::AbsoluteY),
            0xc0 => Instruction(Opcode::CPY, AddressingMode::Immediate),
            0xc1 => Instruction(Opcode::CMP, AddressingMode::IndirectX),
            0xc4 => Instruction(Opcode::CPY, AddressingMode::ZeroPage),
            0xc5 => Instruction(Opcode::CMP, AddressingMode::ZeroPage),
            0xc6 => Instruction(Opcode::DEC, AddressingMode::ZeroPage),
            0xc8 => Instruction(Opcode::INY, AddressingMode::Implied),
            0xc9 => Instruction(Opcode::CMP, AddressingMode::Immediate),
            0xca => Instruction(Opcode::DEX, AddressingMode::Implied),
            0xcc => Instruction(Opcode::CPY, AddressingMode::Absolute),
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
            0xe1 => Instruction(Opcode::SBC, AddressingMode::IndirectX),
            0xe4 => Instruction(Opcode::CPX, AddressingMode::ZeroPage),
            0xe5 => Instruction(Opcode::SBC, AddressingMode::ZeroPage),
            0xe6 => Instruction(Opcode::INC, AddressingMode::ZeroPage),
            0xe8 => Instruction(Opcode::INX, AddressingMode::Implied),
            0xe9 => Instruction(Opcode::SBC, AddressingMode::Immediate),
            0xea => Instruction(Opcode::NOP, AddressingMode::Implied),
            0xec => Instruction(Opcode::CPX, AddressingMode::Absolute),
            0xed => Instruction(Opcode::SBC, AddressingMode::Absolute),
            0xee => Instruction(Opcode::INC, AddressingMode::Absolute),
            0xf0 => Instruction(Opcode::BEQ, AddressingMode::Relative),
            0xf1 => Instruction(Opcode::SBC, AddressingMode::IndirectY),
            0xf5 => Instruction(Opcode::SBC, AddressingMode::ZeroPageX),
            0xf6 => Instruction(Opcode::INC, AddressingMode::ZeroPageX),
            0xf8 => Instruction(Opcode::SED, AddressingMode::Implied),
            0xf9 => Instruction(Opcode::SBC, AddressingMode::AbsoluteY),
            0xfd => Instruction(Opcode::SBC, AddressingMode::AbsoluteX),
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
    fn whether_cpy_instruction_was_created_from_opcode() {
        let opcodes = [0xc0u8,0xc4u8,0xccu8];
        for op in opcodes {
            let instruction = Instruction::from(op);
            assert_eq!(instruction.0, Opcode::CPY);
            assert_eq!(instruction.1, match op {
                0xc0 => AddressingMode::Immediate,
                0xc4 => AddressingMode::ZeroPage,
                0xcc => AddressingMode::Absolute,
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
    fn whether_jmp_instruction_was_created_from_opcode() {
        let opcodes = [0x4cu8,0x6cu8];
        for op in opcodes {
            let instruction = Instruction::from(op);
            assert_eq!(instruction.0, Opcode::JMP);
            assert_eq!(instruction.1, match op {
                0x4c => AddressingMode::Absolute,
                0x6c => AddressingMode::Indirect,
                _ => panic!("invalid opcode has been specified")
            });
        }
    }

    #[test]
    fn whether_jsr_instruction_was_created_from_opcode() {
        let instruction = Instruction::from(0x20u8);
        assert_eq!(instruction.0, Opcode::JSR);
        assert_eq!(instruction.1, AddressingMode::Absolute);
    }

    #[test]
    fn whether_lda_instruction_was_created_from_opcode() {
        let opcodes = [0xa1u8,0xa5u8,0xa9u8,0xadu8,0xb1u8,0xb5u8,0xb9u8,0xbdu8];
        for op in opcodes {
            let instruction = Instruction::from(op);
            assert_eq!(instruction.0, Opcode::LDA);
            assert_eq!(instruction.1, match op {
                0xa9 => AddressingMode::Immediate,
                0xa5 => AddressingMode::ZeroPage,
                0xb5 => AddressingMode::ZeroPageX,
                0xad => AddressingMode::Absolute,
                0xbd => AddressingMode::AbsoluteX,
                0xb9 => AddressingMode::AbsoluteY,
                0xa1 => AddressingMode::IndirectX,
                0xb1 => AddressingMode::IndirectY,
                _ => panic!("invalid opcode has been specified")
            });
        }
    }

    #[test]
    fn whether_ldx_instruction_was_created_from_opcode() {
        let opcodes = [0xa2u8,0xa6u8,0xaeu8,0xb6u8,0xbeu8];
        for op in opcodes {
            let instruction = Instruction::from(op);
            assert_eq!(instruction.0, Opcode::LDX);
            assert_eq!(instruction.1, match op {
                0xa2 => AddressingMode::Immediate,
                0xa6 => AddressingMode::ZeroPage,
                0xb6 => AddressingMode::ZeroPageY,
                0xae => AddressingMode::Absolute,
                0xbe => AddressingMode::AbsoluteY,
                _ => panic!("invalid opcode has been specified")
            });
        }
    }

    #[test]
    fn whether_ldy_instruction_was_created_from_opcode() {
        let opcodes = [0xa0u8,0xa4u8,0xacu8,0xb4u8,0xbcu8];
        for op in opcodes {
            let instruction = Instruction::from(op);
            assert_eq!(instruction.0, Opcode::LDY);
            assert_eq!(instruction.1, match op {
                0xa0 => AddressingMode::Immediate,
                0xa4 => AddressingMode::ZeroPage,
                0xb4 => AddressingMode::ZeroPageX,
                0xac => AddressingMode::Absolute,
                0xbc => AddressingMode::AbsoluteX,
                _ => panic!("invalid opcode has been specified")
            });
        }
    }

    #[test]
    fn whether_lsr_instruction_was_created_from_opcode() {
        let opcodes = [0x46u8,0x4au8,0x4eu8,0x56u8,0x5eu8];
        for op in opcodes {
            let instruction = Instruction::from(op);
            assert_eq!(instruction.0, Opcode::LSR);
            assert_eq!(instruction.1, match op {
                0x4a => AddressingMode::Accumulator,
                0x46 => AddressingMode::ZeroPage,
                0x56 => AddressingMode::ZeroPageX,
                0x4e => AddressingMode::Absolute,
                0x5e => AddressingMode::AbsoluteX,
                _ => panic!("invalid opcode has been specified")
            });
        }
    }

    #[test]
    fn whether_nop_instruction_was_created_from_opcode() {
        // TODO: A decision will be made later as to
        //       whether the unofficial instruction should be implemented.
        let opcodes = [0xeau8];
        for op in opcodes {
            let instruction = Instruction::from(op);
            assert_eq!(instruction.0, Opcode::NOP);
            assert_eq!(instruction.1, AddressingMode::Implied);
        }
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

    #[test]
    fn whether_pha_instruction_was_created_from_opcode() {
        let instruction = Instruction::from(0x48u8);
        assert_eq!(instruction.0, Opcode::PHA);
        assert_eq!(instruction.1, AddressingMode::Implied);
    }

    #[test]
    fn whether_php_instruction_was_created_from_opcode() {
        let instruction = Instruction::from(0x08u8);
        assert_eq!(instruction.0, Opcode::PHP);
        assert_eq!(instruction.1, AddressingMode::Implied);
    }

    #[test]
    fn whether_pla_instruction_was_created_from_opcode() {
        let instruction = Instruction::from(0x68u8);
        assert_eq!(instruction.0, Opcode::PLA);
        assert_eq!(instruction.1, AddressingMode::Implied);
    }

    #[test]
    fn whether_plp_instruction_was_created_from_opcode() {
        let instruction = Instruction::from(0x28u8);
        assert_eq!(instruction.0, Opcode::PLP);
        assert_eq!(instruction.1, AddressingMode::Implied);
    }

    #[test]
    fn whether_rol_instruction_was_created_from_opcode() {
        let opcodes = [0x26u8,0x2au8,0x2eu8,0x36u8,0x3eu8];
        for op in opcodes {
            let instruction = Instruction::from(op);
            assert_eq!(instruction.0, Opcode::ROL);
            assert_eq!(instruction.1, match op {
                0x2a => AddressingMode::Accumulator,
                0x26 => AddressingMode::ZeroPage,
                0x36 => AddressingMode::ZeroPageX,
                0x2e => AddressingMode::Absolute,
                0x3e => AddressingMode::AbsoluteX,
                _ => panic!("invalid opcode has been specified")
            });
        }
    }

    #[test]
    fn whether_ror_instruction_was_created_from_opcode() {
        let opcodes = [0x66u8,0x6au8,0x6eu8,0x76u8,0x7eu8];
        for op in opcodes {
            let instruction = Instruction::from(op);
            assert_eq!(instruction.0, Opcode::ROR);
            assert_eq!(instruction.1, match op {
                0x6a => AddressingMode::Accumulator,
                0x66 => AddressingMode::ZeroPage,
                0x76 => AddressingMode::ZeroPageX,
                0x6e => AddressingMode::Absolute,
                0x7e => AddressingMode::AbsoluteX,
                _ => panic!("invalid opcode has been specified")
            });
        }
    }

    #[test]
    fn whether_rti_instruction_was_created_from_opcode() {
        let instruction = Instruction::from(0x40u8);
        assert_eq!(instruction.0, Opcode::RTI);
        assert_eq!(instruction.1, AddressingMode::Implied);
    }

    #[test]
    fn whether_rts_instruction_was_created_from_opcode() {
        let instruction = Instruction::from(0x60u8);
        assert_eq!(instruction.0, Opcode::RTS);
        assert_eq!(instruction.1, AddressingMode::Implied);
    }

    #[test]
    fn whether_sbc_instruction_was_created_from_opcode() {
        let opcodes = [0xe1u8,0xe5u8,0xe9u8,0xedu8,0xf1u8,0xf5u8,0xf9u8,0xfdu8];
        for op in opcodes {
            let instruction = Instruction::from(op);
            assert_eq!(instruction.0, Opcode::SBC);
            assert_eq!(instruction.1, match op {
                0xe9 => AddressingMode::Immediate,
                0xe5 => AddressingMode::ZeroPage,
                0xf5 => AddressingMode::ZeroPageX,
                0xed => AddressingMode::Absolute,
                0xfd => AddressingMode::AbsoluteX,
                0xf9 => AddressingMode::AbsoluteY,
                0xe1 => AddressingMode::IndirectX,
                0xf1 => AddressingMode::IndirectY,
                _ => panic!("invalid opcode has been specified")
            });
        }
    }

    #[test]
    fn whether_sec_instruction_was_created_from_opcode() {
        let instruction = Instruction::from(0x38u8);
        assert_eq!(instruction.0, Opcode::SEC);
        assert_eq!(instruction.1, AddressingMode::Implied);
    }

    #[test]
    fn whether_sed_instruction_was_created_from_opcode() {
        let instruction = Instruction::from(0xf8u8);
        assert_eq!(instruction.0, Opcode::SED);
        assert_eq!(instruction.1, AddressingMode::Implied);
    }

    #[test]
    fn whether_sei_instruction_was_created_from_opcode() {
        let instruction = Instruction::from(0x78u8);
        assert_eq!(instruction.0, Opcode::SEI);
        assert_eq!(instruction.1, AddressingMode::Implied);
    }

    #[test]
    fn whether_sta_instruction_was_created_from_opcode() {
        let opcodes = [0x81u8,0x85u8,0x8du8,0x91u8,0x95u8,0x99u8,0x9du8];
        for op in opcodes {
            let instruction = Instruction::from(op);
            assert_eq!(instruction.0, Opcode::STA);
            assert_eq!(instruction.1, match op {
                0x85 => AddressingMode::ZeroPage,
                0x95 => AddressingMode::ZeroPageX,
                0x8d => AddressingMode::Absolute,
                0x9d => AddressingMode::AbsoluteX,
                0x99 => AddressingMode::AbsoluteY,
                0x81 => AddressingMode::IndirectX,
                0x91 => AddressingMode::IndirectY,
                _ => panic!("invalid opcode has been specified")
            });
        }
    }

    #[test]
    fn whether_stx_instruction_was_created_from_opcode() {
        let opcodes = [0x86u8,0x8eu8,0x96u8];
        for op in opcodes {
            let instruction = Instruction::from(op);
            assert_eq!(instruction.0, Opcode::STX);
            assert_eq!(instruction.1, match op {
                0x86 => AddressingMode::ZeroPage,
                0x96 => AddressingMode::ZeroPageY,
                0x8e => AddressingMode::Absolute,
                _ => panic!("invalid opcode has been specified")
            });
        }
    }
}