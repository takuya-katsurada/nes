use crate::Cpu;
use crate::instruction::AddressingMode;

#[derive(Copy, Clone, Debug)]
pub struct Operand {
    pub address: u16,
    pub data: u8,
    pub cycle: u8,
}
const IMPLIED: Operand = Operand { address: 0, data: 0, cycle: 0 };
const ACCUMULATOR: Operand = Operand { address: 0, data: 0, cycle: 1 };

impl Cpu {
    pub(crate) fn fetch_u8(&mut self, system: &mut memory::system::SystemBus) -> u8 {
        let v = system.read_u8(self.pc);
        self.pc += 1;
        v
    }

    pub(crate) fn fetch_u16(&mut self, system: &mut memory::system::SystemBus) -> u16 {
        let lo = self.fetch_u8(system);
        let hi = self.fetch_u8(system);
        u16::from(lo) | (u16::from(hi) << 8)
    }


    pub(crate) fn fetch(&mut self, system: &mut memory::system::SystemBus, mode : AddressingMode) -> Operand {
        match mode {
            AddressingMode::Implied => IMPLIED,
            AddressingMode::Accumulator => ACCUMULATOR,
            AddressingMode::Immediate => {
                let address = self.pc;
                Operand { address, data: self.fetch_u8(system), cycle: 1 }
            }
            AddressingMode::ZeroPage => {
                let address = u16::from(self.fetch_u8(system));
                Operand { address, data: system.read_u8(address), cycle: 2 }
            }
            AddressingMode::ZeroPageX => {
                let address = u16::from(self.fetch_u8(system).wrapping_add(self.x));
                Operand { address, data: system.read_u8(address), cycle: 3 }
            }
            AddressingMode::ZeroPageY => {
                let address = u16::from(self.fetch_u8(system).wrapping_add(self.y));
                Operand { address, data: system.read_u8(address), cycle: 3 }
            }
            AddressingMode::Absolute => {
                let address = self.fetch_u16(system);
                Operand { address, data: system.read_u8(address), cycle: 3 }
            }
            AddressingMode::AbsoluteX => {
                let address = self.fetch_u16(system).wrapping_add(u16::from(self.x));
                let additional_cycle =
                    if (address & 0xff00u16) != (address.wrapping_add(u16::from(self.x)) & 0xff00u16) {
                        1
                    } else {
                        0
                    };
                Operand { address, data: system.read_u8(address), cycle: 3 + additional_cycle }
            }
            AddressingMode::AbsoluteY => {
                let address = self.fetch_u16(system).wrapping_add(u16::from(self.y));
                let additional_cycle =
                    if (address & 0xff00u16) != (address.wrapping_add(u16::from(self.y)) & 0xff00u16) {
                        1
                    } else {
                        0
                    };
                Operand { address, data: system.read_u8(address), cycle: 3 + additional_cycle }
            }
            AddressingMode::Indirect => {
                // 6502 bug, so the low byte is not wrapped and the high byte is not incremented.
                let s1 = self.fetch_u8(system);
                let s2 = self.fetch_u8(system);

                let d1 = u16::from(s1) | (u16::from(s2) << 8);
                let d2 = u16::from(s1.wrapping_add(1)) | (u16::from(s2) << 8);

                let lo = u16::from(system.read_u8(d1));
                let hi = u16::from(system.read_u8(d2));

                let address = lo | hi << 8;
                Operand { address, data: system.read_u8(address), cycle: 5 }
            }
            AddressingMode::IndirectX => {
                // 6502 bug, so the low byte is not wrapped and the high byte is not incremented.
                let s = self.fetch_u8(system).wrapping_add(self.x);

                let lo = u16::from(system.read_u8(u16::from(s)));
                let hi = u16::from(system.read_u8(u16::from(s.wrapping_add(1))));

                let address = lo | hi << 8;
                Operand { address, data: system.read_u8(address), cycle: 5 }
            }
            AddressingMode::IndirectY => {
                // 6502 bug, so the low byte is not wrapped and the high byte is not incremented.
                let s = self.fetch_u8(system);

                let lo = u16::from(system.read_u8(u16::from(s)));
                let hi = u16::from(system.read_u8(u16::from(s.wrapping_add(1))));

                let base = lo | hi << 8;
                let address = base.wrapping_add(u16::from(self.y));
                let additional_cycle = if (base & 0xff00u16) != (address & 0xff00u16) {
                    1
                } else {
                    0
                };
                Operand { address, data: system.read_u8(address), cycle: 4 + additional_cycle }
            }

            _ => panic!("not implemented")
        }
    }
}

mod tests {
    use memory::system::SystemBus;
    use crate::instruction::AddressingMode;

