use crate::{Memory, PPU_REGISTER_BASE_ADDRESS, APU_IO_REGISTER_BASE_ADDRESS};

pub trait SystemBus {
    fn read_u8(&mut self, address: u16) -> u8;
    fn write_u8(&mut self, address: u16, data: u8);
}

impl SystemBus for Memory {
    fn read_u8(&mut self, address: u16) -> u8 {
        if address < PPU_REGISTER_BASE_ADDRESS {
            let index = usize::from(address) % self.ram.len();
            return self.ram[index];
        }

        if address < APU_IO_REGISTER_BASE_ADDRESS {
            let index = usize::from(address - PPU_REGISTER_BASE_ADDRESS) % self.ppu_registers.len();
            let value = match index {
                0x04 => {
                    self.request_to_read_oam_data = true;
                    self.ppu_registers[index]
                },
                0x07 => {
                    self.request_to_read_ppu_data = true;
                    self.ppu_registers[index]
                }
                _ =>  todo!("{}", index),
            };
            return value;
        }

        // TODO: Read from the contents of Rom.
        //       BRK test fails if 0 is set as a fixed value
        let index = usize::from(address) % self.ram.len();
        return self.ram[index];
    }

    fn write_u8(&mut self, address: u16, data: u8) {
        if address < PPU_REGISTER_BASE_ADDRESS {
            let index = usize::from(address) % self.ram.len();
            self.ram[index] = data;
            return;
        }

        if address < APU_IO_REGISTER_BASE_ADDRESS {
            let index = usize::from(address - PPU_REGISTER_BASE_ADDRESS) % self.ppu_registers.len();
            match index {
                0x04 => {
                    self.request_to_write_oam_data = true;
                    self.ppu_registers[index] = data;
                },
                0x06 => {
                    if self.is_second_write {
                        self.ppu_register_address_lower = data;
                        self.request_to_write_ppu_address = true;
                        self.is_second_write = false;
                    } else {
                        self.ppu_registers[index] = data;
                        self.is_second_write = true;
                    }
                },
                0x07 => {
                    self.ppu_registers[index] = data;
                    self.request_to_write_ppu_data = true;
                }
                _ =>  todo!("{}", index),
            };
            return;
        }

        // TODO: Write to the contents of Rom.
        //       If left unimplemented, the BRK test will fail.
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

        mem.write_u8(0x2004u16, 0xffu8);
        assert_eq!(mem.read_u8(0x2004u16), 0xffu8);
        assert!(mem.request_to_read_oam_data);
        assert!(mem.request_to_write_oam_data);

        mem.is_second_write = true;
        mem.write_u8(0x2006u16, 0x12u8);
        assert_eq!(mem.ppu_register_address_lower, 0x12u8);
        assert_eq!(mem.request_to_write_ppu_address, true);
        assert_eq!(mem.is_second_write, false);
        mem.write_u8(0x2006u16, 0x34u8);
        assert_eq!(mem.ppu_registers[0x06], 0x34u8);
        assert_eq!(mem.is_second_write, true);

        mem.write_u8(0x2007u16, 0x56u8);
        assert_eq!(mem.read_u8(0x2007u16), 0x56u8);
        assert_eq!(mem.request_to_read_ppu_data, true);
        assert_eq!(mem.request_to_write_ppu_data, true);
    }
}