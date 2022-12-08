mod errors;

use cpu::Cpu;
use ppu::Ppu;
use memory::Memory;
use rom::Rom;
use errors::EmulationError;

#[derive(Clone)]
pub struct Nes {
    cpu: Cpu,
    ppu: Ppu,
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
        if !Rom::is_valid(data) {
            return Err(EmulationError::InvalidRom);
        }

        let nes = Nes {
            cpu: Cpu::default(),
            ppu: Ppu::default(),
            mem: Memory::default(),
            rom: Rom::new(data),
        };
        return Ok(nes);
    }

    pub fn step(&mut self) {
        let mut total_cycle: usize = 0;
        while total_cycle < ppu::CPU_CYCLES_PER_DRAW_FRAME {
            let cpu_cycle = usize::from(self.cpu.step(&mut self.mem));
            if let Some(interrupt) = self.ppu.step(&mut self.mem) {
                self.cpu.interrupt(&mut self.mem, interrupt);
            }

            total_cycle += cpu_cycle;
        }
    }

    pub fn snapshot(self) -> Snapshot {
        Snapshot {
            prg_rom_bytes: self.rom.prg_rom_bytes,
            chr_rom_bytes: self.rom.chr_rom_bytes,
        }
    }
}