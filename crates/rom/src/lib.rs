
use std::io::Read;
use std::error::Error;

pub struct Rom {
    pub prg_rom_bytes: usize,
    pub chr_rom_bytes: usize,
}

impl Default for Rom {
    fn default() -> Self {
        Rom {
            prg_rom_bytes: 0,
            chr_rom_bytes: 0,
        }
    }
}

impl Rom {
    pub fn isValid(data: &mut Vec<u8>) -> bool {
        &data[0..4] == [0x4e,0x45,0x53,0x1a]
    }

    pub fn new(data: Vec<u8>) -> Self {
        Rom {
            prg_rom_bytes: usize::from(data[4]),
            chr_rom_bytes: usize::from(data[5]),
        }
    }
}