use crate::Memory;

pub trait PpuRegistersController {
    // 0x2001: PPU MASK
    fn read_ppu_is_clip_sprite(&mut self) -> bool;
    fn read_ppu_is_write_bg(&mut self) -> bool;
    fn read_ppu_is_write_sprite(&mut self) -> bool;
    // 0x2003: OAM ADDR (Object Attribute Memory).
    fn read_oam_address(&mut self) -> u8;
    // 0x2004: OAM DATA (Object Attribute Memory).
    fn read_oam_data(&mut self) -> (u8, bool, bool);
    fn write_oam_data(&mut self, data: u8);
    // 0x2006: PPU ADDR.
    fn read_ppu_address(&mut self) -> (u16, bool);
    // $2007: PPU DATA
    fn read_ppu_data(&mut self) -> (u8, bool, bool);
    fn write_ppu_data(&mut self, data: u8);
}

impl PpuRegistersController for Memory {
    fn read_ppu_is_clip_sprite(&mut self) -> bool {
        (self.ppu_registers[0x01] & 0x04u8) == 0x04u8
    }

    fn read_ppu_is_write_bg(&mut self) -> bool {
        (self.ppu_registers[0x01] & 0x08u8) == 0x08u8
    }

    fn read_ppu_is_write_sprite(&mut self) -> bool {
        (self.ppu_registers[0x01] & 0x10u8) == 0x10u8
    }

    fn read_oam_address(&mut self) -> u8 {
        self.ppu_registers[0x03]
    }

    fn read_oam_data(&mut self) -> (u8, bool, bool) {
        let r = self.request_to_read_oam_data;
        let w = self.request_to_write_oam_data;
        let v = self.ppu_registers[0x04];

        match (r,w) {
            (_, true) => {
                self.request_to_write_oam_data = false;
                (v, false, true)
            },
            (true, _) => {
                self.request_to_read_oam_data = false;
                (v, true, false)
            },
            _ => (v, false, false),
        }
    }

    fn write_oam_data(&mut self, data: u8) {
        self.ppu_registers[0x04] = data;
    }

    fn read_ppu_address(&mut self) -> (u16, bool) {
        let hi = u16::from(self.ppu_registers[0x06]) << 8;
        let lo = u16::from(self.ppu_register_address_lower);
        let is_request = self.request_to_write_ppu_address;

        self.request_to_write_ppu_address = false;
        (lo|hi, is_request)
    }

    fn read_ppu_data(&mut self) -> (u8, bool, bool) {
        let r = self.request_to_read_ppu_data;
        let w = self.request_to_write_ppu_data;
        let v = self.ppu_registers[0x07];

        match (r,w) {
            (_, true) => {
                self.request_to_write_ppu_data = false;
                (v, false, true)
            },
            (true, _) => {
                self.request_to_read_ppu_data = false;
                (v, true, false)
            },
            _ => (v, false, false)
        }
    }

    fn write_ppu_data(&mut self, data: u8) {
        self.ppu_registers[0x07] = data;
    }
}