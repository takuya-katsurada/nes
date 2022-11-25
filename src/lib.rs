mod errors;

use cpu::Cpu;
use memory::Memory;
use rom::Rom;
use errors::EmulationError;

#[derive(Clone)]
pub struct Nes {
    cpu: Cpu,
    mem: Memory,
    rom: Rom,
}

#[derive(Clone)]
pub struct Snapshot {
    pub prg_rom_bytes: usize,
    pub chr_rom_bytes: usize,
}

impl Nes {
    pub fn from(data: &mut Vec<u8>) -> Result<Nes, EmulationError> {
        if !Rom::isValid(data) {
            return Err(EmulationError::InvalidRom);
        }

        let nes = Nes {
            cpu: Cpu::default(),
            mem: Memory::default(),
            rom: Rom::new(data),
        };
        return Ok(nes);
    }

    pub fn snapshot(self) -> Snapshot {
        Snapshot {
            prg_rom_bytes: self.rom.prg_rom_bytes,
            chr_rom_bytes: self.rom.chr_rom_bytes,
        }
    }
}