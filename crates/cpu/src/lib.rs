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
            _ => panic!("invalid opcode has been specified")
        }
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
}