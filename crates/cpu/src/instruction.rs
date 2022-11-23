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
    STY,
    TAX,
    TAY,
    TSX,
    TXA,
    TXS,
    TYA,

    // Illegal Opcodes
    // https://wiki.nesdev.com/w/index.php/Programming_with_unofficial_opcodes
    ALR,
    ANC,
    ARR,
    AXS,
    DCP,
    ISC,
    LAX,
    RLA,
    SAX,
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

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum Support {
    Official,
    Illegal,
}

#[derive(Copy, Clone, Debug)]
pub struct Instruction {
    pub opcode: Opcode,
    pub addressing_mode: AddressingMode,
    pub support: Support,
}

impl Instruction {
    pub fn from(op: u8) -> Instruction {
        let opcode = Instruction::make_opcode(op);
        let addressing_mode = Instruction::make_addressing_mode(op);
        let support = Instruction::make_support(op);

        Instruction { opcode, addressing_mode, support }
    }

    #[inline(always)]
    fn make_opcode(op: u8) -> Opcode {
        return match op {
            0x61|0x65|0x69|0x6d|0x71|0x75|0x79|0x7d => Opcode::ADC,
            0x21|0x25|0x29|0x2d|0x31|0x35|0x39|0x3d => Opcode::AND,
            0x06|0x0a|0x0e|0x16|0x1e                => Opcode::ASL,
            0x90                                    => Opcode::BCC,
            0xb0                                    => Opcode::BCS,
            0xf0                                    => Opcode::BEQ,
            0x24|0x2c                               => Opcode::BIT,
            0x30                                    => Opcode::BMI,
            0xd0                                    => Opcode::BNE,
            0x10                                    => Opcode::BPL,
            0x00                                    => Opcode::BRK,
            0x50                                    => Opcode::BVC,
            0x70                                    => Opcode::BVS,
            0x18                                    => Opcode::CLC,
            0xd8                                    => Opcode::CLD,
            0x58                                    => Opcode::CLI,
            0xb8                                    => Opcode::CLV,
            0xc1|0xc5|0xc9|0xcd|0xd1|0xd5|0xd9|0xdd => Opcode::CMP,
            0xe0|0xe4|0xec                          => Opcode::CPX,
            0xc0|0xc4|0xcc                          => Opcode::CPY,
            0xc6|0xce|0xd6|0xde                     => Opcode::DEC,
            0xca                                    => Opcode::DEX,
            0x88                                    => Opcode::DEY,
            0x41|0x45|0x49|0x4d|0x51|0x55|0x59|0x5d => Opcode::EOR,
            0xe6|0xee|0xf6|0xfe                     => Opcode::INC,
            0xe8                                    => Opcode::INX,
            0xc8                                    => Opcode::INY,
            0x4c|0x6c                               => Opcode::JMP,
            0x20                                    => Opcode::JSR,
            0xa1|0xa5|0xa9|0xad|0xb1|0xb5|0xb9|0xbd => Opcode::LDA,
            0xa2|0xa6|0xae|0xb6|0xbe                => Opcode::LDX,
            0xa0|0xa4|0xac|0xb4|0xbc                => Opcode::LDY,
            0x46|0x4a|0x4e|0x56|0x5e                => Opcode::LSR,
            0x1a|0x3a|0x5a|0x7a|0xda|0xea|0xfa      => Opcode::NOP,
            0x01|0x05|0x09|0x0d|0x11|0x15|0x19|0x1d => Opcode::ORA,
            0x48                                    => Opcode::PHA,
            0x08                                    => Opcode::PHP,
            0x68                                    => Opcode::PLA,
            0x28                                    => Opcode::PLP,
            0x26|0x2a|0x2e|0x36|0x3e                => Opcode::ROL,
            0x66|0x6a|0x6e|0x76|0x7e                => Opcode::ROR,
            0x40                                    => Opcode::RTI,
            0x60                                    => Opcode::RTS,
            0xe1|0xe5|0xe9|0xed|0xf1|0xf5|0xf9|0xfd => Opcode::SBC,
            0x38                                    => Opcode::SEC,
            0xf8                                    => Opcode::SED,
            0x78                                    => Opcode::SEI,
            0x81|0x85|0x8d|0x91|0x95|0x99|0x9d      => Opcode::STA,
            0x86|0x8e|0x96                          => Opcode::STX,
            0x84|0x8c|0x94                          => Opcode::STY,
            0xaa                                    => Opcode::TAX,
            0xa8                                    => Opcode::TAY,
            0xba                                    => Opcode::TSX,
            0x8a                                    => Opcode::TXA,
            0x9a                                    => Opcode::TXS,
            0x98                                    => Opcode::TYA,

            0x4b                                    => Opcode::ALR,
            0x0b                                    => Opcode::ANC,
            0x6b                                    => Opcode::ARR,
            0xcb                                    => Opcode::AXS,
            0xc3|0xc7|0xcf|0xd3|0xd7|0xdb|0xdf      => Opcode::DCP,
            0xe3|0xe7|0xef|0xf3|0xf7|0xfb|0xff      => Opcode::ISC,
            0xa3|0xa7|0xaf|0xb3|0xb7|0xbf           => Opcode::LAX,
            0x23|0x27|0x2f|0x33|0x37|0x3b|0x3f      => Opcode::RLA,
            0x83|0x87|0x8f|0x97                     => Opcode::SAX,

            _ => panic!("unsupported CPU instruction:{:08x}", op),
        }
    }