    # [test]
    fn test_fetch_u8() {
        let mut cpu = super::Cpu::default();
        let mut mem = memory::Memory::default();

        cpu.pc = 0x0002u16;
        mem.write_u8(0x0002u16, 0xf0u8);

        let v = cpu.fetch_u8(&mut mem);
        assert_eq!(v, 0xf0u8);
        assert_eq!(cpu.pc, 0x0003u16);
    }

    # [test]
    fn test_fetch_u16() {
        let mut cpu = super::Cpu::default();
        let mut mem = memory::Memory::default();

        cpu.pc = 0x0002u16;
        mem.write_u8(0x0002u16, 0x34u8);
        mem.write_u8(0x0003u16, 0x12u8);

        let v = cpu.fetch_u16(&mut mem);
        assert_eq!(v, 0x1234u16);
        assert_eq!(cpu.pc, 0x0004u16);
    }

    # [test]
    fn test_fetch_as_implied() {
        let mut cpu = super::Cpu::default();
        let mut mem = memory::Memory::default();

        let v = cpu.fetch(&mut mem, AddressingMode::Implied);
        assert_eq!(v.address, 0x0000u16);
        assert_eq!(v.data, 0x00u8);
        assert_eq!(v.cycle, 0x00u8);
    }

    # [test]
    fn test_fetch_as_accumulator() {
        let mut cpu = super::Cpu::default();
        let mut mem = memory::Memory::default();

        let v = cpu.fetch(&mut mem, AddressingMode::Accumulator);
        assert_eq!(v.address, 0x0000u16);
        assert_eq!(v.data, 0x00u8);
        assert_eq!(v.cycle, 0x01u8);
    }

    # [test]
    fn test_fetch_as_immediate() {
        let mut cpu = super::Cpu::default();
        let mut mem = memory::Memory::default();

        cpu.pc = 0x0002u16;
        mem.write_u8(0x0002u16, 0xffu8);

        let v = cpu.fetch(&mut mem, AddressingMode::Immediate);
        assert_eq!(v.address, 0x0002u16);
        assert_eq!(v.data, 0xffu8);
        assert_eq!(v.cycle, 0x01u8);
        assert_eq!(cpu.pc, 0x0003u16);
    }

    # [test]
    fn test_fetch_as_zero_page() {
        let mut cpu = super::Cpu::default();
        let mut mem = memory::Memory::default();

        cpu.pc = 0x0002u16;
        mem.write_u8(0x0002u16, 0x42u8);
        mem.write_u8(0x0042u16, 0xeeu8);

        let v = cpu.fetch(&mut mem, AddressingMode::ZeroPage);
        assert_eq!(v.address, 0x0042u16);
        assert_eq!(v.data, 0xeeu8);
        assert_eq!(v.cycle, 0x02u8);
        assert_eq!(cpu.pc, 0x0003u16);
    }

    # [test]
    fn test_fetch_as_zero_page_x() {
        let mut cpu = super::Cpu::default();
        let mut mem = memory::Memory::default();

        cpu.x  = 0x03u8;
        cpu.pc = 0x0002u16;
        mem.write_u8(0x0002u16, 0x42u8);
        mem.write_u8(0x0042u16, 0xeeu8);
        mem.write_u8(0x0045u16, 0xaau8);

        let v = cpu.fetch(&mut mem, AddressingMode::ZeroPageX);
        assert_eq!(v.address, 0x0045u16);
        assert_eq!(v.data, 0xaau8);
        assert_eq!(v.cycle, 0x03u8);
        assert_eq!(cpu.pc, 0x0003u16);
    }

    # [test]
    fn test_fetch_as_zero_page_y() {
        let mut cpu = super::Cpu::default();
        let mut mem = memory::Memory::default();

        cpu.y  = 0x03u8;
        cpu.pc = 0x0002u16;
        mem.write_u8(0x0002u16, 0x42u8);
        mem.write_u8(0x0042u16, 0xeeu8);
        mem.write_u8(0x0045u16, 0xaau8);

        let v = cpu.fetch(&mut mem, AddressingMode::ZeroPageY);
        assert_eq!(v.address, 0x0045u16);
        assert_eq!(v.data, 0xaau8);
        assert_eq!(v.cycle, 0x03u8);
        assert_eq!(cpu.pc, 0x0003u16);
    }

    # [test]
    fn test_fetch_as_absolute() {
        let mut cpu = super::Cpu::default();
        let mut mem = memory::Memory::default();

        cpu.pc = 0x0002u16;
        mem.write_u8(0x0002u16, 0x42u8);
        mem.write_u8(0x0003u16, 0x16u8);
        mem.write_u8(0x1642u16, 0xbbu8);

        let v = cpu.fetch(&mut mem, AddressingMode::Absolute);
        assert_eq!(v.address, 0x1642u16);
        assert_eq!(v.data, 0xbbu8);
        assert_eq!(v.cycle, 0x03u8);
        assert_eq!(cpu.pc, 0x0004u16);
    }

