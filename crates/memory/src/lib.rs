pub mod system;
pub mod video;

pub const CPU_RAM_SIZE: usize = 0x0800;
pub const PPU_REGISTER_SIZE: usize = 0x0008;

pub const CPU_RAM_BASE_ADDRESS: u16 = 0x0000;
pub const PPU_REGISTER_BASE_ADDRESS: u16 = 0x2000;
pub const APU_IO_REGISTER_BASE_ADDRESS: u16 = 0x4000;

#[derive(Clone)]
pub struct Memory {
    pub ram: [u8; CPU_RAM_SIZE],
    pub ppu_registers: [u8; PPU_REGISTER_SIZE],

    pub request_to_read_oam_data: bool,
    pub request_to_write_oam_data: bool,
}

impl Default for Memory {
    fn default() -> Self {
        Self {
            ram: [0; CPU_RAM_SIZE],
            ppu_registers: [0; PPU_REGISTER_SIZE],

            request_to_read_oam_data: false,
            request_to_write_oam_data: false,
        }
    }
}