    #[inline(always)]
    fn make_addressing_mode(op: u8) -> AddressingMode {
        return match op {
            0x00|0x08|0x18|0x1a|0x28|0x38|0x3a|0x40|0x48|0x58|0x5a|0x60|0x68|0x78|0x7a|
            0x88|0x8a|0x98|0x9a|0xa8|0xaa|0xb8|0xba|0xc8|0xca|0xd8|0xda|0xe8|0xea|0xf8|
            0xfa
            => AddressingMode::Implied,
            0x0a|0x2a|0x4a|0x6a
            => AddressingMode::Accumulator,
            0x09|0x0b|0x29|0x49|0x4b|0x69|0x6b|0xa0|0xa2|0xa9|0xc0|0xc9|0xcb|0xe0|0xe9
            => AddressingMode::Immediate,
            0x05|0x06|0x24|0x25|0x26|0x27|0x45|0x46|0x65|0x66|0x84|0x85|0x86|0x87|0xa4|
            0xa5|0xa6|0xa7|0xc4|0xc5|0xc6|0xc7|0xe4|0xe5|0xe6|0xe7
            => AddressingMode::ZeroPage,
            0x15|0x16|0x35|0x36|0x37|0x55|0x56|0x75|0x76|0x94|0x95|0xb4|0xb5|0xd5|0xd6|0xd7|
            0xd7|0xf5|0xf6|0xf7
            => AddressingMode::ZeroPageX,
            0x96|0x97|0xb6|0xb7
            => AddressingMode::ZeroPageY,
            0x0d|0x0e|0x20|0x2c|0x2d|0x2e|0x2f|0x4c|0x4d|0x4e|0x6d|0x6e|0x8c|0x8d|0x8e|
            0x8f|0xac|0xad|0xae|0xaf|0xcc|0xcd|0xce|0xcf|0xec|0xed|0xee|0xef
            => AddressingMode::Absolute,
            0x1d|0x1e|0x3d|0x3e|0x3f|0x5d|0x5e|0x7d|0x7e|0x9d|0xbc|0xbd|0xdd|0xde|0xdf|
            0xfd|0xfe|0xff
            => AddressingMode::AbsoluteX,
            0x19|0x39|0x3b|0x59|0x79|0x99|0xb9|0xbe|0xbf|0xd9|0xdb|0xf9|0xfb
            => AddressingMode::AbsoluteY,
            0x6c
            => AddressingMode::Indirect,
            0x01|0x21|0x23|0x41|0x61|0x81|0x83|0xa1|0xa3|0xc1|0xc3|0xe1|0xe3
            => AddressingMode::IndirectX,
            0x11|0x31|0x33|0x51|0x71|0x91|0xb1|0xb3|0xd1|0xd3|0xf1|0xf3
            => AddressingMode::IndirectY,
            0x10|0x30|0x50|0x70|0x90|0xb0|0xd0|0xf0
            => AddressingMode::Relative,
            _ => panic!("unsupported CPU instruction:{:08x}", op),
        }
    }

