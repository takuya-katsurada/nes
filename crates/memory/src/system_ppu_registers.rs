use crate::Memory;

pub trait PpuRegistersController {
    // 0x2003: OAM ADDR (Object Attribute Memory).
    fn read_oam_address(&mut self) -> u8;
    // 0x2004: OAM DATA (Object Attribute Memory).
    fn read_oam_data(&mut self) -> (u8, bool, bool);
    fn write_oam_data(&mut self, data: u8);
}

impl PpuRegistersController for Memory {

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
}