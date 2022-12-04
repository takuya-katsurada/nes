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

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum Interrupt {
    BRK,
    IRQ,
    NMI,
    RESET,
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
    pub fn interrupt(&mut self, system: &mut dyn memory::system::SystemBus, request_type: Interrupt) {
        let is_multilevel_interrupt = self.read_interrupt_flag();
        if is_multilevel_interrupt && (request_type == Interrupt::IRQ || request_type == Interrupt::BRK) {
            return
        }

        let address = match request_type {
            Interrupt::BRK => {
                self.write_break_flag(true);
                self.pc = self.pc + 1;

                let lo = (self.pc >> 8) as u8;
                let hi = (self.pc & 0xff) as u8;

                self.stack_push(system, lo);
                self.stack_push(system, hi);
                self.stack_push(system, self.p);
                self.write_interrupt_flag(true);

                (0xfffeu16, 0xffffu16)
            },
            Interrupt::IRQ => {
                self.write_break_flag(false);

                let lo = (self.pc >> 8) as u8;
                let hi = (self.pc & 0xff) as u8;

                self.stack_push(system, lo);
                self.stack_push(system, hi);
                self.stack_push(system, self.p);
                self.write_interrupt_flag(true);

                (0xfffeu16, 0xffffu16)
            },
            Interrupt::NMI => {
                self.write_break_flag(false);

                let lo = (self.pc >> 8) as u8;
                let hi = (self.pc & 0xff) as u8;

                self.stack_push(system, lo);
                self.stack_push(system, hi);
                self.stack_push(system, self.p);
                self.write_interrupt_flag(true);

                (0xfffau16, 0xfffbu16)
            }
            Interrupt::RESET => {
                self.write_interrupt_flag(true);
                (0xfffcu16, 0xfffdu16)
            }
        };

        let lo = system.read_u8(address.0);
        let hi = system.read_u8(address.1);

        self.pc = u16::from(lo) | (u16::from(hi) << 8);
    }

    pub fn step(&mut self, system: &mut dyn memory::system::SystemBus) -> u8 {
        let current_pc = self.pc;
        let raw_opcode = self.fetch_u8(system);
        let instruction = Instruction::from(raw_opcode);

        let opcode = instruction.opcode;
        let mode = instruction.addressing_mode;

        match opcode {
            Opcode::ADC => {
                let operand = self.fetch(system, mode);
                let v = u16::from(self.a) + u16::from(operand.data) +
                    (if self.read_carry_flag() { 1 } else { 0 });
                let result = (v & 0xff) as u8;

                let of = ((self.a ^ result) & (operand.data ^ result) & 0x80u8) == 0x80u8;
                self.write_carry_flag(v > 0x00ffu16);
                self.write_overflow_flag(of);
                self.check_zero_and_negative_flag(result);
                self.a = result;
                1 + operand.cycle
            }
            Opcode::AND => {
                let operand = self.fetch(system, mode);
                let result = self.a & operand.data;

                self.check_zero_and_negative_flag(result);
                self.a = result;
                1 + operand.cycle
            }
            Opcode::ASL => {
                let operand = self.fetch(system, mode);
                let result = operand.data.wrapping_shl(1);

                self.write_carry_flag((operand.data & 0x80) == 0x80);
                self.check_zero_and_negative_flag(result);
                if mode == AddressingMode::Accumulator {
                    self.a = result;
                    1 + operand.cycle
                } else {
                    system.write_u8(operand.address, result);
                    3 + operand.cycle
                }
            }
            Opcode::BCC => {
                let operand = self.fetch(system, mode);
                if !self.read_carry_flag() {
                    self.pc = operand.address;
                    2 + operand.cycle
                } else {
                    1 + operand.cycle
                }
            }
            Opcode::BCS => {
                let operand = self.fetch(system, mode);
                if self.read_carry_flag() {
                    self.pc = operand.address;
                    2 + operand.cycle
                } else {
                    1 + operand.cycle
                }
            }
            Opcode::BEQ => {
                let operand = self.fetch(system, mode);
                if self.read_zero_flag() {
                    self.pc = operand.address;
                    2 + operand.cycle
                } else {
                    1 + operand.cycle
                }
            }
            Opcode::BIT => {
                let operand = self.fetch(system, mode);
                let v = operand.data;
                self.write_overflow_flag((v & 0x40) == 0x40);
                self.check_zero_and_negative_flag(v);
                1 + operand.cycle
            }
            Opcode::BMI => {
                let operand = self.fetch(system, mode);
                if self.read_negative_flag() {
                    self.pc = operand.address;
                    2 + operand.cycle
                } else {
                    1 + operand.cycle
                }
            }
            Opcode::BNE => {
                let operand = self.fetch(system, mode);
                if !self.read_zero_flag() {
                    self.pc = operand.address;
                    2 + operand.cycle
                } else {
                    1 + operand.cycle
                }
            }
            Opcode::BPL => {
                let operand = self.fetch(system, mode);
                if !self.read_negative_flag() {
                    self.pc = operand.address;
                    2 + operand.cycle
                } else {
                    1 + operand.cycle
                }
            }
            Opcode::BRK => {
                self.interrupt(system, Interrupt::BRK);
                7
            }
            Opcode::BVC => {
                let operand = self.fetch(system, mode);
                if !self.read_overflow_flag() {
                    self.pc = operand.address;
                    2 + operand.cycle
                } else {
                    1 + operand.cycle
                }
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
            Opcode::CMP => {
                let operand = self.fetch(system, mode);
                let (result, _) = self.a.overflowing_sub(operand.data);

                self.write_carry_flag(self.a > result);
                self.check_zero_and_negative_flag(result);
                1 + operand.cycle
            }
            Opcode::CPX => {
                let operand = self.fetch(system, mode);
                let (result, _) = self.x.overflowing_sub(operand.data);

                self.write_carry_flag(self.x > result);
                self.check_zero_and_negative_flag(result);
                1 + operand.cycle
            }
            Opcode::CPY => {
                let operand = self.fetch(system, mode);
                let (result, _) = self.y.overflowing_sub(operand.data);

                self.write_carry_flag(self.y > result);
                self.check_zero_and_negative_flag(result);
                1 + operand.cycle
            }
            Opcode::DEC => {
                let operand = self.fetch(system, mode);
                let result = operand.data.wrapping_sub(1);

                self.check_zero_and_negative_flag(result);
                system.write_u8(operand.address, result);
                3 + operand.cycle
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
            Opcode::INC => {
                let operand = self.fetch(system, mode);
                let result = operand.data.wrapping_add(1);

                self.check_zero_and_negative_flag(result);
                system.write_u8(operand.address, result);
                3 + operand.cycle
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
            Opcode::JMP => {
                let operand = self.fetch(system, mode);
                self.pc = operand.address;
                operand.cycle
            }
            Opcode::JSR => {
                let operand = self.fetch(system, mode);

                let address = current_pc + 2;
                self.stack_push(system, (address >> 8) as u8);
                self.stack_push(system, (address & 0xff) as u8);
                self.pc = operand.address;
                6
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
            Opcode::LSR => {
                let operand = self.fetch(system, mode);
                let result = operand.data.wrapping_shr(1);

                self.write_carry_flag((operand.data & 0x01) == 0x01);
                self.check_zero_and_negative_flag(result);
                if mode == AddressingMode::Accumulator {
                    self.a = result;
                    1 + operand.cycle
                } else {
                    system.write_u8(operand.address, result);
                    3 + operand.cycle
                }
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
            Opcode::PHA => {
                self.stack_push(system, self.a);
                3
            }
            Opcode::PHP => {
                self.stack_push(system, self.p);
                3
            }
            Opcode::PLA => {
                let result = self.stack_pop(system);

                self.check_zero_and_negative_flag(result);
                self.a = result;
                4
            }
            Opcode::PLP => {
                let result = self.stack_pop(system);
                self.p = result;
                4
            }
            Opcode::ROL => {
                let operand = self.fetch(system, mode);
                let result = operand.data.wrapping_shl(1) | (
                    if self.read_carry_flag() { 0x01 } else { 0x00 }
                );

                self.write_carry_flag((operand.data & 0x80) == 0x80);
                self.check_zero_and_negative_flag(result);

                if mode == AddressingMode::Accumulator {
                    self.a = result;
                    1 + operand.cycle
                } else {
                    system.write_u8(operand.address, result);
                    3 + operand.cycle
                }
            }
            Opcode::ROR => {
                let operand = self.fetch(system, mode);
                let result = operand.data.wrapping_shr(1) | (
                    if self.read_carry_flag() { 0x80 } else { 0x00 }
                );

                self.write_carry_flag((operand.data & 0x01) == 0x01);
                self.check_zero_and_negative_flag(result);

                if mode == AddressingMode::Accumulator {
                    self.a = result;
                    1 + operand.cycle
                } else {
                    system.write_u8(operand.address, result);
                    3 + operand.cycle
                }
            }
            Opcode::RTI => {
                let status = self.stack_pop(system);
                let lo = self.stack_pop(system);
                let hi = self.stack_pop(system);
                self.p = status;
                self.pc = u16::from(lo) | (u16::from(hi) << 8);
                6
            }
            Opcode::RTS => {
                let lo = self.stack_pop(system);
                let hi = self.stack_pop(system);
                self.pc = u16::from(lo) | (u16::from(hi) << 8) + 1;
                6
            }
            Opcode::SBC => {
                let operand = self.fetch(system, mode);
                let (v, c1) = self.a.overflowing_sub(operand.data);
                let (result, c2) = v.overflowing_sub(
                    if self.read_carry_flag(){ 0 } else { 1 }
                );

                let of = (((self.a ^ operand.data) & 0x80) == 0x80) && (((self.a ^ result) & 0x80) == 0x80);
                self.write_carry_flag(!(c1 || c2));
                self.write_negative_flag(of);
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

            Opcode::ALR => {
                let operand = self.fetch(system, mode);

                let v = self.a & operand.data;
                let result = v.wrapping_shr(1);

                self.write_carry_flag((v & 0x01) == 0x01);
                self.check_zero_and_negative_flag(result);
                self.a = result;
                1 + operand.cycle
            }
            Opcode::ANC => {
                let operand = self.fetch(system, mode);

                let result = self.a & operand.data;

                self.write_carry_flag((result & 0x80) == 0x80);
                self.check_zero_and_negative_flag(result);
                self.a = result;
                1 + operand.cycle
            }
            Opcode::ARR => {
                let operand = self.fetch(system, mode);

                let v = self.a & operand.data;
                let result = v.wrapping_shr(1) | (
                    if self.read_carry_flag() { 0x80 } else { 0x00 }
                );

                self.write_carry_flag((result & 0x40) == 0x40);
                self.write_overflow_flag(((result & 0x40) ^ ((result & 0x20) << 1)) == 0x40);
                self.check_zero_and_negative_flag(result);
                self.a = result;
                1 + operand.cycle
            }
            Opcode::DCP => {
                let operand = self.fetch(system, mode);

                let v = operand.data.wrapping_sub(1);
                let result = self.a.wrapping_sub(v);

                self.write_carry_flag(self.a >= v);
                self.check_zero_and_negative_flag(result);
                system.write_u8(operand.address, v);
                3 + operand.cycle
            }
            Opcode::IGN => {
                let operand = self.fetch(system, mode);
                1 + operand.cycle
            }
            Opcode::LAX => {
                let operand = self.fetch(system, mode);
                let result = operand.data;

                self.check_zero_and_negative_flag(result);
                self.a = result;
                self.x = result;
                1 + operand.cycle
            }
            Opcode::SAX => {
                let operand = self.fetch(system, mode);
                let result = self.a & self.x;

                system.write_u8(operand.address, result);
                1 + operand.cycle
            }
            Opcode::SKB => {
                let operand = self.fetch(system, mode);
                1 + operand.cycle
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
    fn execute_adc_instruction()
    {
        let mut cpu = super::Cpu::default();
        let mut mem = memory::Memory::default();

        for param in [
            (0x00, 0x01, false, 0x01, false, false, false),
            (0x00, 0x00, false, 0x00, false, true, false),
            (0x80, 0x01, false, 0x81, false, false, true),
            (0x00, 0x01, true, 0x02, false, false, false),
            (0xff, 0x01, true, 0x01, true, false, false),
        ] {
            cpu.a  = param.0;
            cpu.pc = 0x0000u16;
            cpu.write_carry_flag(param.2);
            mem.write_u8(0x0000, 0x69);
            mem.write_u8(0x0001, param.1);

            let cycle = cpu.step(&mut mem);
            assert_eq!(cpu.a, param.3);
            assert_eq!(cpu.read_carry_flag(), param.4);
            assert_eq!(cpu.read_zero_flag(), param.5);
            assert_eq!(cpu.read_negative_flag(), param.6);
            assert_eq!(cycle, 0x02u8);
        }
    }

    # [test]
    fn execute_alr_instruction()
    {
        let mut cpu = super::Cpu::default();
        let mut mem = memory::Memory::default();

        for param in [
            (0xff, 0x02, 0x01, false, false, false),
            (0xff, 0x01, 0x00, true, true, false),
        ]{
            cpu.a  = param.0;
            cpu.pc = 0x0000u16;
            mem.write_u8(0x0000, 0x4b);
            mem.write_u8(0x0001, param.1);

            let cycle = cpu.step(&mut mem);
            assert_eq!(cpu.a, param.2);
            assert_eq!(cpu.read_carry_flag(), param.3);
            assert_eq!(cpu.read_zero_flag(), param.4);
            assert_eq!(cpu.read_negative_flag(), param.5);
            assert_eq!(cycle, 0x02u8);
        }
    }

    # [test]
    fn execute_anc_instruction()
    {
        let mut cpu = super::Cpu::default();
        let mut mem = memory::Memory::default();

        for param in [
            (0xff, 0x01, 0x01, false, false, false),
            (0xff, 0x00, 0x00, false, true, false),
            (0xff, 0x80, 0x80, true, false, true),
        ]{
            cpu.a  = param.0;
            cpu.pc = 0x0000u16;
            mem.write_u8(0x0000, 0x0b);
            mem.write_u8(0x0001, param.1);

            let cycle = cpu.step(&mut mem);
            assert_eq!(cpu.a, param.2);
            assert_eq!(cpu.read_carry_flag(), param.3);
            assert_eq!(cpu.read_zero_flag(), param.4);
            assert_eq!(cpu.read_negative_flag(), param.5);
            assert_eq!(cycle, 0x02u8);
        }
    }

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
    fn execute_arr_instruction()
    {
        let mut cpu = super::Cpu::default();
        let mut mem = memory::Memory::default();

        for param in [
            (0xff, 0x40, 0x20, false, false, false, false),
            (0xff, 0x80, 0x40, false, true, false, false),
            (0xff, 0x01, 0x00, false, false, true, false),
            (0xff, 0x40, 0xa0, true, false, false, true),
        ] {
            cpu.a  = param.0;
            cpu.pc = 0x0000u16;
            cpu.write_carry_flag(param.3);
            mem.write_u8(0x0000, 0x6b);
            mem.write_u8(0x0001, param.1);

            let cycle = cpu.step(&mut mem);
            assert_eq!(cpu.a, param.2);
            assert_eq!(cpu.read_carry_flag(), param.4);
            assert_eq!(cpu.read_zero_flag(), param.5);
            assert_eq!(cpu.read_negative_flag(), param.6);
            assert_eq!(cycle, 0x02u8);
        }
    }


    # [test]
    fn execute_asl_instruction()
    {
        let mut cpu = super::Cpu::default();
        let mut mem = memory::Memory::default();

        for param in [
            (0x01, 0x02, false, false, false),
            (0x00, 0x00, false, true, false),
            (0x40, 0x80, false, false, true),
            (0x81, 0x02, true, false, false),
        ] {
            cpu.a  = param.0;
            cpu.pc = 0x0000u16;
            mem.write_u8(0x0000, 0x0a);

            let cycle = cpu.step(&mut mem);
            assert_eq!(cpu.a, param.1);
            assert_eq!(cpu.read_carry_flag(), param.2);
            assert_eq!(cpu.read_zero_flag(), param.3);
            assert_eq!(cpu.read_negative_flag(), param.4);
            assert_eq!(cycle, 0x02u8);
        }

        for param in [
            (0x01, 0x02, false, false, false),
            (0x00, 0x00, false, true, false),
            (0x40, 0x80, false, false, true),
            (0x81, 0x02, true, false, false),
        ]{
            cpu.pc = 0x0000u16;
            mem.write_u8(0x0000, 0x06);
            mem.write_u8(0x0001, 0x0f);
            mem.write_u8(0x000f, param.0);

            let cycle = cpu.step(&mut mem);
            assert_eq!(mem.read_u8(0x000f), param.1);
            assert_eq!(cpu.read_carry_flag(), param.2);
            assert_eq!(cpu.read_zero_flag(), param.3);
            assert_eq!(cpu.read_negative_flag(), param.4);
            assert_eq!(cycle, 0x05u8);
        }
    }

    # [test]
    fn execute_bcc_instruction()
    {
        let mut cpu = super::Cpu::default();
        let mut mem = memory::Memory::default();

        for param in [
            (false, 0x0012u16, 0x03u8),
            (true, 0x0002u16, 0x02u8),
        ] {
            cpu.pc = 0x0000u16;
            cpu.write_carry_flag(param.0);
            mem.write_u8(0x0000, 0x90u8);
            mem.write_u8(0x0001, 0x10u8);
            mem.write_u8(0x0012, 0xffu8);

            let cycle = cpu.step(&mut mem);
            assert_eq!(cpu.pc, param.1);
            assert_eq!(cycle, param.2);
        }
    }

    # [test]
    fn execute_bcs_instruction()
    {
        let mut cpu = super::Cpu::default();
        let mut mem = memory::Memory::default();

        for param in [
            (false, 0x0002u16, 0x02u8),
            (true, 0x0012u16, 0x03u8),
        ] {
            cpu.pc = 0x0000u16;
            cpu.write_carry_flag(param.0);
            mem.write_u8(0x0000, 0xb0u8);
            mem.write_u8(0x0001, 0x10u8);
            mem.write_u8(0x0012, 0xffu8);

            let cycle = cpu.step(&mut mem);
            assert_eq!(cpu.pc, param.1);
            assert_eq!(cycle, param.2);
        }
    }

    # [test]
    fn execute_beq_instruction()
    {
        let mut cpu = super::Cpu::default();
        let mut mem = memory::Memory::default();

        for param in [
            (false, 0x0002u16, 0x02u8),
            (true, 0x0012u16, 0x03u8),
        ] {
            cpu.pc = 0x0000u16;
            cpu.write_zero_flag(param.0);
            mem.write_u8(0x0000, 0xf0u8);
            mem.write_u8(0x0001, 0x10u8);
            mem.write_u8(0x0012, 0xffu8);

            let cycle = cpu.step(&mut mem);
            assert_eq!(cpu.pc, param.1);
            assert_eq!(cycle, param.2);
        }
    }

    # [test]
    fn execute_bit_instruction()
    {
        let mut cpu = super::Cpu::default();
        let mut mem = memory::Memory::default();

        for param in [
            (0x01, false, false, false),
            (0x40, true, false, false),
            (0x00, false, true, false),
            (0x80, false, false, true),
        ] {
            cpu.pc = 0x0000u16;
            mem.write_u8(0x0000, 0x24u8);
            mem.write_u8(0x0001, 0x02u8);
            mem.write_u8(0x0002, param.0);

            let cycle = cpu.step(&mut mem);
            assert_eq!(cpu.read_overflow_flag(), param.1);
            assert_eq!(cpu.read_zero_flag(), param.2);
            assert_eq!(cpu.read_negative_flag(), param.3);
            assert_eq!(cycle, 0x03u8);
        }
    }

    # [test]
    fn execute_bmi_instruction()
    {
        let mut cpu = super::Cpu::default();
        let mut mem = memory::Memory::default();

        for param in [
            (false, 0x0002u16, 0x02u8),
            (true, 0x0012u16, 0x03u8),
        ] {
            cpu.pc = 0x0000u16;
            cpu.write_negative_flag(param.0);
            mem.write_u8(0x0000, 0x30u8);
            mem.write_u8(0x0001, 0x10u8);
            mem.write_u8(0x0012, 0xffu8);

            let cycle = cpu.step(&mut mem);
            assert_eq!(cpu.pc, param.1);
            assert_eq!(cycle, param.2);
        }
    }

    # [test]
    fn execute_bne_instruction()
    {
        let mut cpu = super::Cpu::default();
        let mut mem = memory::Memory::default();

        for param in [
            (false, 0x0012u16, 0x03u8),
            (true, 0x0002u16, 0x02u8),
        ] {
            cpu.pc = 0x0000u16;
            cpu.write_zero_flag(param.0);
            mem.write_u8(0x0000, 0xd0u8);
            mem.write_u8(0x0001, 0x10u8);
            mem.write_u8(0x0012, 0xffu8);

            let cycle = cpu.step(&mut mem);
            assert_eq!(cpu.pc, param.1);
            assert_eq!(cycle, param.2);
        }
    }

    # [test]
    fn execute_bpl_instruction()
    {
        let mut cpu = super::Cpu::default();
        let mut mem = memory::Memory::default();

        for param in [
            (false, 0x0012u16, 0x03u8),
            (true, 0x0002u16, 0x02u8),
        ] {
            cpu.pc = 0x0000u16;
            cpu.write_negative_flag(param.0);
            mem.write_u8(0x0000, 0x10u8);
            mem.write_u8(0x0001, 0x10u8);
            mem.write_u8(0x0012, 0xffu8);

            let cycle = cpu.step(&mut mem);
            assert_eq!(cpu.pc, param.1);
            assert_eq!(cycle, param.2);
        }
    }

    # [test]
    fn execute_brk_instruction()
    {
        let mut cpu = super::Cpu::default();
        let mut mem = memory::Memory::default();

        cpu.p  = 0x82u8;
        cpu.sp = 0x00ffu16;
        cpu.pc = 0x0080u16;
        mem.write_u8(0x0080, 0x00u8);
        mem.write_u8(0xfffe, 0x34u8);
        mem.write_u8(0xffff, 0x12u8);

        let cycle = cpu.step(&mut mem);
        assert_eq!(cpu.pc, 0x1234);
        assert_eq!(mem.read_u8(0x00ff), 0x00);
        assert_eq!(mem.read_u8(0x00fe), 0x82);
        assert_eq!(mem.read_u8(0x00fd), 0x92);
        assert_eq!(cpu.read_interrupt_flag(), true);
        assert_eq!(cycle, 7);
    }

        # [test]
    fn execute_bvc_instruction()
    {
        let mut cpu = super::Cpu::default();
        let mut mem = memory::Memory::default();

        for param in [
            (false, 0x0012u16, 0x03u8),
            (true, 0x0002u16, 0x02u8),
        ] {
            cpu.pc = 0x0000u16;
            cpu.write_overflow_flag(param.0);
            mem.write_u8(0x0000, 0x50u8);
            mem.write_u8(0x0001, 0x10u8);
            mem.write_u8(0x0012, 0xffu8);

            let cycle = cpu.step(&mut mem);
            assert_eq!(cpu.pc, param.1);
            assert_eq!(cycle, param.2);
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
    fn execute_cmp_instruction()
    {
        let mut cpu = super::Cpu::default();
        let mut mem = memory::Memory::default();

        for param in [
            (0x01, 0x02, false, false, true),
            (0x01, 0x01, true, true, false),
            (0x02, 0x01, true, false, false),
        ] {
            cpu.a  = param.0;
            cpu.pc = 0x0000u16;
            mem.write_u8(0x0000, 0xc9u8);
            mem.write_u8(0x0001, param.1);

            let cycle = cpu.step(&mut mem);
            assert_eq!(cpu.read_carry_flag(), param.2);
            assert_eq!(cpu.read_zero_flag(), param.3);
            assert_eq!(cpu.read_negative_flag(), param.4);
            assert_eq!(cycle, 0x02u8);
        }
    }

    # [test]
    fn execute_cpx_instruction()
    {
        let mut cpu = super::Cpu::default();
        let mut mem = memory::Memory::default();

        for param in [
            (0x01, 0x02, false, false, true),
            (0x01, 0x01, true, true, false),
            (0x02, 0x01, true, false, false),
        ] {
            cpu.x  = param.0;
            cpu.pc = 0x0000u16;
            mem.write_u8(0x0000, 0xe0u8);
            mem.write_u8(0x0001, param.1);

            let cycle = cpu.step(&mut mem);
            assert_eq!(cpu.read_carry_flag(), param.2);
            assert_eq!(cpu.read_zero_flag(), param.3);
            assert_eq!(cpu.read_negative_flag(), param.4);
            assert_eq!(cycle, 0x02u8);
        }
    }

    # [test]
    fn execute_cpy_instruction()
    {
        let mut cpu = super::Cpu::default();
        let mut mem = memory::Memory::default();

        for param in [
            (0x01, 0x02, false, false, true),
            (0x01, 0x01, true, true, false),
            (0x02, 0x01, true, false, false),
        ] {
            cpu.y  = param.0;
            cpu.pc = 0x0000u16;
            mem.write_u8(0x0000, 0xc0u8);
            mem.write_u8(0x0001, param.1);

            let cycle = cpu.step(&mut mem);
            assert_eq!(cpu.read_carry_flag(), param.2);
            assert_eq!(cpu.read_zero_flag(), param.3);
            assert_eq!(cpu.read_negative_flag(), param.4);
            assert_eq!(cycle, 0x02u8);
        }
    }

    # [test]
    fn execute_dcp_instruction()
    {
        let mut cpu = super::Cpu::default();
        let mut mem = memory::Memory::default();

        for param in [
            (0x01, 0x08, 0x07, false, false, true),
            (0x00, 0x01, 0x00, true, true, false),
        ] {
            cpu.a  = param.0;
            cpu.pc = 0x0000u16;
            mem.write_u8(0x0000, 0xc7u8);
            mem.write_u8(0x0001, 0x02u8);
            mem.write_u8(0x0002, param.1);

            let cycle = cpu.step(&mut mem);
            assert_eq!(mem.read_u8(0x0002), param.2);
            assert_eq!(cpu.read_carry_flag(), param.3);
            assert_eq!(cpu.read_zero_flag(), param.4);
            assert_eq!(cpu.read_negative_flag(), param.5);
        }
    }


    # [test]
    fn execute_dec_instruction()
    {
        let mut cpu = super::Cpu::default();
        let mut mem = memory::Memory::default();

        for param in [
            (0x02, 0x01, false, false),
            (0x01, 0x00, true, false),
            (0x81, 0x80, false, true),
        ] {
            cpu.pc = 0x0000u16;
            mem.write_u8(0x0000, 0xc6);
            mem.write_u8(0x0001, 0x0002);
            mem.write_u8(0x0002, param.0);

            let cycle = cpu.step(&mut mem);
            assert_eq!(mem.read_u8(0x0002), param.1);
            assert_eq!(cpu.read_zero_flag(), param.2);
            assert_eq!(cpu.read_negative_flag(), param.3);
            assert_eq!(cycle, 0x05u8);
        }
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
    fn execute_inc_instruction()
    {
        let mut cpu = super::Cpu::default();
        let mut mem = memory::Memory::default();

        for param in [
            (0x00, 0x01, false, false),
            (0xff, 0x00, true, false),
            (0x7f, 0x80, false, true),
        ] {
            cpu.pc = 0x0000u16;
            mem.write_u8(0x0000, 0xe6);
            mem.write_u8(0x0001, 0x0002);
            mem.write_u8(0x0002, param.0);

            let cycle = cpu.step(&mut mem);
            assert_eq!(mem.read_u8(0x0002), param.1);
            assert_eq!(cpu.read_zero_flag(), param.2);
            assert_eq!(cpu.read_negative_flag(), param.3);
            assert_eq!(cycle, 0x05u8);
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
    fn execute_jmp_instruction()
    {
        let mut cpu = super::Cpu::default();
        let mut mem = memory::Memory::default();

        cpu.pc = 0x0000u16;
        mem.write_u8(0x0000, 0x4cu8);
        mem.write_u8(0x0001, 0x34u8);
        mem.write_u8(0x0002, 0x12u8);
        mem.write_u8(0x1234, 0xffu8);

        let cycle = cpu.step(&mut mem);
        assert_eq!(cpu.pc, 0x1234);
        assert_eq!(cycle, 0x03u8);
    }

    # [test]
    fn execute_jsr_instruction()
    {
        let mut cpu = super::Cpu::default();
        let mut mem = memory::Memory::default();

        cpu.sp = 0x00ffu16;
        cpu.pc = 0x0000u16;
        mem.write_u8(0x0000, 0x20u8);
        mem.write_u8(0x0001, 0x34u8);
        mem.write_u8(0x0002, 0x12u8);
        mem.write_u8(0x1234, 0xffu8);

        let cycle = cpu.step(&mut mem);
        assert_eq!(cpu.pc, 0x1234);
        assert_eq!(mem.read_u8(0xff), 0x00);
        assert_eq!(mem.read_u8(0xfe), 0x02);
        assert_eq!(cycle, 0x06u8);
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
    fn execute_lsr_instruction()
    {
        let mut cpu = super::Cpu::default();
        let mut mem = memory::Memory::default();

        for param in [
            (0x02, 0x01, false, false, false),
            (0x01, 0x00, true, true, false),
        ] {
            cpu.a  = param.0;
            cpu.pc = 0x0000u16;
            mem.write_u8(0x0000, 0x4a);

            let cycle = cpu.step(&mut mem);
            assert_eq!(cpu.a, param.1);
            assert_eq!(cpu.read_carry_flag(), param.2);
            assert_eq!(cpu.read_zero_flag(), param.3);
            assert_eq!(cpu.read_negative_flag(), param.4);
            assert_eq!(cycle, 0x02u8);
        }

        for param in [
            (0x02, 0x01, false, false, false),
            (0x01, 0x00, true, true, false),
        ]{
            cpu.pc = 0x0000u16;
            mem.write_u8(0x0000, 0x46);
            mem.write_u8(0x0001, 0x0f);
            mem.write_u8(0x000f, param.0);

            let cycle = cpu.step(&mut mem);
            assert_eq!(mem.read_u8(0x000f), param.1);
            assert_eq!(cpu.read_carry_flag(), param.2);
            assert_eq!(cpu.read_zero_flag(), param.3);
            assert_eq!(cpu.read_negative_flag(), param.4);
            assert_eq!(cycle, 0x05u8);

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
    fn execute_pha_instruction()
    {
        let mut cpu = super::Cpu::default();
        let mut mem = memory::Memory::default();

        cpu.a  = 0x80u8;
        cpu.pc = 0x0000u16;
        cpu.sp = 0x00ffu16;
        mem.write_u8(0x0000, 0x48u8);

        let cycle = cpu.step(&mut mem);
        assert_eq!(mem.read_u8(0xff), 0x80u8);
        assert_eq!(cpu.sp, 0x00fe);
        assert_eq!(cycle, 0x03u8);
    }

    # [test]
    fn execute_php_instruction()
    {
        let mut cpu = super::Cpu::default();
        let mut mem = memory::Memory::default();

        cpu.p  = 0x80u8;
        cpu.pc = 0x0000u16;
        cpu.sp = 0x00ffu16;
        mem.write_u8(0x0000, 0x08u8);

        let cycle = cpu.step(&mut mem);
        assert_eq!(mem.read_u8(0xff), 0x80u8);
        assert_eq!(cpu.sp, 0x00fe);
        assert_eq!(cycle, 0x03u8);
    }

    # [test]
    fn execute_pla_instruction()
    {
        let mut cpu = super::Cpu::default();
        let mut mem = memory::Memory::default();

        for param in [
            (0x01, false, false),
            (0x00, true, false),
            (0x80, false, true),
        ] {
            cpu.a = 0x00u8;
            cpu.pc = 0x0000u16;
            cpu.sp = 0x00feu16;
            mem.write_u8(0x0000, 0x68u8);
            mem.write_u8(0x00ff, param.0);

            let cycle = cpu.step(&mut mem);
            assert_eq!(cpu.a, param.0);
            assert_eq!(cpu.sp, 0x00ffu16);
            assert_eq!(cpu.read_zero_flag(), param.1);
            assert_eq!(cpu.read_negative_flag(), param.2);
            assert_eq!(cycle, 0x04u8);
        }
    }

    # [test]
    fn execute_plp_instruction()
    {
        let mut cpu = super::Cpu::default();
        let mut mem = memory::Memory::default();

        cpu.p  = 0x00u8;
        cpu.pc = 0x0000u16;
        cpu.sp = 0x00feu16;
        mem.write_u8(0x0000, 0x28u8);
        mem.write_u8(0x00ff, 0x90u8);

        let cycle = cpu.step(&mut mem);
        assert_eq!(cpu.p, 0x90u8);
        assert_eq!(cpu.sp, 0x00ffu16);
        assert_eq!(cycle, 0x04u8);
    }

    # [test]
    fn execute_rol_instruction()
    {
        let mut cpu = super::Cpu::default();
        let mut mem = memory::Memory::default();

        for param in [
            (0x01, 0x02, false, false, false, false),
            (0x80, 0x00, false, true, true, false),
            (0x80, 0x01, true, true, false, false),
            (0x40, 0x80, false, false, false, true),
        ] {
            cpu.a = param.0;
            cpu.pc = 0x0000u16;
            cpu.write_carry_flag(param.2);
            mem.write_u8(0x0000, 0x2a);

            let cycle = cpu.step(&mut mem);
            assert_eq!(cpu.a, param.1);
            assert_eq!(cpu.read_carry_flag(), param.3);
            assert_eq!(cpu.read_zero_flag(), param.4);
            assert_eq!(cpu.read_negative_flag(), param.5);
            assert_eq!(cycle, 0x02u8);
        }

        for param in [
            (0x01, 0x02, false, false, false, false),
            (0x80, 0x00, false, true, true, false),
            (0x80, 0x01, true, true, false, false),
            (0x40, 0x80, false, false, false, true),
        ] {
            cpu.pc = 0x0000u16;
            cpu.write_carry_flag(param.2);
            mem.write_u8(0x0000, 0x26);
            mem.write_u8(0x0001, 0x0f);
            mem.write_u8(0x000f, param.0);

            let cycle = cpu.step(&mut mem);
            assert_eq!(mem.read_u8(0x000f), param.1);
            assert_eq!(cpu.read_carry_flag(), param.3);
            assert_eq!(cpu.read_zero_flag(), param.4);
            assert_eq!(cpu.read_negative_flag(), param.5);
            assert_eq!(cycle, 0x05u8);
        }
    }

    # [test]
    fn execute_ror_instruction()
    {
        let mut cpu = super::Cpu::default();
        let mut mem = memory::Memory::default();

        for param in [
            (0x02, 0x01, false, false, false, false),
            (0x01, 0x00, false, true, true, false),
            (0x01, 0x80, true, true, false, true),
        ] {
            cpu.a = param.0;
            cpu.pc = 0x0000u16;
            cpu.write_carry_flag(param.2);
            mem.write_u8(0x0000, 0x6a);

            let cycle = cpu.step(&mut mem);
            assert_eq!(cpu.a, param.1);
            assert_eq!(cpu.read_carry_flag(), param.3);
            assert_eq!(cpu.read_zero_flag(), param.4);
            assert_eq!(cpu.read_negative_flag(), param.5);
            assert_eq!(cycle, 0x02u8);
        }

        for param in [
            (0x02, 0x01, false, false, false, false),
            (0x01, 0x00, false, true, true, false),
            (0x01, 0x80, true, true, false, true),
        ] {
            cpu.pc = 0x0000u16;
            cpu.write_carry_flag(param.2);
            mem.write_u8(0x0000, 0x66);
            mem.write_u8(0x0001, 0x0f);
            mem.write_u8(0x000f, param.0);

            let cycle = cpu.step(&mut mem);
            assert_eq!(mem.read_u8(0x000f), param.1);
            assert_eq!(cpu.read_carry_flag(), param.3);
            assert_eq!(cpu.read_zero_flag(), param.4);
            assert_eq!(cpu.read_negative_flag(), param.5);
            assert_eq!(cycle, 0x05u8);
        }
    }

    # [test]
    fn execute_rti_instruction()
    {
        let mut cpu = super::Cpu::default();
        let mut mem = memory::Memory::default();

        cpu.p  = 0x00u8;
        cpu.sp = 0x00fcu16;
        cpu.pc = 0x0000u16;
        mem.write_u8(0x0000, 0x40u8);
        mem.write_u8(0x00ff, 0x12u8);
        mem.write_u8(0x00fe, 0x34u8);
        mem.write_u8(0x00fd, 0x82u8);

        let cycle = cpu.step(&mut mem);

        assert_eq!(cpu.p, 0x82u8);
        assert_eq!(cpu.pc, 0x1234);
        assert_eq!(cycle, 6);
    }

    # [test]
    fn execute_rts_instruction()
    {
        let mut cpu = super::Cpu::default();
        let mut mem = memory::Memory::default();

        cpu.sp = 0x00fdu16;
        cpu.pc = 0x0000u16;
        mem.write_u8(0x0000, 0x60u8);
        mem.write_u8(0x00ff, 0x12u8);
        mem.write_u8(0x00fe, 0x34u8);

        let cycle = cpu.step(&mut mem);
        assert_eq!(cpu.pc, 0x1235);
        assert_eq!(cycle, 6);
    }


    # [test]
    fn execute_sax_instruction()
    {
        let mut cpu = super::Cpu::default();
        let mut mem = memory::Memory::default();

        cpu.a  = 0xffu8;
        cpu.x  = 0x01u8;
        cpu.pc = 0x0000u16;
        mem.write_u8(0x0000, 0x87u8);
        mem.write_u8(0x0001, 0x02u8);
        mem.write_u8(0x0002, 0xffu8);

        let cycle = cpu.step(&mut mem);
        assert_eq!(mem.read_u8(0x02), 0x01);
        assert_eq!(cycle, 3);
    }


    # [test]
    fn execute_sbc_instruction()
    {
        let mut cpu = super::Cpu::default();
        let mut mem = memory::Memory::default();

        for param in [
            (0x03, 0x01, false, 0x01, true, false, false),
            (0x02, 0x01, false, 0x00, true, true, false),
            (0x82, 0x01, false, 0x80, true, false, true),
        ] {
            cpu.a  = param.0;
            cpu.pc = 0x0000u16;
            cpu.write_carry_flag(param.2);
            mem.write_u8(0x0000, 0xe9);
            mem.write_u8(0x0001, param.1);

            let cycle = cpu.step(&mut mem);
            assert_eq!(cpu.a, param.3);
            assert_eq!(cpu.read_carry_flag(), param.4);
            assert_eq!(cpu.read_zero_flag(), param.5);
            assert_eq!(cpu.read_negative_flag(), param.6);
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

    # [test]
    fn execute_ign_instruction()
    {
        let mut cpu = super::Cpu::default();
        let mut mem = memory::Memory::default();

        for param in [
            (0x04, 0x0002, 0x03),
            (0x14, 0x0002, 0x04),
            (0x0c, 0x0003, 0x04),
            (0x1c, 0x0003, 0x04),
        ] {
            cpu.pc = 0x0000u16;
            mem.write_u8(0x0000, param.0);

            let cycle = cpu.step(&mut mem);
            assert_eq!(cpu.pc, param.1);
            assert_eq!(cycle, param.2);
        }
    }

    # [test]
    fn execute_lax_instruction()
    {
        let mut cpu = super::Cpu::default();
        let mut mem = memory::Memory::default();

        for param in [
            (0x00, 0x01, false, false),
            (0x01, 0x00, true, false),
            (0x00, 0x80, false, true),
        ] {
            cpu.a  = param.0;
            cpu.x  = param.0;
            cpu.pc = 0x0000u16;
            mem.write_u8(0x0000, 0xa7u8);
            mem.write_u8(0x0001, 0x02u8);
            mem.write_u8(0x0002, param.1);

            let cycle = cpu.step(&mut mem);
            assert_eq!(cpu.a, param.1);
            assert_eq!(cpu.x, param.1);
            assert_eq!(cpu.read_zero_flag(), param.2);
            assert_eq!(cpu.read_negative_flag(), param.3);
            assert_eq!(cycle, 0x03u8);
        }
    }

    # [test]
    fn execute_skb_instruction()
    {
        let mut cpu = super::Cpu::default();
        let mut mem = memory::Memory::default();

        cpu.pc = 0x0000u16;
        mem.write_u8(0x0000, 0x80u8);

        let cycle = cpu.step(&mut mem);
        assert_eq!(cpu.pc, 0x0002u16);
        assert_eq!(cycle, 0x02u8);
    }
}