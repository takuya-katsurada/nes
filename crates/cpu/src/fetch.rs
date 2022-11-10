use crate::Cpu;
use crate::instruction::AddressingMode;

#[derive(Copy, Clone, Debug)]
pub struct Operand {
    address: u16,
    data: u8,
    cycle: u16,
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
        assert_eq!(v.cycle, 0x0000u16);
    }

    # [test]
    fn test_fetch_as_accumulator() {
        let mut cpu = super::Cpu::default();
        let mut mem = memory::Memory::default();

        let v = cpu.fetch(&mut mem, AddressingMode::Accumulator);
        assert_eq!(v.address, 0x0000u16);
        assert_eq!(v.data, 0x00u8);
        assert_eq!(v.cycle, 0x0001u16);
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
        assert_eq!(v.cycle, 0x0001u16);
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
        assert_eq!(v.cycle, 0x0002u16);
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
        assert_eq!(v.cycle, 0x0003u16);
        assert_eq!(cpu.pc, 0x0003u16);
    }

}