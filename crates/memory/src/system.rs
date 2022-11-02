use crate::Memory;

pub trait SystemBus {
    fn read_u8(&self, address: u16) -> u8;
    fn write_u8(&mut self, address: u16, data: u8);
}

impl SystemBus for Memory {
    fn read_u8(&self, address: u16) -> u8 {
        let index = usize::from(address) % self.ram.len();
        return self.ram[index];
    }

    fn write_u8(&mut self, address: u16, data: u8) {
        let index = usize::from(address) % self.ram.len();
        self.ram[index] = data;
    }
}

#[cfg(test)]
mod tests {
    use crate::Memory;
    use crate::system::SystemBus;

    # [test]
    fn test_read_and_write_to_ram() {
        let mut mem = Memory::default();

        mem.write_u8(0x0000u16, 0xffu8);
        assert_eq!(mem.read_u8(0x0000u16), 0xffu8);
        assert_eq!(mem.read_u8(0x0800u16), 0xffu8);

        mem.write_u8(0x0801u16, 0x0fu8);
        assert_eq!(mem.read_u8(0x0001u16), 0x0fu8);
        assert_eq!(mem.read_u8(0x0801u16), 0x0fu8);
    }
}