    #[inline(always)]
    fn make_support(op: u8) -> Support {
        return match op {
            0x0b|0x1a|0x23|0x27|0x2f|0x33|0x37|0x3a|0x3b|0x3f|0x4b|0x5a|0x6b|0x7a|0x83|
            0x87|0x8f|0x97|0xa3|0xa7|0xaf|0xb3|0xb7|0xbf|0xc3|0xc7|0xcb|0xcf|0xd3|0xd7|
            0xda|0xdb|0xdf|0xfa|0xe3|0xe7|0xef|0xf3|0xf7|0xfb|0xff
            => Support::Illegal,
            _   => Support::Official
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::instruction::{Instruction, Opcode, AddressingMode, Support};

    #[test]
    fn whether_adc_instruction_was_created_from_opcode() {
        let opcodes = [0x61u8,0x65u8,0x69u8,0x6du8,0x71u8,0x75u8,0x79u8,0x7du8];
        for op in opcodes {
            let instruction = Instruction::from(op);
            assert_eq!(instruction.opcode, Opcode::ADC);
            assert_eq!(instruction.addressing_mode, match op {
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
            assert_eq!(instruction.support, Support::Official);
        }
    }

    #[test]
    fn whether_and_instruction_was_created_from_opcode() {
        let opcodes = [0x21u8,0x25u8,0x29u8,0x2du8,0x31u8,0x35u8,0x39u8,0x3du8];
        for op in opcodes {
            let instruction = Instruction::from(op);
            assert_eq!(instruction.opcode, Opcode::AND);
            assert_eq!(instruction.addressing_mode, match op {
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
            assert_eq!(instruction.support, Support::Official);
        }
    }

    #[test]
    fn whether_asl_instruction_was_created_from_opcode() {
        let opcodes = [0x06u8,0x0au8,0x0eu8,0x16u8,0x1eu8];
        for op in opcodes {
            let instruction = Instruction::from(op);
            assert_eq!(instruction.opcode, Opcode::ASL);
            assert_eq!(instruction.addressing_mode, match op {
                0x0a => AddressingMode::Accumulator,
                0x06 => AddressingMode::ZeroPage,
                0x16 => AddressingMode::ZeroPageX,
                0x0e => AddressingMode::Absolute,
                0x1e => AddressingMode::AbsoluteX,
                _ => panic!("invalid opcode has been specified")
            });
            assert_eq!(instruction.support, Support::Official);
        }
    }

    #[test]
    fn whether_bcc_instruction_was_created_from_opcode() {
        let instruction = Instruction::from(0x90u8);
        assert_eq!(instruction.opcode, Opcode::BCC);
        assert_eq!(instruction.addressing_mode, AddressingMode::Relative);
        assert_eq!(instruction.support, Support::Official);
    }

    #[test]
    fn whether_bcs_instruction_was_created_from_opcode() {
        let instruction = Instruction::from(0xb0u8);
        assert_eq!(instruction.opcode, Opcode::BCS);
        assert_eq!(instruction.addressing_mode, AddressingMode::Relative);
        assert_eq!(instruction.support, Support::Official);
    }

    #[test]
    fn whether_beq_instruction_was_created_from_opcode() {
        let instruction = Instruction::from(0xf0u8);
        assert_eq!(instruction.opcode, Opcode::BEQ);
        assert_eq!(instruction.addressing_mode, AddressingMode::Relative);
        assert_eq!(instruction.support, Support::Official);
    }

    #[test]
    fn whether_bit_instruction_was_created_from_opcode() {
        let opcodes = [0x24u8,0x2cu8];
        for op in opcodes {
            let instruction = Instruction::from(op);
            assert_eq!(instruction.opcode, Opcode::BIT);
            assert_eq!(instruction.addressing_mode, match op {
                0x24 => AddressingMode::ZeroPage,
                0x2c => AddressingMode::Absolute,
                _ => panic!("invalid opcode has been specified")
            });
            assert_eq!(instruction.support, Support::Official);
        }
    }

    #[test]
    fn whether_bmi_instruction_was_created_from_opcode() {
        let instruction = Instruction::from(0x30u8);
        assert_eq!(instruction.opcode, Opcode::BMI);
        assert_eq!(instruction.addressing_mode, AddressingMode::Relative);
        assert_eq!(instruction.support, Support::Official);
    }

    #[test]
    fn whether_bne_instruction_was_created_from_opcode() {
        let instruction = Instruction::from(0xd0u8);
        assert_eq!(instruction.opcode, Opcode::BNE);
        assert_eq!(instruction.addressing_mode, AddressingMode::Relative);
        assert_eq!(instruction.support, Support::Official);
    }

    #[test]
    fn whether_bpl_instruction_was_created_from_opcode() {
        let instruction = Instruction::from(0x10u8);
        assert_eq!(instruction.opcode, Opcode::BPL);
        assert_eq!(instruction.addressing_mode, AddressingMode::Relative);
        assert_eq!(instruction.support, Support::Official);
    }

    #[test]
    fn whether_brk_instruction_was_created_from_opcode() {
        let instruction = Instruction::from(0x00u8);
        assert_eq!(instruction.opcode, Opcode::BRK);
        assert_eq!(instruction.addressing_mode, AddressingMode::Implied);
        assert_eq!(instruction.support, Support::Official);
    }

    #[test]
    fn whether_bvc_instruction_was_created_from_opcode() {
        let instruction = Instruction::from(0x50u8);
        assert_eq!(instruction.opcode, Opcode::BVC);
        assert_eq!(instruction.addressing_mode, AddressingMode::Relative);
        assert_eq!(instruction.support, Support::Official);
    }

    #[test]
    fn whether_bvs_instruction_was_created_from_opcode() {
        let instruction = Instruction::from(0x70u8);
        assert_eq!(instruction.opcode, Opcode::BVS);
        assert_eq!(instruction.addressing_mode, AddressingMode::Relative);
        assert_eq!(instruction.support, Support::Official);
    }

    #[test]
    fn whether_clc_instruction_was_created_from_opcode() {
        let instruction = Instruction::from(0x18u8);
        assert_eq!(instruction.opcode, Opcode::CLC);
        assert_eq!(instruction.addressing_mode, AddressingMode::Implied);
        assert_eq!(instruction.support, Support::Official);
    }

    #[test]
    fn whether_cld_instruction_was_created_from_opcode() {
        let instruction = Instruction::from(0xd8u8);
        assert_eq!(instruction.opcode, Opcode::CLD);
        assert_eq!(instruction.addressing_mode, AddressingMode::Implied);
        assert_eq!(instruction.support, Support::Official);
    }

    #[test]
    fn whether_cli_instruction_was_created_from_opcode() {
        let instruction = Instruction::from(0x58u8);
        assert_eq!(instruction.opcode, Opcode::CLI);
        assert_eq!(instruction.addressing_mode, AddressingMode::Implied);
        assert_eq!(instruction.support, Support::Official);
    }

    #[test]
    fn whether_clv_instruction_was_created_from_opcode() {
        let instruction = Instruction::from(0xb8u8);
        assert_eq!(instruction.opcode, Opcode::CLV);
        assert_eq!(instruction.addressing_mode, AddressingMode::Implied);
        assert_eq!(instruction.support, Support::Official);
    }

    #[test]
    fn whether_cmp_instruction_was_created_from_opcode() {
        let opcodes = [0xc1u8,0xc5u8,0xc9u8,0xcdu8,0xd1u8,0xd5u8,0xd9u8,0xddu8];
        for op in opcodes {
            let instruction = Instruction::from(op);
            assert_eq!(instruction.opcode, Opcode::CMP);
            assert_eq!(instruction.addressing_mode, match op {
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
            assert_eq!(instruction.support, Support::Official);
        }
    }

    #[test]
    fn whether_cpx_instruction_was_created_from_opcode() {
        let opcodes = [0xe0u8,0xe4u8,0xecu8];
        for op in opcodes {
            let instruction = Instruction::from(op);
            assert_eq!(instruction.opcode, Opcode::CPX);
            assert_eq!(instruction.addressing_mode, match op {
                0xe0 => AddressingMode::Immediate,
                0xe4 => AddressingMode::ZeroPage,
                0xec => AddressingMode::Absolute,
                _ => panic!("invalid opcode has been specified")
            });
            assert_eq!(instruction.support, Support::Official);
        }
    }

    #[test]
    fn whether_cpy_instruction_was_created_from_opcode() {
        let opcodes = [0xc0u8,0xc4u8,0xccu8];
        for op in opcodes {
            let instruction = Instruction::from(op);
            assert_eq!(instruction.opcode, Opcode::CPY);
            assert_eq!(instruction.addressing_mode, match op {
                0xc0 => AddressingMode::Immediate,
                0xc4 => AddressingMode::ZeroPage,
                0xcc => AddressingMode::Absolute,
                _ => panic!("invalid opcode has been specified")
            });
            assert_eq!(instruction.support, Support::Official);
        }
    }

    #[test]
    fn whether_dec_instruction_was_created_from_opcode() {
        let opcodes = [0xc6u8,0xceu8,0xd6u8,0xdeu8];
        for op in opcodes {
            let instruction = Instruction::from(op);
            assert_eq!(instruction.opcode, Opcode::DEC);
            assert_eq!(instruction.addressing_mode, match op {
                0xc6 => AddressingMode::ZeroPage,
                0xd6 => AddressingMode::ZeroPageX,
                0xce => AddressingMode::Absolute,
                0xde => AddressingMode::AbsoluteX,
                _ => panic!("invalid opcode has been specified")
            });
            assert_eq!(instruction.support, Support::Official);
        }
    }

    #[test]
    fn whether_dex_instruction_was_created_from_opcode() {
        let instruction = Instruction::from(0xcau8);
        assert_eq!(instruction.opcode, Opcode::DEX);
        assert_eq!(instruction.addressing_mode, AddressingMode::Implied);
        assert_eq!(instruction.support, Support::Official);
    }

    #[test]
    fn whether_dey_instruction_was_created_from_opcode() {
        let instruction = Instruction::from(0x88u8);
        assert_eq!(instruction.opcode, Opcode::DEY);
        assert_eq!(instruction.addressing_mode, AddressingMode::Implied);
        assert_eq!(instruction.support, Support::Official);
    }

    #[test]
    fn whether_eor_instruction_was_created_from_opcode() {
        let opcodes = [0x41u8,0x45u8,0x49u8,0x4du8,0x51u8,0x55u8,0x59u8,0x5du8];
        for op in opcodes {
            let instruction = Instruction::from(op);
            assert_eq!(instruction.opcode, Opcode::EOR);
            assert_eq!(instruction.addressing_mode, match op {
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
            assert_eq!(instruction.support, Support::Official);
        }
    }

    #[test]
    fn whether_inc_instruction_was_created_from_opcode() {
        let opcodes = [0xe6u8,0xeeu8,0xf6u8,0xfeu8];
        for op in opcodes {
            let instruction = Instruction::from(op);
            assert_eq!(instruction.opcode, Opcode::INC);
            assert_eq!(instruction.addressing_mode, match op {
                0xe6 => AddressingMode::ZeroPage,
                0xf6 => AddressingMode::ZeroPageX,
                0xee => AddressingMode::Absolute,
                0xfe => AddressingMode::AbsoluteX,
                _ => panic!("invalid opcode has been specified")
            });
            assert_eq!(instruction.support, Support::Official);
        }
    }

    #[test]
    fn whether_inx_instruction_was_created_from_opcode() {
        let instruction = Instruction::from(0xe8u8);
        assert_eq!(instruction.opcode, Opcode::INX);
        assert_eq!(instruction.addressing_mode, AddressingMode::Implied);
        assert_eq!(instruction.support, Support::Official);
    }

    #[test]
    fn whether_iny_instruction_was_created_from_opcode() {
        let instruction = Instruction::from(0xc8u8);
        assert_eq!(instruction.opcode, Opcode::INY);
        assert_eq!(instruction.addressing_mode, AddressingMode::Implied);
        assert_eq!(instruction.support, Support::Official);
    }

    #[test]
    fn whether_jmp_instruction_was_created_from_opcode() {
        let opcodes = [0x4cu8,0x6cu8];
        for op in opcodes {
            let instruction = Instruction::from(op);
            assert_eq!(instruction.opcode, Opcode::JMP);
            assert_eq!(instruction.addressing_mode, match op {
                0x4c => AddressingMode::Absolute,
                0x6c => AddressingMode::Indirect,
                _ => panic!("invalid opcode has been specified")
            });
            assert_eq!(instruction.support, Support::Official);
        }
    }

    #[test]
    fn whether_jsr_instruction_was_created_from_opcode() {
        let instruction = Instruction::from(0x20u8);
        assert_eq!(instruction.opcode, Opcode::JSR);
        assert_eq!(instruction.addressing_mode, AddressingMode::Absolute);
        assert_eq!(instruction.support, Support::Official);
    }

    #[test]
    fn whether_lda_instruction_was_created_from_opcode() {
        let opcodes = [0xa1u8,0xa5u8,0xa9u8,0xadu8,0xb1u8,0xb5u8,0xb9u8,0xbdu8];
        for op in opcodes {
            let instruction = Instruction::from(op);
            assert_eq!(instruction.opcode, Opcode::LDA);
            assert_eq!(instruction.addressing_mode, match op {
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
            assert_eq!(instruction.support, Support::Official);
        }
    }

    #[test]
    fn whether_ldx_instruction_was_created_from_opcode() {
        let opcodes = [0xa2u8,0xa6u8,0xaeu8,0xb6u8,0xbeu8];
        for op in opcodes {
            let instruction = Instruction::from(op);
            assert_eq!(instruction.opcode, Opcode::LDX);
            assert_eq!(instruction.addressing_mode, match op {
                0xa2 => AddressingMode::Immediate,
                0xa6 => AddressingMode::ZeroPage,
                0xb6 => AddressingMode::ZeroPageY,
                0xae => AddressingMode::Absolute,
                0xbe => AddressingMode::AbsoluteY,
                _ => panic!("invalid opcode has been specified")
            });
            assert_eq!(instruction.support, Support::Official);
        }
    }

    #[test]
    fn whether_ldy_instruction_was_created_from_opcode() {
        let opcodes = [0xa0u8,0xa4u8,0xacu8,0xb4u8,0xbcu8];
        for op in opcodes {
            let instruction = Instruction::from(op);
            assert_eq!(instruction.opcode, Opcode::LDY);
            assert_eq!(instruction.addressing_mode, match op {
                0xa0 => AddressingMode::Immediate,
                0xa4 => AddressingMode::ZeroPage,
                0xb4 => AddressingMode::ZeroPageX,
                0xac => AddressingMode::Absolute,
                0xbc => AddressingMode::AbsoluteX,
                _ => panic!("invalid opcode has been specified")
            });
            assert_eq!(instruction.support, Support::Official);
        }
    }

    #[test]
    fn whether_lsr_instruction_was_created_from_opcode() {
        let opcodes = [0x46u8,0x4au8,0x4eu8,0x56u8,0x5eu8];
        for op in opcodes {
            let instruction = Instruction::from(op);
            assert_eq!(instruction.opcode, Opcode::LSR);
            assert_eq!(instruction.addressing_mode, match op {
                0x4a => AddressingMode::Accumulator,
                0x46 => AddressingMode::ZeroPage,
                0x56 => AddressingMode::ZeroPageX,
                0x4e => AddressingMode::Absolute,
                0x5e => AddressingMode::AbsoluteX,
                _ => panic!("invalid opcode has been specified")
            });
            assert_eq!(instruction.support, Support::Official);
        }
    }

    #[test]
    fn whether_nop_instruction_was_created_from_opcode() {
        let opcodes = [0x1au8,0x3au8,0x5au8,0x7au8,0xdau8,0xeau8,0xfau8];
        for op in opcodes {
            let instruction = Instruction::from(op);
            assert_eq!(instruction.opcode, Opcode::NOP);
            assert_eq!(instruction.addressing_mode, AddressingMode::Implied);
            assert_eq!(instruction.support, match op {
                0xeau8 => Support::Official,
                0x1au8|0x3au8|0x5au8|0x7au8|0xdau8|0xfau8 => Support::Illegal,
                _ => panic!("invalid opcode has been specified")
            });
        }
    }

    #[test]
    fn whether_ora_instruction_was_created_from_opcode() {
        let opcodes = [0x01u8,0x05u8,0x09u8,0x0du8,0x11u8,0x15u8,0x19u8,0x1du8];
        for op in opcodes {
            let instruction = Instruction::from(op);
            assert_eq!(instruction.opcode, Opcode::ORA);
            assert_eq!(instruction.addressing_mode, match op {
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
            assert_eq!(instruction.support, Support::Official);
        }
    }

    #[test]
    fn whether_pha_instruction_was_created_from_opcode() {
        let instruction = Instruction::from(0x48u8);
        assert_eq!(instruction.opcode, Opcode::PHA);
        assert_eq!(instruction.addressing_mode, AddressingMode::Implied);
        assert_eq!(instruction.support, Support::Official);
    }

    #[test]
    fn whether_php_instruction_was_created_from_opcode() {
        let instruction = Instruction::from(0x08u8);
        assert_eq!(instruction.opcode, Opcode::PHP);
        assert_eq!(instruction.addressing_mode, AddressingMode::Implied);
        assert_eq!(instruction.support, Support::Official);
    }

    #[test]
    fn whether_pla_instruction_was_created_from_opcode() {
        let instruction = Instruction::from(0x68u8);
        assert_eq!(instruction.opcode, Opcode::PLA);
        assert_eq!(instruction.addressing_mode, AddressingMode::Implied);
        assert_eq!(instruction.support, Support::Official);
    }

    #[test]
    fn whether_plp_instruction_was_created_from_opcode() {
        let instruction = Instruction::from(0x28u8);
        assert_eq!(instruction.opcode, Opcode::PLP);
        assert_eq!(instruction.addressing_mode, AddressingMode::Implied);
        assert_eq!(instruction.support, Support::Official);
    }

    #[test]
    fn whether_rol_instruction_was_created_from_opcode() {
        let opcodes = [0x26u8,0x2au8,0x2eu8,0x36u8,0x3eu8];
        for op in opcodes {
            let instruction = Instruction::from(op);
            assert_eq!(instruction.opcode, Opcode::ROL);
            assert_eq!(instruction.addressing_mode, match op {
                0x2a => AddressingMode::Accumulator,
                0x26 => AddressingMode::ZeroPage,
                0x36 => AddressingMode::ZeroPageX,
                0x2e => AddressingMode::Absolute,
                0x3e => AddressingMode::AbsoluteX,
                _ => panic!("invalid opcode has been specified")
            });
            assert_eq!(instruction.support, Support::Official);
        }
    }

    #[test]
    fn whether_ror_instruction_was_created_from_opcode() {
        let opcodes = [0x66u8,0x6au8,0x6eu8,0x76u8,0x7eu8];
        for op in opcodes {
            let instruction = Instruction::from(op);
            assert_eq!(instruction.opcode, Opcode::ROR);
            assert_eq!(instruction.addressing_mode, match op {
                0x6a => AddressingMode::Accumulator,
                0x66 => AddressingMode::ZeroPage,
                0x76 => AddressingMode::ZeroPageX,
                0x6e => AddressingMode::Absolute,
                0x7e => AddressingMode::AbsoluteX,
                _ => panic!("invalid opcode has been specified")
            });
            assert_eq!(instruction.support, Support::Official);
        }
    }

    #[test]
    fn whether_rti_instruction_was_created_from_opcode() {
        let instruction = Instruction::from(0x40u8);
        assert_eq!(instruction.opcode, Opcode::RTI);
        assert_eq!(instruction.addressing_mode, AddressingMode::Implied);
        assert_eq!(instruction.support, Support::Official);
    }

    #[test]
    fn whether_rts_instruction_was_created_from_opcode() {
        let instruction = Instruction::from(0x60u8);
        assert_eq!(instruction.opcode, Opcode::RTS);
        assert_eq!(instruction.addressing_mode, AddressingMode::Implied);
        assert_eq!(instruction.support, Support::Official);
    }

    #[test]
    fn whether_sbc_instruction_was_created_from_opcode() {
        let opcodes = [0xe1u8,0xe5u8,0xe9u8,0xedu8,0xf1u8,0xf5u8,0xf9u8,0xfdu8];
        for op in opcodes {
            let instruction = Instruction::from(op);
            assert_eq!(instruction.opcode, Opcode::SBC);
            assert_eq!(instruction.addressing_mode, match op {
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
            assert_eq!(instruction.support, Support::Official);
        }
    }

    #[test]
    fn whether_sec_instruction_was_created_from_opcode() {
        let instruction = Instruction::from(0x38u8);
        assert_eq!(instruction.opcode, Opcode::SEC);
        assert_eq!(instruction.addressing_mode, AddressingMode::Implied);
        assert_eq!(instruction.support, Support::Official);
    }

    #[test]
    fn whether_sed_instruction_was_created_from_opcode() {
        let instruction = Instruction::from(0xf8u8);
        assert_eq!(instruction.opcode, Opcode::SED);
        assert_eq!(instruction.addressing_mode, AddressingMode::Implied);
        assert_eq!(instruction.support, Support::Official);
    }

    #[test]
    fn whether_sei_instruction_was_created_from_opcode() {
        let instruction = Instruction::from(0x78u8);
        assert_eq!(instruction.opcode, Opcode::SEI);
        assert_eq!(instruction.addressing_mode, AddressingMode::Implied);
        assert_eq!(instruction.support, Support::Official);
    }

    #[test]
    fn whether_sta_instruction_was_created_from_opcode() {
        let opcodes = [0x81u8,0x85u8,0x8du8,0x91u8,0x95u8,0x99u8,0x9du8];
        for op in opcodes {
            let instruction = Instruction::from(op);
            assert_eq!(instruction.opcode, Opcode::STA);
            assert_eq!(instruction.addressing_mode, match op {
                0x85 => AddressingMode::ZeroPage,
                0x95 => AddressingMode::ZeroPageX,
                0x8d => AddressingMode::Absolute,
                0x9d => AddressingMode::AbsoluteX,
                0x99 => AddressingMode::AbsoluteY,
                0x81 => AddressingMode::IndirectX,
                0x91 => AddressingMode::IndirectY,
                _ => panic!("invalid opcode has been specified")
            });
            assert_eq!(instruction.support, Support::Official);
        }
    }

    #[test]
    fn whether_stx_instruction_was_created_from_opcode() {
        let opcodes = [0x86u8,0x8eu8,0x96u8];
        for op in opcodes {
            let instruction = Instruction::from(op);
            assert_eq!(instruction.opcode, Opcode::STX);
            assert_eq!(instruction.addressing_mode, match op {
                0x86 => AddressingMode::ZeroPage,
                0x96 => AddressingMode::ZeroPageY,
                0x8e => AddressingMode::Absolute,
                _ => panic!("invalid opcode has been specified")
            });
            assert_eq!(instruction.support, Support::Official);
        }
    }

    #[test]
    fn whether_sty_instruction_was_created_from_opcode() {
        let opcodes = [0x84u8,0x8cu8,0x94u8];
        for op in opcodes {
            let instruction = Instruction::from(op);
            assert_eq!(instruction.opcode, Opcode::STY);
            assert_eq!(instruction.addressing_mode, match op {
                0x84 => AddressingMode::ZeroPage,
                0x94 => AddressingMode::ZeroPageX,
                0x8c => AddressingMode::Absolute,
                _ => panic!("invalid opcode has been specified")
            });
            assert_eq!(instruction.support, Support::Official);
        }
    }

    #[test]
    fn whether_tax_instruction_was_created_from_opcode() {
        let instruction = Instruction::from(0xaau8);
        assert_eq!(instruction.opcode, Opcode::TAX);
        assert_eq!(instruction.addressing_mode, AddressingMode::Implied);
        assert_eq!(instruction.support, Support::Official);
    }

    #[test]
    fn whether_tay_instruction_was_created_from_opcode() {
        let instruction = Instruction::from(0xa8u8);
        assert_eq!(instruction.opcode, Opcode::TAY);
        assert_eq!(instruction.addressing_mode, AddressingMode::Implied);
        assert_eq!(instruction.support, Support::Official);
    }

    #[test]
    fn whether_tsx_instruction_was_created_from_opcode() {
        let instruction = Instruction::from(0xbau8);
        assert_eq!(instruction.opcode, Opcode::TSX);
        assert_eq!(instruction.addressing_mode, AddressingMode::Implied);
        assert_eq!(instruction.support, Support::Official);
    }

    #[test]
    fn whether_txa_instruction_was_created_from_opcode() {
        let instruction = Instruction::from(0x8au8);
        assert_eq!(instruction.opcode, Opcode::TXA);
        assert_eq!(instruction.addressing_mode, AddressingMode::Implied);
        assert_eq!(instruction.support, Support::Official);
    }

    #[test]
    fn whether_txs_instruction_was_created_from_opcode() {
        let instruction = Instruction::from(0x9au8);
        assert_eq!(instruction.opcode, Opcode::TXS);
        assert_eq!(instruction.addressing_mode, AddressingMode::Implied);
        assert_eq!(instruction.support, Support::Official);
    }

    #[test]
    fn whether_tya_instruction_was_created_from_opcode() {
        let instruction = Instruction::from(0x98u8);
        assert_eq!(instruction.opcode, Opcode::TYA);
        assert_eq!(instruction.addressing_mode, AddressingMode::Implied);
        assert_eq!(instruction.support, Support::Official);
    }

    #[test]
    fn whether_alr_instruction_was_created_from_opcode() {
        let instruction = Instruction::from(0x4bu8);
        assert_eq!(instruction.opcode, Opcode::ALR);
        assert_eq!(instruction.addressing_mode, AddressingMode::Immediate);
        assert_eq!(instruction.support, Support::Illegal);
    }

    #[test]
    fn whether_anc_instruction_was_created_from_opcode() {
        let instruction = Instruction::from(0x0bu8);
        assert_eq!(instruction.opcode, Opcode::ANC);
        assert_eq!(instruction.addressing_mode, AddressingMode::Immediate);
        assert_eq!(instruction.support, Support::Illegal);
    }

    #[test]
    fn whether_arr_instruction_was_created_from_opcode() {
        let instruction = Instruction::from(0x6bu8);
        assert_eq!(instruction.opcode, Opcode::ARR);
        assert_eq!(instruction.addressing_mode, AddressingMode::Immediate);
        assert_eq!(instruction.support, Support::Illegal);
    }

    #[test]
    fn whether_axs_instruction_was_created_from_opcode() {
        let instruction = Instruction::from(0xcbu8);
        assert_eq!(instruction.opcode, Opcode::AXS);
        assert_eq!(instruction.addressing_mode, AddressingMode::Immediate);
        assert_eq!(instruction.support, Support::Illegal);
    }

    #[test]
    fn whether_dcp_instruction_was_created_from_opcode() {
        let opcodes = [0xc3u8,0xc7u8,0xcfu8,0xd3u8,0xd7u8,0xdbu8,0xdfu8];
        for op in opcodes {
            let instruction = Instruction::from(op);
            assert_eq!(instruction.opcode, Opcode::DCP);
            assert_eq!(instruction.addressing_mode, match op {
                0xc3 => AddressingMode::IndirectX,
                0xc7 => AddressingMode::ZeroPage,
                0xcf => AddressingMode::Absolute,
                0xd3 => AddressingMode::IndirectY,
                0xd7 => AddressingMode::ZeroPageX,
                0xdb => AddressingMode::AbsoluteY,
                0xdf => AddressingMode::AbsoluteX,
                _ => panic!("invalid opcode has been specified")
            });
            assert_eq!(instruction.support, Support::Illegal);
        }
    }

    #[test]
    fn whether_isc_instruction_was_created_from_opcode() {
        let opcodes = [0xe3u8,0xe7u8,0xefu8,0xf3u8,0xf7u8,0xfbu8,0xffu8];
        for op in opcodes {
            let instruction = Instruction::from(op);
            assert_eq!(instruction.opcode, Opcode::ISC);
            assert_eq!(instruction.addressing_mode, match op {
                0xe3 => AddressingMode::IndirectX,
                0xe7 => AddressingMode::ZeroPage,
                0xef => AddressingMode::Absolute,
                0xf3 => AddressingMode::IndirectY,
                0xf7 => AddressingMode::ZeroPageX,
                0xfb => AddressingMode::AbsoluteY,
                0xff => AddressingMode::AbsoluteX,
                _ => panic!("invalid opcode has been specified")
            });
            assert_eq!(instruction.support, Support::Illegal);
        }
    }

    #[test]
    fn whether_lax_instruction_was_created_from_opcode() {
        let opcodes = [0xa3u8,0xa7u8,0xafu8,0xb3u8,0xb7u8,0xbfu8];
        for op in opcodes {
            let instruction = Instruction::from(op);
            assert_eq!(instruction.opcode, Opcode::LAX);
            assert_eq!(instruction.addressing_mode, match op {
                0xa3 => AddressingMode::IndirectX,
                0xa7 => AddressingMode::ZeroPage,
                0xaf => AddressingMode::Absolute,
                0xb3 => AddressingMode::IndirectY,
                0xb7 => AddressingMode::ZeroPageY,
                0xbf => AddressingMode::AbsoluteY,
                _ => panic!("invalid opcode has been specified")
            });
            assert_eq!(instruction.support, Support::Illegal);
        }
    }

    #[test]
    fn whether_rla_instruction_was_created_from_opcode() {
        let opcodes = [0x23u8,0x27u8,0x2fu8,0x33u8,0x37u8,0x3bu8,0x3fu8];
        for op in opcodes {
            let instruction = Instruction::from(op);
            assert_eq!(instruction.opcode, Opcode::RLA);
            assert_eq!(instruction.addressing_mode, match op {
                0x23 => AddressingMode::IndirectX,
                0x27 => AddressingMode::ZeroPage,
                0x2f => AddressingMode::Absolute,
                0x33 => AddressingMode::IndirectY,
                0x37 => AddressingMode::ZeroPageX,
                0x3b => AddressingMode::AbsoluteY,
                0x3f => AddressingMode::AbsoluteX,
                _ => panic!("invalid opcode has been specified")
            });
            assert_eq!(instruction.support, Support::Illegal);
        }
    }

    #[test]
    fn whether_sax_instruction_was_created_from_opcode() {
        let opcodes = [0x83u8,0x87u8,0x8fu8,0x97u8];
        for op in opcodes {
            let instruction = Instruction::from(op);
            assert_eq!(instruction.opcode, Opcode::SAX);
            assert_eq!(instruction.addressing_mode, match op {
                0x83 => AddressingMode::IndirectX,
                0x87 => AddressingMode::ZeroPage,
                0x8f => AddressingMode::Absolute,
                0x97 => AddressingMode::ZeroPageY,
                _ => panic!("invalid opcode has been specified")
            });
            assert_eq!(instruction.support, Support::Illegal);
        }
    }
}