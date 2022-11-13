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
    pub fn step(&mut self, system: &mut dyn memory::system::SystemBus) -> u8 {
        let raw_opcode = self.fetch_u8(system);
        let Instruction(opcode, mode) = Instruction::from(raw_opcode);

        match opcode {
            Opcode::AND => {
                let operand = self.fetch(system, mode);
                let result = self.a & operand.data;

                self.check_zero_and_negative_flag(result);
                self.a = result;
                1 + operand.cycle
            }
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
            Opcode::DEX => {
                let result = self.x.wrapping_sub(1);

                self.check_zero_and_negative_flag(result);
                self.x = result;
                2
            }
            Opcode::DEY => {
                let result = self.y.wrapping_sub(1);

                self.check_zero_and_negative_flag(result);
                self.y = result;
                2
            }
            Opcode::EOR => {
                let operand = self.fetch(system, mode);
                let result = self.a ^ operand.data;

                self.check_zero_and_negative_flag(result);
                self.a = result;
                1 + operand.cycle
            }
            Opcode::INX => {
                let result = self.x.wrapping_add(1);

                self.check_zero_and_negative_flag(result);
                self.x = result;
                2
            }
            Opcode::INY => {
                let result = self.y.wrapping_add(1);

                self.check_zero_and_negative_flag(result);
                self.y = result;
                2
            }
            Opcode::LDA => {
                let operand = self.fetch(system, mode);
                let result = operand.data;

                self.check_zero_and_negative_flag(result);
                self.a = result;
                1 + operand.cycle
            }
            Opcode::LDX => {
                let operand = self.fetch(system, mode);
                let result = operand.data;

                self.check_zero_and_negative_flag(result);
                self.x = result;
                1 + operand.cycle
            }
            Opcode::LDY => {
                let operand = self.fetch(system, mode);
                let result = operand.data;

                self.check_zero_and_negative_flag(result);
                self.y = result;
                1 + operand.cycle
            }
            Opcode::NOP => {
                2
            }
            Opcode::ORA => {
                let operand = self.fetch(system, mode);
                let result = self.a | operand.data;

                self.check_zero_and_negative_flag(result);
                self.a = result;
                1 + operand.cycle
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
            Opcode::STA => {
                let operand = self.fetch(system, mode);

                system.write_u8(operand.address, self.a);
                1 + operand.cycle
            }
            Opcode::STX => {
                let operand = self.fetch(system, mode);

                system.write_u8(operand.address, self.x);
                1 + operand.cycle
            }
            Opcode::STY => {
                let operand = self.fetch(system, mode);

                system.write_u8(operand.address, self.y);
                1 + operand.cycle
            }
            Opcode::TAX => {
                self.check_zero_and_negative_flag(self.a);
                self.x = self.a;
                2
            }
            Opcode::TAY => {
                self.check_zero_and_negative_flag(self.a);
                self.y = self.a;
                2
            }
            Opcode::TSX => {
                let result = (self.sp & 0x00ff) as u8;

                self.check_zero_and_negative_flag(result);
                self.x = result;
                2
            }
            Opcode::TXA => {
                self.check_zero_and_negative_flag(self.x);
                self.a = self.x;
                2
            }
            Opcode::TXS => {
                self.sp = (self.x as u16) | 0x0100u16;
                2
            }
            Opcode::TYA => {
                self.check_zero_and_negative_flag(self.y);
                self.a = self.y;
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

#[cfg(test)]
mod tests {
    use memory::system::SystemBus;

    # [test]
    fn execute_and_instruction()
    {
        let mut cpu = super::Cpu::default();
        let mut mem = memory::Memory::default();

        for param in [
            (0xff, 0x01, 0x01, false, false),
            (0xff, 0x00, 0x00, true, false),
            (0xff, 0x80, 0x80, false, true),
        ] {
            cpu.a  = param.0;
            cpu.pc = 0x0000u16;
            mem.write_u8(0x0000, 0x29);
            mem.write_u8(0x0001, param.1);

            let cycle = cpu.step(&mut mem);
            assert_eq!(cpu.a, param.2);
            assert_eq!(cpu.read_zero_flag(), param.3);
            assert_eq!(cpu.read_negative_flag(), param.4);
            assert_eq!(cycle, 0x02u8);
        }
    }

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
    fn execute_dex_instruction()
    {
        let mut cpu = super::Cpu::default();
        let mut mem = memory::Memory::default();

        for param in [
            (0x02, 0x01, false, false),
            (0x01, 0x00, true, false),
            (0x81, 0x80, false, true),
        ] {
            cpu.x  = param.0;
            cpu.pc = 0x0000u16;
            mem.write_u8(0x0000, 0xcau8);

            let cycle = cpu.step(&mut mem);
            assert_eq!(cpu.x, param.1);
            assert_eq!(cpu.read_zero_flag(), param.2);
            assert_eq!(cpu.read_negative_flag(), param.3);
            assert_eq!(cycle, 0x02u8);
        }
    }

    # [test]
    fn execute_dey_instruction()
    {
        let mut cpu = super::Cpu::default();
        let mut mem = memory::Memory::default();

        for param in [
            (0x02, 0x01, false, false),
            (0x01, 0x00, true, false),
            (0x81, 0x80, false, true),
        ] {
            cpu.y  = param.0;
            cpu.pc = 0x0000u16;
            mem.write_u8(0x0000, 0x88u8);

            let cycle = cpu.step(&mut mem);
            assert_eq!(cpu.y, param.1);
            assert_eq!(cpu.read_zero_flag(), param.2);
            assert_eq!(cpu.read_negative_flag(), param.3);
            assert_eq!(cycle, 0x02u8);
        }
    }

    # [test]
    fn execute_eor_instruction()
    {
        let mut cpu = super::Cpu::default();
        let mut mem = memory::Memory::default();

        for param in [
            (0xff, 0x8f, 0x70, false, false),
            (0xff, 0xff, 0x00, true, false),
            (0xff, 0x0f, 0xf0, false, true),
        ] {
            cpu.a  = param.0;
            cpu.pc = 0x0000u16;
            mem.write_u8(0x0000, 0x49);
            mem.write_u8(0x0001, param.1);

            let cycle = cpu.step(&mut mem);
            assert_eq!(cpu.a, param.2);
            assert_eq!(cpu.read_zero_flag(), param.3);
            assert_eq!(cpu.read_negative_flag(), param.4);
            assert_eq!(cycle, 0x02u8);
        }
    }

    # [test]
    fn execute_inx_instruction()
    {
        let mut cpu = super::Cpu::default();
        let mut mem = memory::Memory::default();

        for param in [
            (0x00, 0x01, false, false),
            (0xff, 0x00, true, false),
            (0x7f, 0x80, false, true),
        ] {
            cpu.x  = param.0;
            cpu.pc = 0x0000u16;
            mem.write_u8(0x0000, 0xe8u8);

            let cycle = cpu.step(&mut mem);
            assert_eq!(cpu.x, param.1);
            assert_eq!(cpu.read_zero_flag(), param.2);
            assert_eq!(cpu.read_negative_flag(), param.3);
            assert_eq!(cycle, 0x02u8);
        }
    }

    # [test]
    fn execute_iny_instruction()
    {
        let mut cpu = super::Cpu::default();
        let mut mem = memory::Memory::default();

        for param in [
            (0x00, 0x01, false, false),
            (0xff, 0x00, true, false),
            (0x7f, 0x80, false, true),
        ] {
            cpu.y  = param.0;
            cpu.pc = 0x0000u16;
            mem.write_u8(0x0000, 0xc8u8);

            let cycle = cpu.step(&mut mem);
            assert_eq!(cpu.y, param.1);
            assert_eq!(cpu.read_zero_flag(), param.2);
            assert_eq!(cpu.read_negative_flag(), param.3);
            assert_eq!(cycle, 0x02u8);
        }
    }

    # [test]
    fn execute_lda_instruction()
    {
        let mut cpu = super::Cpu::default();
        let mut mem = memory::Memory::default();

        for param in [
            (0x00, 0x01, false, false),
            (0x01, 0x00, true, false),
            (0x00, 0x80, false, true),
        ] {
            cpu.a  = param.0;
            cpu.pc = 0x0000u16;
            mem.write_u8(0x0000, 0xa9u8);
            mem.write_u8(0x0001, param.1);

            let cycle = cpu.step(&mut mem);
            assert_eq!(cpu.a, param.1);
            assert_eq!(cpu.read_zero_flag(), param.2);
            assert_eq!(cpu.read_negative_flag(), param.3);
            assert_eq!(cycle, 0x02u8);
        }
    }

    # [test]
    fn execute_ldx_instruction()
    {
        let mut cpu = super::Cpu::default();
        let mut mem = memory::Memory::default();

        for param in [
            (0x00, 0x01, false, false),
            (0x01, 0x00, true, false),
            (0x00, 0x80, false, true),
        ] {
            cpu.x  = param.0;
            cpu.pc = 0x0000u16;
            mem.write_u8(0x0000, 0xa2u8);
            mem.write_u8(0x0001, param.1);

            let cycle = cpu.step(&mut mem);
            assert_eq!(cpu.x, param.1);
            assert_eq!(cpu.read_zero_flag(), param.2);
            assert_eq!(cpu.read_negative_flag(), param.3);
            assert_eq!(cycle, 0x02u8);
        }
    }

    # [test]
    fn execute_ldy_instruction()
    {
        let mut cpu = super::Cpu::default();
        let mut mem = memory::Memory::default();

        for param in [
            (0x00, 0x01, false, false),
            (0x01, 0x00, true, false),
            (0x00, 0x80, false, true),
        ] {
            cpu.y  = param.0;
            cpu.pc = 0x0000u16;
            mem.write_u8(0x0000, 0xa0u8);
            mem.write_u8(0x0001, param.1);

            let cycle = cpu.step(&mut mem);
            assert_eq!(cpu.y, param.1);
            assert_eq!(cpu.read_zero_flag(), param.2);
            assert_eq!(cpu.read_negative_flag(), param.3);
            assert_eq!(cycle, 0x02u8);
        }
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
    fn execute_ora_instruction()
    {
        let mut cpu = super::Cpu::default();
        let mut mem = memory::Memory::default();

        for param in [
            (0x08, 0x07, 0x0f, false, false),
            (0x00, 0x00, 0x00, true, false),
            (0x0f, 0x80, 0x8f, false, true),
        ] {
            cpu.a  = param.0;
            cpu.pc = 0x0000u16;
            mem.write_u8(0x0000, 0x09);
            mem.write_u8(0x0001, param.1);

            let cycle = cpu.step(&mut mem);
            assert_eq!(cpu.a, param.2);
            assert_eq!(cpu.read_zero_flag(), param.3);
            assert_eq!(cpu.read_negative_flag(), param.4);
            assert_eq!(cycle, 0x02u8);
        }
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
    fn execute_sta_instruction()
    {
        let mut cpu = super::Cpu::default();
        let mut mem = memory::Memory::default();

        cpu.a  = 0xffu8;
        cpu.pc = 0x0000u16;
        mem.write_u8(0x0000, 0x85u8);
        mem.write_u8(0x0001, 0x02u8);
        mem.write_u8(0x0002, 0x0fu8);

        let cycle = cpu.step(&mut mem);
        assert_eq!(mem.read_u8(0x0002), 0xffu8);
        assert_eq!(cycle, 0x03u8);
    }

    # [test]
    fn execute_stx_instruction()
    {
        let mut cpu = super::Cpu::default();
        let mut mem = memory::Memory::default();

        cpu.x  = 0xffu8;
        cpu.pc = 0x0000u16;
        mem.write_u8(0x0000, 0x86u8);
        mem.write_u8(0x0001, 0x02u8);
        mem.write_u8(0x0002, 0x0fu8);

        let cycle = cpu.step(&mut mem);
        assert_eq!(mem.read_u8(0x0002), 0xffu8);
        assert_eq!(cycle, 0x03u8);
    }

    # [test]
    fn execute_sty_instruction()
    {
        let mut cpu = super::Cpu::default();
        let mut mem = memory::Memory::default();

        cpu.y  = 0xffu8;
        cpu.pc = 0x0000u16;
        mem.write_u8(0x0000, 0x84u8);
        mem.write_u8(0x0001, 0x02u8);
        mem.write_u8(0x0002, 0x0fu8);

        let cycle = cpu.step(&mut mem);
        assert_eq!(mem.read_u8(0x0002), 0xffu8);
        assert_eq!(cycle, 0x03u8);
    }

    # [test]
    fn execute_tax_instruction()
    {
        let mut cpu = super::Cpu::default();
        let mut mem = memory::Memory::default();

        for param in [
            (0x0f, 0x00, false, false),
            (0x00, 0xff, true, false),
            (0xf0, 0x00, false, true),
        ] {
            cpu.a  = param.0;
            cpu.x  = param.1;
            cpu.pc = 0x0000u16;
            mem.write_u8(0x0000, 0xaau8);

            let cycle = cpu.step(&mut mem);
            assert_eq!(cpu.a, param.0);
            assert_eq!(cpu.x, param.0);
            assert_eq!(cpu.read_zero_flag(), param.2);
            assert_eq!(cpu.read_negative_flag(), param.3);
            assert_eq!(cycle, 0x02u8);
        }
    }

    # [test]
    fn execute_tay_instruction()
    {
        let mut cpu = super::Cpu::default();
        let mut mem = memory::Memory::default();

        for param in [
            (0x0f, 0x00, false, false),
            (0x00, 0xff, true, false),
            (0xf0, 0x00, false, true),
        ] {
            cpu.a  = param.0;
            cpu.y  = param.1;
            cpu.pc = 0x0000u16;
            mem.write_u8(0x0000, 0xa8u8);

            let cycle = cpu.step(&mut mem);
            assert_eq!(cpu.a, param.0);
            assert_eq!(cpu.y, param.0);
            assert_eq!(cpu.read_zero_flag(), param.2);
            assert_eq!(cpu.read_negative_flag(), param.3);
            assert_eq!(cycle, 0x02u8);
        }
    }

    # [test]
    fn execute_tsx_instruction()
    {
        let mut cpu = super::Cpu::default();
        let mut mem = memory::Memory::default();

        for param in [
            (0x0010, 0x00, 0x10, false, false),
            (0x0100, 0xff, 0x00, true, false),
            (0x00f0, 0x00, 0xf0, false, true),
        ] {
            cpu.sp = param.0;
            cpu.x  = param.1;
            cpu.pc = 0x0000u16;
            mem.write_u8(0x0000, 0xbau8);

            let cycle = cpu.step(&mut mem);
            assert_eq!(cpu.sp, param.0);
            assert_eq!(cpu.x, param.2);
            assert_eq!(cpu.read_zero_flag(), param.3);
            assert_eq!(cpu.read_negative_flag(), param.4);
            assert_eq!(cycle, 0x02u8);
        }
    }

    # [test]
    fn execute_txa_instruction()
    {
        let mut cpu = super::Cpu::default();
        let mut mem = memory::Memory::default();

        for param in [
            (0x0f, 0x00, false, false),
            (0x00, 0xff, true, false),
            (0xf0, 0x00, false, true),
        ] {
            cpu.x  = param.0;
            cpu.a  = param.1;
            cpu.pc = 0x0000u16;
            mem.write_u8(0x0000, 0x8au8);

            let cycle = cpu.step(&mut mem);
            assert_eq!(cpu.a, param.0);
            assert_eq!(cpu.x, param.0);
            assert_eq!(cpu.read_zero_flag(), param.2);
            assert_eq!(cpu.read_negative_flag(), param.3);
            assert_eq!(cycle, 0x02u8);
        }
    }

    # [test]
    fn execute_txs_instruction()
    {
        let mut cpu = super::Cpu::default();
        let mut mem = memory::Memory::default();

        cpu.x  = 0x0fu8;
        cpu.pc = 0x0000u16;
        mem.write_u8(0x0000, 0x9au8);

        let cycle = cpu.step(&mut mem);
        assert_eq!(cpu.sp, 0x10fu16);
        assert_eq!(cycle, 0x02u8);
    }

    # [test]
    fn execute_tya_instruction()
    {
        let mut cpu = super::Cpu::default();
        let mut mem = memory::Memory::default();

        for param in [
            (0x0f, 0x00, false, false),
            (0x00, 0xff, true, false),
            (0xf0, 0x00, false, true),
        ] {
            cpu.y  = param.0;
            cpu.a  = param.1;
            cpu.pc = 0x0000u16;
            mem.write_u8(0x0000, 0x98u8);

            let cycle = cpu.step(&mut mem);
            assert_eq!(cpu.a, param.0);
            assert_eq!(cpu.y, param.0);
            assert_eq!(cpu.read_zero_flag(), param.2);
            assert_eq!(cpu.read_negative_flag(), param.3);
            assert_eq!(cycle, 0x02u8);
        }
    }

}