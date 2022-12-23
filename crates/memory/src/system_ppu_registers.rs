use crate::Memory;

const PPU_CTRL:   usize = 0x00;
const PPU_MASK:   usize = 0x01;
const PPU_STATUS: usize = 0x02;
const OAM_ADDR:   usize = 0x03;
const OAM_DATA:   usize = 0x04;
const PPU_SCROLL: usize = 0x05;
const PPU_ADDR:   usize = 0x06;
const PPU_DATA:   usize = 0x07;

pub trait PpuRegistersController {
    // 0x2000: PPU CTRL.
    fn name_table_base_address(&self) -> u16;
    fn address_increment(&self) -> u8;
    fn sprite_pattern_table_address(&self) -> u16;
    fn bg_pattern_table_address(&self) -> u16;
    fn sprite_height(&self) -> u8;
    fn is_master(&self) -> bool;
    fn is_nmi_enable(&self) -> bool;

    // 0x2001: PPU MASK
    fn is_monochrome(&self) -> bool;
    fn is_clip_bg(&self) -> bool;
    fn is_clip_sprite(&self) -> bool;
    fn is_write_bg(&self) -> bool;
    fn is_write_sprite(&self) -> bool;

    // 0x2002: PPU STATUS
    fn is_vblank(&self) -> bool;
    fn on_vblank(&mut self, on: bool);
    fn is_hit_sprite0(&self) -> bool;
    fn on_hit_sprite0(&mut self, on: bool);
    fn is_sprite_overflow(&self) -> bool;
    fn on_sprite_overflow(&mut self, on: bool);
    fn clear_ppu_status(&mut self);

    // 0x2003: OAM ADDR
    fn read_oam_address(&mut self) -> u8;

    // 0x2004: OAM DATA
    fn read_oam_data(&mut self) -> (u8, bool, bool);
    fn write_oam_data(&mut self, data: u8);

    // 0x2006: PPU ADDR.
    fn read_ppu_address(&mut self) -> (u16, bool);

    // $2007: PPU DATA
    fn read_ppu_data(&mut self) -> (u8, bool, bool);
    fn write_ppu_data(&mut self, data: u8);
}

impl PpuRegistersController for Memory {

    #[inline(always)]
    fn name_table_base_address(&self) -> u16 {
        0x2000u16 + (0x0400u16 * (self.ppu_registers[PPU_CTRL] & 0x03u8) as u16)
    }

    #[inline(always)]
    fn address_increment(&self) -> u8 {
        if (self.ppu_registers[PPU_CTRL] & 0x04u8) == 0x04u8 { 32u8 } else { 1u8 }
    }

    #[inline(always)]
    fn sprite_pattern_table_address(&self) -> u16 {
        if (self.ppu_registers[PPU_CTRL] & 0x08u8) == 0x08u8 { 0x1000u16 } else { 0x0000u16 }
    }

    #[inline(always)]
    fn bg_pattern_table_address(&self) -> u16 {
        if (self.ppu_registers[PPU_CTRL] & 0x10u8) == 0x10u8 { 0x1000u16 } else { 0x0000u16 }
    }

    #[inline(always)]
    fn sprite_height(&self) -> u8 {
        if (self.ppu_registers[PPU_CTRL] & 0x20u8) == 0x20u8 { 16 } else { 8 }
    }

    #[inline(always)]
    fn is_master(&self) -> bool {
        (self.ppu_registers[PPU_CTRL] & 0x40u8) == 0x40u8
    }

    #[inline(always)]
    fn is_nmi_enable(&self) -> bool {
        (self.ppu_registers[PPU_CTRL] & 0x80u8) == 0x80u8
    }

    #[inline(always)]
    fn is_monochrome(&self) -> bool {
        (self.ppu_registers[PPU_MASK] & 0x01u8) == 0x01u8
    }

    #[inline(always)]
    fn is_clip_bg(&self) -> bool {
        (self.ppu_registers[PPU_MASK] & 0x02u8) == 0x02u8
    }

    #[inline(always)]
    fn is_clip_sprite(&self) -> bool {
        (self.ppu_registers[PPU_MASK] & 0x04u8) == 0x04u8
    }

    #[inline(always)]
    fn is_write_bg(&self) -> bool {
        (self.ppu_registers[PPU_MASK] & 0x08u8) == 0x08u8
    }

    #[inline(always)]
    fn is_write_sprite(&self) -> bool {
        (self.ppu_registers[PPU_MASK] & 0x10u8) == 0x10u8
    }

    #[inline(always)]
    fn is_vblank(&self) -> bool {
        (self.ppu_registers[PPU_STATUS] & 0x80u8) == 0x80u8
    }

    #[inline(always)]
    fn on_vblank(&mut self, on: bool) {
        let v = self.ppu_registers[PPU_STATUS];
        self.ppu_registers[PPU_STATUS] = if on { v | 0x80u8 } else { v & (!0x80u8) }
    }

    #[inline(always)]
    fn is_hit_sprite0(&self) -> bool {
        (self.ppu_registers[PPU_STATUS] & 0x40u8) == 0x40u8
    }

    #[inline(always)]
    fn on_hit_sprite0(&mut self, on: bool) {
        let v = self.ppu_registers[PPU_STATUS];
        self.ppu_registers[PPU_STATUS] = if on { v | 0x40u8 } else { v & (!0x40u8) }
    }

    #[inline(always)]
    fn is_sprite_overflow(&self) -> bool {
        (self.ppu_registers[PPU_STATUS] & 0x20u8) == 0x20u8
    }

    #[inline(always)]
    fn on_sprite_overflow(&mut self, on: bool) {
        let v = self.ppu_registers[PPU_STATUS];
        self.ppu_registers[PPU_STATUS] = if on { v | 0x20u8 } else { v & (!0x20u8) }
    }

    #[inline(always)]
    fn clear_ppu_status(&mut self) {
        self.ppu_registers[PPU_STATUS] = 0x00
    }

    #[inline(always)]
    fn read_oam_address(&mut self) -> u8 {
        self.ppu_registers[OAM_ADDR]
    }

    fn read_oam_data(&mut self) -> (u8, bool, bool) {
        let r = self.request_to_read_oam_data;
        let w = self.request_to_write_oam_data;
        let v = self.ppu_registers[OAM_DATA];

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

    #[inline(always)]
    fn write_oam_data(&mut self, data: u8) {
        self.ppu_registers[OAM_DATA] = data;
    }

    fn read_ppu_address(&mut self) -> (u16, bool) {
        let hi = u16::from(self.ppu_registers[PPU_ADDR]) << 8;
        let lo = u16::from(self.ppu_register_address_lower);
        let is_request = self.request_to_write_ppu_address;

        self.request_to_write_ppu_address = false;
        (lo|hi, is_request)
    }

    fn read_ppu_data(&mut self) -> (u8, bool, bool) {
        let r = self.request_to_read_ppu_data;
        let w = self.request_to_write_ppu_data;
        let v = self.ppu_registers[PPU_DATA];

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

    #[inline(always)]
    fn write_ppu_data(&mut self, data: u8) {
        self.ppu_registers[PPU_DATA] = data;
    }
}