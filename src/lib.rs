use cpu::Cpu;
use memory::Memory;
use rom::Rom;

#[derive(Clone)]
pub struct Nes {
    cpu: Cpu,
    mem: Memory,
    rom: Rom,
}

impl Default for Nes {
    fn default() -> Self {
        Nes {
            cpu: Default::default(),
            mem: Default::default(),
            rom: Default::default(),
        }
    }
}