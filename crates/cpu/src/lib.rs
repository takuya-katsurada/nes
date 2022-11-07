mod fetch;
mod instruction;
mod register;

use memory::Memory;
use memory::system::SystemBus;

use crate::instruction::{Opcode,AddressingMode,Instruction};

#[derive(Clone)]
pub struct Cpu {
    // Accumulator
    pub a: u8,
    // Index Register X
    pub x: u8,
    // Index Register Y
    pub y: u8,
    // Processor Status Register [N:V:R:B:D:I:Z:C]
    pub p: u8,
    // Program Counter
    pub pc: u16,
    // Stack Pointer
    pub sp: u16,
}

impl Default for Cpu {
    fn default() -> Self {
        Self {
            a:  0,
            x:  0,
            y:  0,
            p:  0,
            pc: 0,
            sp: 0,
        }
    }
}

impl Cpu {
    pub fn step(&mut self, system: &mut memory::system::SystemBus) -> u8 {
        let raw_opcode = self.fetch_u8(system);
        let Instruction(opcode, mode) = Instruction::from(raw_opcode);

        match opcode {
            Opcode::CLC => {
                self.write_carry_flag(false);
                2
            }
            Opcode::CLD => {
                self.write_decimal_flag(false);
                2
            }
            Opcode::CLI => {
                self.write_interrupt_flag(false);
                2
            }
            Opcode::CLV => {
                self.write_overflow_flag(false);
                2
            }
            Opcode::NOP => {
                2
            }
            Opcode::SEC => {
                self.write_carry_flag(true);
                2
            }
            Opcode::SED => {
                self.write_decimal_flag(true);
                2
            }
            Opcode::SEI => {
                self.write_interrupt_flag(true);
                2
            }
            Opcode::TAX => {
                self.check_zero_and_negative_flag(self.a);
                self.x = self.a;
                2
            }

            _ => panic!("invalid opcode has been specified")
        }
    }

    #[inline(always)]
    fn check_zero_and_negative_flag(&mut self, value: u8) {
        self.write_zero_flag(value == 0);
        self.write_negative_flag((value & 0x80) == 0x80);
    }
}

mod tests {
    use memory::system::SystemBus;

    # [test]
    fn execute_clc_instruction()
    {
        let mut cpu = super::Cpu::default();
        let mut mem = memory::Memory::default();

        cpu.pc = 0x0000u16;
        cpu.write_carry_flag(true);
        mem.write_u8(0x0000, 0x18u8);

        let cycle = cpu.step(&mut mem);
        assert_eq!(cpu.read_carry_flag(), false);
        assert_eq!(cycle, 0x02u8);
    }

    # [test]
    fn execute_cld_instruction()
    {
        let mut cpu = super::Cpu::default();
        let mut mem = memory::Memory::default();

        cpu.pc = 0x0000u16;
        cpu.write_decimal_flag(true);
        mem.write_u8(0x0000, 0xd8u8);

        let cycle = cpu.step(&mut mem);
        assert_eq!(cpu.read_decimal_flag(), false);
        assert_eq!(cycle, 0x02u8);
    }

    # [test]
    fn execute_cli_instruction()
    {
        let mut cpu = super::Cpu::default();
        let mut mem = memory::Memory::default();

        cpu.pc = 0x0000u16;
        cpu.write_interrupt_flag(true);
        mem.write_u8(0x0000, 0x58u8);

        let cycle = cpu.step(&mut mem);
        assert_eq!(cpu.read_interrupt_flag(), false);
        assert_eq!(cycle, 0x02u8);
    }

    # [test]
    fn execute_clv_instruction()
    {
        let mut cpu = super::Cpu::default();
        let mut mem = memory::Memory::default();

        cpu.pc = 0x0000u16;
        cpu.write_overflow_flag(true);
        mem.write_u8(0x0000, 0xb8u8);

        let cycle = cpu.step(&mut mem);
        assert_eq!(cpu.read_overflow_flag(), false);
        assert_eq!(cycle, 0x02u8);
    }

    # [test]
    fn execute_nop_instruction()
    {
        let mut cpu = super::Cpu::default();
        let mut mem = memory::Memory::default();

        cpu.pc = 0x0000u16;
        mem.write_u8(0x0000, 0xeau8);

        let cycle = cpu.step(&mut mem);
        assert_eq!(cycle, 0x02u8);
    }

    # [test]
    fn execute_sec_instruction()
    {
        let mut cpu = super::Cpu::default();
        let mut mem = memory::Memory::default();

        cpu.pc = 0x0000u16;
        mem.write_u8(0x0000, 0x38u8);

        let cycle = cpu.step(&mut mem);
        assert_eq!(cpu.read_carry_flag(), true);
        assert_eq!(cycle, 0x02u8);
    }

    # [test]
    fn execute_sed_instruction()
    {
        let mut cpu = super::Cpu::default();
        let mut mem = memory::Memory::default();

        cpu.pc = 0x0000u16;
        mem.write_u8(0x0000, 0xf8u8);

        let cycle = cpu.step(&mut mem);
        assert_eq!(cpu.read_decimal_flag(), true);
        assert_eq!(cycle, 0x02u8);
    }

    # [test]
    fn execute_sei_instruction()
    {
        let mut cpu = super::Cpu::default();
        let mut mem = memory::Memory::default();

        cpu.pc = 0x0000u16;
        mem.write_u8(0x0000, 0x78u8);

        let cycle = cpu.step(&mut mem);
        assert_eq!(cpu.read_interrupt_flag(), true);
        assert_eq!(cycle, 0x02u8);
    }

    # [test]
    fn execute_tax_instruction()
    {
        let mut cpu = super::Cpu::default();
        let mut mem = memory::Memory::default();

        cpu.a = 0x0f;
        cpu.x = 0x00;
        cpu.pc = 0x0000u16;
        mem.write_u8(0x0000, 0xaau8);

        let cycle = cpu.step(&mut mem);
        assert_eq!(cpu.a, 0x0f);
        assert_eq!(cpu.x, 0x0f);
        assert_eq!(cycle, 0x02u8);
    }
}