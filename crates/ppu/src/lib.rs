mod register;

/// CPU cycles per line.
pub const CPU_CYCLES_PER_LINE: usize = 341 / 3;

/// Render screen area (width).
pub const RENDER_SCREEN_AREA_WIDTH: usize = 256;
/// Render screen area (height).
pub const RENDER_SCREEN_AREA_HEIGHT: usize = 240;

/// CPU cycles per draw frame.
pub const CPU_CYCLES_PER_DRAW_FRAME: usize = CPU_CYCLES_PER_LINE * ((RENDER_SCREEN_AREA_HEIGHT + 1) as usize);

pub const OAM_SIZE: usize = 0x0100;

#[derive(Clone)]
pub struct Ppu {
    oam: [u8; OAM_SIZE],
}

impl Default for Ppu {
    fn default() -> Self {
        Self {
            oam: [0; OAM_SIZE],
        }
    }
}

impl Ppu {
    pub fn reset(&mut self) {
        self.oam = [0; OAM_SIZE];
    }

    pub fn step(
        &mut self,
        registers: &mut dyn memory::system_ppu_registers::PpuRegistersController
    ) -> Option<cpu::Interrupt> {

        {
            let address = registers.read_oam_address();
            let (data, reading_requested, writing_requested) = registers.read_oam_data();
            if writing_requested {
                self.oam[usize::from(address)] = data;
            }
            if reading_requested {
                let data = self.oam[usize::from(address)];
                registers.write_oam_data(data);
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use memory::system::SystemBus;
    use memory::system_ppu_registers::PpuRegistersController;
    use crate::OAM_SIZE;

    # [test]
    fn reset()
    {
        let mut ppu = super::Ppu::default();
        ppu.reset();

        assert_eq!(ppu.oam, [0; OAM_SIZE]);
    }

    # [test]
    fn execute_step_to_read_and_write_oam()
    {
        {
            let mut ppu = super::Ppu::default();
            let mut mem = memory::Memory::default();

            mem.ppu_registers[0x03] = 0xff;
            mem.write_u8(0x2004, 0x80);
            ppu.step(&mut mem);

            assert_eq!(ppu.oam[0xff], 0x80);
        }

        {
            let mut ppu = super::Ppu::default();
            let mut mem = memory::Memory::default();

            ppu.oam[0xff] = 0x0f;
            mem.ppu_registers[0x03] = 0xff;
            mem.ppu_registers[0x04] = 0x80;
            mem.read_u8(0x2004);
            ppu.step(&mut mem);

            assert_eq!(mem.ppu_registers[0x04], 0x0f);
        }
    }
}