    # [test]
    fn test_fetch_as_absolute_x() {
        let mut cpu = super::Cpu::default();
        let mut mem = memory::Memory::default();

        mem.write_u8(0x0002u16, 0x42u8);
        mem.write_u8(0x0003u16, 0x16u8);
        mem.write_u8(0x1642u16, 0xbbu8);
        mem.write_u8(0x1647u16, 0xddu8);
        mem.write_u8(0x16dcu16, 0xeeu8);

        for param in [
            (0x05u8, 0x1647u16, 0xddu8, 0x03u8),
            (0x9au8, 0x16dcu16, 0xeeu8, 0x04u8),
        ] {
            cpu.x = param.0;
            cpu.pc = 0x0002u16;

            let v = cpu.fetch(&mut mem, AddressingMode::AbsoluteX);
            assert_eq!(v.address, param.1);
            assert_eq!(v.data, param.2);
            assert_eq!(v.cycle, param.3);
            assert_eq!(cpu.pc, 0x0004u16);
        }
    }

    # [test]
    fn test_fetch_as_absolute_y() {
        let mut cpu = super::Cpu::default();
        let mut mem = memory::Memory::default();

        mem.write_u8(0x0002u16, 0x42u8);
        mem.write_u8(0x0003u16, 0x16u8);
        mem.write_u8(0x1642u16, 0xbbu8);
        mem.write_u8(0x1647u16, 0xddu8);
        mem.write_u8(0x16dcu16, 0xeeu8);

        for param in [
            (0x05u8, 0x1647u16, 0xddu8, 0x03u8),
            (0x9au8, 0x16dcu16, 0xeeu8, 0x04u8),
        ] {
            cpu.y = param.0;
            cpu.pc = 0x0002u16;

            let v = cpu.fetch(&mut mem, AddressingMode::AbsoluteY);
            assert_eq!(v.address, param.1);
            assert_eq!(v.data, param.2);
            assert_eq!(v.cycle, param.3);
            assert_eq!(cpu.pc, 0x0004u16);
        }
    }

    # [test]
    fn test_fetch_as_indirect() {
        let mut cpu = super::Cpu::default();
        let mut mem = memory::Memory::default();

        mem.write_u8(0x0002u16, 0xffu8);
        mem.write_u8(0x0003u16, 0x16u8);
        mem.write_u8(0x1234u16, 0x88u8);
        mem.write_u8(0x1600u16, 0x12u8);
        mem.write_u8(0x16ffu16, 0x34u8);

        cpu.pc = 0x0002u16;
        let v = cpu.fetch(&mut mem, AddressingMode::Indirect);
        assert_eq!(v.address, 0x1234u16);
        assert_eq!(v.data, 0x88u8);
        assert_eq!(v.cycle, 0x05u8);
        assert_eq!(cpu.pc, 0x0004u16);
    }

    # [test]
    fn test_fetch_as_indirect_x() {
        let mut cpu = super::Cpu::default();
        let mut mem = memory::Memory::default();

        mem.write_u8(0x0000u16, 0x12u8);
        mem.write_u8(0x0002u16, 0xfeu8);
        mem.write_u8(0x00ffu16, 0x34u8);
        mem.write_u8(0x1234u16, 0x66u8);

        cpu.x  = 0x01u8;
        cpu.pc = 0x0002u16;
        let v = cpu.fetch(&mut mem, AddressingMode::IndirectX);
        assert_eq!(v.address, 0x1234u16);
        assert_eq!(v.data, 0x66u8);
        assert_eq!(v.cycle, 0x05u8);
        assert_eq!(cpu.pc, 0x0003u16);
    }

    # [test]
    fn test_fetch_as_indirect_y() {
        let mut cpu = super::Cpu::default();
        let mut mem = memory::Memory::default();

        mem.write_u8(0x0000u16, 0x01u8);
        mem.write_u8(0x0002u16, 0xffu8);
        mem.write_u8(0x00ffu16, 0x80u8);
        mem.write_u8(0x0181u16, 0xaau8);
        mem.write_u8(0x027fu16, 0xbbu8);

        for param in [
            (0x01u8, 0x0181u16, 0xaau8, 0x04u8),
            (0xffu8, 0x027fu16, 0xbbu8, 0x05u8),
        ] {
            cpu.y = param.0;
            cpu.pc = 0x0002u16;

            let v = cpu.fetch(&mut mem, AddressingMode::IndirectY);
            assert_eq!(v.address, param.1);
            assert_eq!(v.data, param.2);
            assert_eq!(v.cycle, param.3);
            assert_eq!(cpu.pc, 0x0003u16);
        }
    }
}