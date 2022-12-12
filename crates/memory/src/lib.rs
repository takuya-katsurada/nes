pub mod system;
pub mod system_ppu_registers;

pub const CPU_RAM_SIZE: usize = 0x0800;
pub const PPU_REGISTER_SIZE: usize = 0x0008;

pub const CPU_RAM_BASE_ADDRESS: u16 = 0x0000;
pub const PPU_REGISTER_BASE_ADDRESS: u16 = 0x2000;
pub const APU_IO_REGISTER_BASE_ADDRESS: u16 = 0x4000;

#[derive(Clone)]
pub struct Memory {
    pub ram: [u8; CPU_RAM_SIZE],
    pub ppu_registers: [u8; PPU_REGISTER_SIZE],

    request_to_read_oam_data: bool,
    request_to_read_ppu_data: bool,
    request_to_write_oam_data: bool,
    request_to_write_ppu_scroll: bool,
    request_to_write_ppu_address: bool,
    request_to_write_ppu_data: bool,

    is_second_write: bool,
    ppu_register_scroll_y: u8,
    ppu_register_address_lower: u8,
}

impl Default for Memory {
    fn default() -> Self {
        Self {
            ram: [0; CPU_RAM_SIZE],
            ppu_registers: [0; PPU_REGISTER_SIZE],

            request_to_read_oam_data: false,
            request_to_read_ppu_data: false,
            request_to_write_oam_data: false,
            request_to_write_ppu_scroll: false,
            request_to_write_ppu_address: false,
            request_to_write_ppu_data: false,

            is_second_write: false,
            ppu_register_scroll_y: 0,
            ppu_register_address_lower: 0,
        }
    }
}