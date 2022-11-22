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
    pub fn from(opcode: u8) -> Instruction {
        match opcode {
            0x00 => Instruction::brk(AddressingMode::Implied),
            0x01 => Instruction::ora(AddressingMode::IndirectX),
            0x05 => Instruction::ora(AddressingMode::ZeroPage),
            0x06 => Instruction::asl(AddressingMode::ZeroPage),
            0x08 => Instruction::php(AddressingMode::Implied),
            0x09 => Instruction::ora(AddressingMode::Immediate),
            0x0a => Instruction::asl(AddressingMode::Accumulator),
            0x0b => Instruction::anc(AddressingMode::Immediate, Support::Illegal),
            0x0d => Instruction::ora(AddressingMode::Absolute),
            0x0e => Instruction::asl(AddressingMode::Absolute),
            0x10 => Instruction::bpl(AddressingMode::Relative),
            0x11 => Instruction::ora(AddressingMode::IndirectY),
            0x15 => Instruction::ora(AddressingMode::ZeroPageX),
            0x16 => Instruction::asl(AddressingMode::ZeroPageX),
            0x18 => Instruction::clc(AddressingMode::Implied),
            0x19 => Instruction::ora(AddressingMode::AbsoluteY),
            0x1d => Instruction::ora(AddressingMode::AbsoluteX),
            0x1e => Instruction::asl(AddressingMode::AbsoluteX),
            0x20 => Instruction::jsr(AddressingMode::Absolute),
            0x21 => Instruction::and(AddressingMode::IndirectX),
            0x24 => Instruction::bit(AddressingMode::ZeroPage),
            0x25 => Instruction::and(AddressingMode::ZeroPage),
            0x26 => Instruction::rol(AddressingMode::ZeroPage),
            0x28 => Instruction::plp(AddressingMode::Implied),
            0x29 => Instruction::and(AddressingMode::Immediate),
            0x2a => Instruction::rol(AddressingMode::Accumulator),
            0x2c => Instruction::bit(AddressingMode::Absolute),
            0x2d => Instruction::and(AddressingMode::Absolute),
            0x2e => Instruction::rol(AddressingMode::Absolute),
            0x30 => Instruction::bmi(AddressingMode::Relative),
            0x31 => Instruction::and(AddressingMode::IndirectY),
            0x35 => Instruction::and(AddressingMode::ZeroPageX),
            0x36 => Instruction::rol(AddressingMode::ZeroPageX),
            0x38 => Instruction::sec(AddressingMode::Implied),
            0x39 => Instruction::and(AddressingMode::AbsoluteY),
            0x3d => Instruction::and(AddressingMode::AbsoluteX),
            0x3e => Instruction::rol(AddressingMode::AbsoluteX),
            0x40 => Instruction::rti(AddressingMode::Implied),
            0x41 => Instruction::eor(AddressingMode::IndirectX),
            0x45 => Instruction::eor(AddressingMode::ZeroPage),
            0x46 => Instruction::lsr(AddressingMode::ZeroPage),
            0x48 => Instruction::pha(AddressingMode::Implied),
            0x49 => Instruction::eor(AddressingMode::Immediate),
            0x4a => Instruction::lsr(AddressingMode::Accumulator),
            0x4b => Instruction::alr(AddressingMode::Immediate, Support::Illegal),
            0x4c => Instruction::jmp(AddressingMode::Absolute),
            0x4d => Instruction::eor(AddressingMode::Absolute),
            0x4e => Instruction::lsr(AddressingMode::Absolute),
            0x50 => Instruction::bvc(AddressingMode::Relative),
            0x51 => Instruction::eor(AddressingMode::IndirectY),
            0x55 => Instruction::eor(AddressingMode::ZeroPageX),
            0x56 => Instruction::lsr(AddressingMode::ZeroPageX),
            0x58 => Instruction::cli(AddressingMode::Implied),
            0x59 => Instruction::eor(AddressingMode::AbsoluteY),
            0x5d => Instruction::eor(AddressingMode::AbsoluteX),
            0x5e => Instruction::lsr(AddressingMode::AbsoluteX),
            0x60 => Instruction::rts(AddressingMode::Implied),
            0x61 => Instruction::adc(AddressingMode::IndirectX),
            0x65 => Instruction::adc(AddressingMode::ZeroPage),
            0x66 => Instruction::ror(AddressingMode::ZeroPage),
            0x68 => Instruction::pla(AddressingMode::Implied),
            0x69 => Instruction::adc(AddressingMode::Immediate),
            0x6a => Instruction::ror(AddressingMode::Accumulator),
            0x6c => Instruction::jmp(AddressingMode::Indirect),
            0x6d => Instruction::adc(AddressingMode::Absolute),
            0x6e => Instruction::ror(AddressingMode::Absolute),
            0x70 => Instruction::bvs(AddressingMode::Relative),
            0x71 => Instruction::adc(AddressingMode::IndirectY),
            0x75 => Instruction::adc(AddressingMode::ZeroPageX),
            0x76 => Instruction::ror(AddressingMode::ZeroPageX),
            0x78 => Instruction::sei(AddressingMode::Implied),
            0x79 => Instruction::adc(AddressingMode::AbsoluteY),
            0x7d => Instruction::adc(AddressingMode::AbsoluteX),
            0x7e => Instruction::ror(AddressingMode::AbsoluteX),
            0x81 => Instruction::sta(AddressingMode::IndirectX),
            0x84 => Instruction::sty(AddressingMode::ZeroPage),
            0x85 => Instruction::sta(AddressingMode::ZeroPage),
            0x86 => Instruction::stx(AddressingMode::ZeroPage),
            0x88 => Instruction::dey(AddressingMode::Implied),
            0x8a => Instruction::txa(AddressingMode::Implied),
            0x8c => Instruction::sty(AddressingMode::Absolute),
            0x8d => Instruction::sta(AddressingMode::Absolute),
            0x8e => Instruction::stx(AddressingMode::Absolute),
            0x90 => Instruction::bcc(AddressingMode::Relative),
            0x91 => Instruction::sta(AddressingMode::IndirectY),
            0x94 => Instruction::sty(AddressingMode::ZeroPageX),
            0x95 => Instruction::sta(AddressingMode::ZeroPageX),
            0x96 => Instruction::stx(AddressingMode::ZeroPageY),
            0x98 => Instruction::tya(AddressingMode::Implied),
            0x99 => Instruction::sta(AddressingMode::AbsoluteY),
            0x9a => Instruction::txs(AddressingMode::Implied),
            0x9d => Instruction::sta(AddressingMode::AbsoluteX),
            0xa0 => Instruction::ldy(AddressingMode::Immediate),
            0xa1 => Instruction::lda(AddressingMode::IndirectX),
            0xa2 => Instruction::ldx(AddressingMode::Immediate),
            0xa4 => Instruction::ldy(AddressingMode::ZeroPage),
            0xa5 => Instruction::lda(AddressingMode::ZeroPage),
            0xa6 => Instruction::ldx(AddressingMode::ZeroPage),
            0xa8 => Instruction::tay(AddressingMode::Implied),
            0xa9 => Instruction::lda(AddressingMode::Immediate),
            0xaa => Instruction::tax(AddressingMode::Implied),
            0xac => Instruction::ldy(AddressingMode::Absolute),
            0xad => Instruction::lda(AddressingMode::Absolute),
            0xae => Instruction::ldx(AddressingMode::Absolute),
            0xb0 => Instruction::bcs(AddressingMode::Relative),
            0xb1 => Instruction::lda(AddressingMode::IndirectY),
            0xb4 => Instruction::ldy(AddressingMode::ZeroPageX),
            0xb5 => Instruction::lda(AddressingMode::ZeroPageX),
            0xb6 => Instruction::ldx(AddressingMode::ZeroPageY),
            0xb8 => Instruction::clv(AddressingMode::Implied),
            0xb9 => Instruction::lda(AddressingMode::AbsoluteY),
            0xba => Instruction::tsx(AddressingMode::Implied),
            0xbc => Instruction::ldy(AddressingMode::AbsoluteX),
            0xbd => Instruction::lda(AddressingMode::AbsoluteX),
            0xbe => Instruction::ldx(AddressingMode::AbsoluteY),
            0xc0 => Instruction::cpy(AddressingMode::Immediate),
            0xc1 => Instruction::cmp(AddressingMode::IndirectX),
            0xc4 => Instruction::cpy(AddressingMode::ZeroPage),
            0xc5 => Instruction::cmp(AddressingMode::ZeroPage),
            0xc6 => Instruction::dec(AddressingMode::ZeroPage),
            0xc8 => Instruction::iny(AddressingMode::Implied),
            0xc9 => Instruction::cmp(AddressingMode::Immediate),
            0xca => Instruction::dex(AddressingMode::Implied),
            0xcc => Instruction::cpy(AddressingMode::Absolute),
            0xcd => Instruction::cmp(AddressingMode::Absolute),
            0xce => Instruction::dec(AddressingMode::Absolute),
            0xd0 => Instruction::bne(AddressingMode::Relative),
            0xd1 => Instruction::cmp(AddressingMode::IndirectY),
            0xd5 => Instruction::cmp(AddressingMode::ZeroPageX),
            0xd6 => Instruction::dec(AddressingMode::ZeroPageX),
            0xd8 => Instruction::cld(AddressingMode::Implied),
            0xd9 => Instruction::cmp(AddressingMode::AbsoluteY),
            0xdd => Instruction::cmp(AddressingMode::AbsoluteX),
            0xde => Instruction::dec(AddressingMode::AbsoluteX),
            0xe0 => Instruction::cpx(AddressingMode::Immediate),
            0xe1 => Instruction::sbc(AddressingMode::IndirectX),
            0xe4 => Instruction::cpx(AddressingMode::ZeroPage),
            0xe5 => Instruction::sbc(AddressingMode::ZeroPage),
            0xe6 => Instruction::inc(AddressingMode::ZeroPage),
            0xe8 => Instruction::inx(AddressingMode::Implied),
            0xe9 => Instruction::sbc(AddressingMode::Immediate),
            0xea => Instruction::nop(AddressingMode::Implied),
            0xec => Instruction::cpx(AddressingMode::Absolute),
            0xed => Instruction::sbc(AddressingMode::Absolute),
            0xee => Instruction::inc(AddressingMode::Absolute),
            0xf0 => Instruction::beq(AddressingMode::Relative),
            0xf1 => Instruction::sbc(AddressingMode::IndirectY),
            0xf5 => Instruction::sbc(AddressingMode::ZeroPageX),
            0xf6 => Instruction::inc(AddressingMode::ZeroPageX),
            0xf8 => Instruction::sed(AddressingMode::Implied),
            0xf9 => Instruction::sbc(AddressingMode::AbsoluteY),
            0xfd => Instruction::sbc(AddressingMode::AbsoluteX),
            0xfe => Instruction::inc(AddressingMode::AbsoluteX),

            _ => panic!("unsupported CPU instruction:{:08x}", opcode),
        }
    }

    #[inline(always)]
    fn adc(mode: AddressingMode) -> Instruction {
        Instruction { opcode: Opcode::ADC, addressing_mode: mode, support: Support::Official }
    }

    #[inline(always)]
    fn and(mode: AddressingMode) -> Instruction {
        Instruction { opcode: Opcode::AND, addressing_mode: mode, support: Support::Official }
    }

    #[inline(always)]
    fn asl(mode: AddressingMode) -> Instruction {
        Instruction { opcode: Opcode::ASL, addressing_mode: mode, support: Support::Official }
    }

    #[inline(always)]
    fn bit(mode: AddressingMode) -> Instruction {
        Instruction { opcode: Opcode::BIT, addressing_mode: mode, support: Support::Official }
    }

    #[inline(always)]
    fn beq(mode: AddressingMode) -> Instruction {
        Instruction { opcode: Opcode::BEQ, addressing_mode: mode, support: Support::Official }
    }

    #[inline(always)]
    fn bmi(mode: AddressingMode) -> Instruction {
        Instruction { opcode: Opcode::BMI, addressing_mode: mode, support: Support::Official }
    }

    #[inline(always)]
    fn bne(mode: AddressingMode) -> Instruction {
        Instruction { opcode: Opcode::BNE, addressing_mode: mode, support: Support::Official }
    }

    #[inline(always)]
    fn bpl(mode: AddressingMode) -> Instruction {
        Instruction { opcode: Opcode::BPL, addressing_mode: mode, support: Support::Official }
    }

    #[inline(always)]
    fn brk(mode: AddressingMode) -> Instruction {
        Instruction { opcode: Opcode::BRK, addressing_mode: mode, support: Support::Official }
    }

    #[inline(always)]
    fn bcc(mode: AddressingMode) -> Instruction {
        Instruction { opcode: Opcode::BCC, addressing_mode: mode, support: Support::Official }
    }

    #[inline(always)]
    fn bcs(mode: AddressingMode) -> Instruction {
        Instruction { opcode: Opcode::BCS, addressing_mode: mode, support: Support::Official }
    }

    #[inline(always)]
    fn bvc(mode: AddressingMode) -> Instruction {
        Instruction { opcode: Opcode::BVC, addressing_mode: mode, support: Support::Official }
    }

    #[inline(always)]
    fn bvs(mode: AddressingMode) -> Instruction {
        Instruction { opcode: Opcode::BVS, addressing_mode: mode, support: Support::Official }
    }

    #[inline(always)]
    fn clc(mode: AddressingMode) -> Instruction {
        Instruction { opcode: Opcode::CLC, addressing_mode: mode, support: Support::Official }
    }

    #[inline(always)]
    fn cld(mode: AddressingMode) -> Instruction {
        Instruction { opcode: Opcode::CLD, addressing_mode: mode, support: Support::Official }
    }

    #[inline(always)]
    fn cli(mode: AddressingMode) -> Instruction {
        Instruction { opcode: Opcode::CLI, addressing_mode: mode, support: Support::Official }
    }

    #[inline(always)]
    fn clv(mode: AddressingMode) -> Instruction {
        Instruction { opcode: Opcode::CLV, addressing_mode: mode, support: Support::Official }
    }

    #[inline(always)]
    fn cmp(mode: AddressingMode) -> Instruction {
        Instruction { opcode: Opcode::CMP, addressing_mode: mode, support: Support::Official }
    }

    #[inline(always)]
    fn cpx(mode: AddressingMode) -> Instruction {
        Instruction { opcode: Opcode::CPX, addressing_mode: mode, support: Support::Official }
    }

    #[inline(always)]
    fn cpy(mode: AddressingMode) -> Instruction {
        Instruction { opcode: Opcode::CPY, addressing_mode: mode, support: Support::Official }
    }

    #[inline(always)]
    fn dec(mode: AddressingMode) -> Instruction {
        Instruction { opcode: Opcode::DEC, addressing_mode: mode, support: Support::Official }
    }

    #[inline(always)]
    fn dex(mode: AddressingMode) -> Instruction {
        Instruction { opcode: Opcode::DEX, addressing_mode: mode, support: Support::Official }
    }

    #[inline(always)]
    fn dey(mode: AddressingMode) -> Instruction {
        Instruction { opcode: Opcode::DEY, addressing_mode: mode, support: Support::Official }
    }

    #[inline(always)]
    fn eor(mode: AddressingMode) -> Instruction {
        Instruction { opcode: Opcode::EOR, addressing_mode: mode, support: Support::Official }
    }

    #[inline(always)]
    fn inc(mode: AddressingMode) -> Instruction {
        Instruction { opcode: Opcode::INC, addressing_mode: mode, support: Support::Official }
    }

    #[inline(always)]
    fn inx(mode: AddressingMode) -> Instruction {
        Instruction { opcode: Opcode::INX, addressing_mode: mode, support: Support::Official }
    }

    #[inline(always)]
    fn iny(mode: AddressingMode) -> Instruction {
        Instruction { opcode: Opcode::INY, addressing_mode: mode, support: Support::Official }
    }

    #[inline(always)]
    fn jmp(mode: AddressingMode) -> Instruction {
        Instruction { opcode: Opcode::JMP, addressing_mode: mode, support: Support::Official }
    }

    #[inline(always)]
    fn jsr(mode: AddressingMode) -> Instruction {
        Instruction { opcode: Opcode::JSR, addressing_mode: mode, support: Support::Official }
    }

    #[inline(always)]
    fn lda(mode: AddressingMode) -> Instruction {
        Instruction { opcode: Opcode::LDA, addressing_mode: mode, support: Support::Official }
    }

    #[inline(always)]
    fn ldx(mode: AddressingMode) -> Instruction {
        Instruction { opcode: Opcode::LDX, addressing_mode: mode, support: Support::Official }
    }

    #[inline(always)]
    fn ldy(mode: AddressingMode) -> Instruction {
        Instruction { opcode: Opcode::LDY, addressing_mode: mode, support: Support::Official }
    }

    #[inline(always)]
    fn lsr(mode: AddressingMode) -> Instruction {
        Instruction { opcode: Opcode::LSR, addressing_mode: mode, support: Support::Official }
    }

    #[inline(always)]
    fn nop(mode: AddressingMode) -> Instruction {
        Instruction { opcode: Opcode::NOP, addressing_mode: mode, support: Support::Official }
    }

    #[inline(always)]
    fn ora(mode: AddressingMode) -> Instruction {
        Instruction { opcode: Opcode::ORA, addressing_mode: mode, support: Support::Official }
    }

    #[inline(always)]
    fn pha(mode: AddressingMode) -> Instruction {
        Instruction { opcode: Opcode::PHA, addressing_mode: mode, support: Support::Official }
    }

    #[inline(always)]
    fn php(mode: AddressingMode) -> Instruction {
        Instruction { opcode: Opcode::PHP, addressing_mode: mode, support: Support::Official }
    }

    #[inline(always)]
    fn pla(mode: AddressingMode) -> Instruction {
        Instruction { opcode: Opcode::PLA, addressing_mode: mode, support: Support::Official }
    }

    #[inline(always)]
    fn plp(mode: AddressingMode) -> Instruction {
        Instruction { opcode: Opcode::PLP, addressing_mode: mode, support: Support::Official }
    }

    #[inline(always)]
    fn rts(mode: AddressingMode) -> Instruction {
        Instruction { opcode: Opcode::RTS, addressing_mode: mode, support: Support::Official }
    }

    #[inline(always)]
    fn rol(mode: AddressingMode) -> Instruction {
        Instruction { opcode: Opcode::ROL, addressing_mode: mode, support: Support::Official }
    }

    #[inline(always)]
    fn ror(mode: AddressingMode) -> Instruction {
        Instruction { opcode: Opcode::ROR, addressing_mode: mode, support: Support::Official }
    }

    #[inline(always)]
    fn rti(mode: AddressingMode) -> Instruction {
        Instruction { opcode: Opcode::RTI, addressing_mode: mode, support: Support::Official }
    }

    #[inline(always)]
    fn sbc(mode: AddressingMode) -> Instruction {
        Instruction { opcode: Opcode::SBC, addressing_mode: mode, support: Support::Official }
    }

    #[inline(always)]
    fn sec(mode: AddressingMode) -> Instruction {
        Instruction { opcode: Opcode::SEC, addressing_mode: mode, support: Support::Official }
    }

    #[inline(always)]
    fn sed(mode: AddressingMode) -> Instruction {
        Instruction { opcode: Opcode::SED, addressing_mode: mode, support: Support::Official }
    }

    #[inline(always)]
    fn sei(mode: AddressingMode) -> Instruction {
        Instruction { opcode: Opcode::SEI, addressing_mode: mode, support: Support::Official }
    }

    #[inline(always)]
    fn sta(mode: AddressingMode) -> Instruction {
        Instruction { opcode: Opcode::STA, addressing_mode: mode, support: Support::Official }
    }

    #[inline(always)]
    fn stx(mode: AddressingMode) -> Instruction {
        Instruction { opcode: Opcode::STX, addressing_mode: mode, support: Support::Official }
    }

    #[inline(always)]
    fn sty(mode: AddressingMode) -> Instruction {
        Instruction { opcode: Opcode::STY, addressing_mode: mode, support: Support::Official }
    }

    #[inline(always)]
    fn txa(mode: AddressingMode) -> Instruction {
        Instruction { opcode: Opcode::TXA, addressing_mode: mode, support: Support::Official }
    }

    #[inline(always)]
    fn tax(mode: AddressingMode) -> Instruction {
        Instruction { opcode: Opcode::TAX, addressing_mode: mode, support: Support::Official }
    }

    #[inline(always)]
    fn tay(mode: AddressingMode) -> Instruction {
        Instruction { opcode: Opcode::TAY, addressing_mode: mode, support: Support::Official }
    }

    #[inline(always)]
    fn tsx(mode: AddressingMode) -> Instruction {
        Instruction { opcode: Opcode::TSX, addressing_mode: mode, support: Support::Official }
    }

    #[inline(always)]
    fn txs(mode: AddressingMode) -> Instruction {
        Instruction { opcode: Opcode::TXS, addressing_mode: mode, support: Support::Official }
    }

    #[inline(always)]
    fn tya(mode: AddressingMode) -> Instruction {
        Instruction { opcode: Opcode::TYA, addressing_mode: mode, support: Support::Official }
    }

    #[inline(always)]
    fn alr(mode: AddressingMode, support: Support) -> Instruction {
        // Indicate that the instruction is an informal instruction
        // by setting the instruction table.
        debug_assert!(support == Support::Illegal);
        Instruction { opcode: Opcode::ALR, addressing_mode: mode, support }
    }

    #[inline(always)]
    fn anc(mode: AddressingMode, support: Support) -> Instruction {
        // Indicate that the instruction is an informal instruction
        // by setting the instruction table.
        debug_assert!(support == Support::Illegal);
        Instruction { opcode: Opcode::ANC, addressing_mode: mode, support }
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
        // TODO: A decision will be made later as to
        //       whether the unofficial instruction should be implemented.
        let opcodes = [0xeau8];
        for op in opcodes {
            let instruction = Instruction::from(op);
            assert_eq!(instruction.opcode, Opcode::NOP);
            assert_eq!(instruction.addressing_mode, AddressingMode::Implied);
            assert_eq!(instruction.support, Support::Official);
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
}