mod instruction;

use memory::system::SystemBus;
use memory::Memory;

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
    fn fetch_u8(&mut self, system: &mut memory::system::SystemBus) -> u8 {
        let v = system.read_u8(self.pc);
        self.pc += 1;
        v
    }
}

mod tests {
    use memory::system::SystemBus;

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
}