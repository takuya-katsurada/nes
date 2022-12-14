use crate::video::Video;

mod video;

/// CPU cycles per line.
pub const CPU_CYCLES_PER_LINE: usize = 341 / 3;

/// Render screen area (width).
pub const RENDER_SCREEN_AREA_WIDTH: usize = 256;
/// Render screen area (height).
pub const RENDER_SCREEN_AREA_HEIGHT: usize = 240;

/// CPU cycles per draw frame.
pub const CPU_CYCLES_PER_DRAW_FRAME: usize = CPU_CYCLES_PER_LINE * ((RENDER_SCREEN_AREA_HEIGHT + 1) as usize);

pub const OAM_SIZE: usize = 0x0100;

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
enum ScanLineMode {
    Visible,
    PostRender,
    VerticalBlanking,
    PreRender
}

impl ScanLineMode {
    fn from(line: u16) -> ScanLineMode {
        match line {
            0..=239   => ScanLineMode::Visible,
            240       => ScanLineMode::PostRender,
            241..=260 => ScanLineMode::VerticalBlanking,
            261       => ScanLineMode::PreRender,
            _         => panic!("invalid line")
        }
    }
}

#[derive(Clone)]
pub struct Ppu {
    oam: [u8; OAM_SIZE],
    video: Video,

    fetch_scroll_x: u8,
    fetch_scroll_y: u8,
    cumulative_cpu_cycles: usize,
}

impl Default for Ppu {
    fn default() -> Self {
        Self {
            oam: [0; OAM_SIZE],
            video: Default::default(),
            fetch_scroll_x: 0,
            fetch_scroll_y: 0,
            cumulative_cpu_cycles: 0,
        }
    }
}

impl Ppu {
    pub fn reset(&mut self) {
        self.oam = [0; OAM_SIZE];
        self.fetch_scroll_x = 0;
        self.fetch_scroll_y = 0;
    }

    pub fn step(
        &mut self,
        cpu_cycles: usize,
        registers: &mut dyn memory::system_ppu_registers::PpuRegistersController
    ) -> Option<cpu::Interrupt> {
        let (scroll_x, scroll_y, _) = registers.read_ppu_scroll();
        self.fetch_scroll_x = scroll_x;
        self.fetch_scroll_y = scroll_y;

        let (ppu_address, _) = registers.read_ppu_address();
        let (ppu_data, is_read_ppu_data, is_write_ppu_data) = registers.read_ppu_data();
        if is_write_ppu_data {
            self.video.write(ppu_address, ppu_data);
            registers.increment_ppu_address();
        }
        if is_read_ppu_data {
            registers.write_ppu_data(self.video.read(ppu_address));
            registers.increment_ppu_address();
        }

        let address = registers.read_oam_address();
        let (data, reading_requested, writing_requested) = registers.read_oam_data();
        if writing_requested {
            self.oam[usize::from(address)] = data;
        }
        if reading_requested {
            let data = self.oam[usize::from(address)];
            registers.write_oam_data(data);
        }

        let current_cup_cycles = self.cumulative_cpu_cycles + cpu_cycles;
        if current_cup_cycles >= CPU_CYCLES_PER_LINE {
            self.cumulative_cpu_cycles = current_cup_cycles - CPU_CYCLES_PER_LINE;
        } else {
            self.cumulative_cpu_cycles = current_cup_cycles;
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use memory::system::SystemBus;
    use memory::system_ppu_registers::PpuRegistersController;
    use crate::{OAM_SIZE, ScanLineMode};

    # [test]
    fn reset()
    {
        let mut ppu = super::Ppu::default();
        ppu.reset();

        assert_eq!(ppu.oam, [0; OAM_SIZE]);
        assert_eq!(ppu.fetch_scroll_x, 0);
        assert_eq!(ppu.fetch_scroll_y, 0);
        assert_eq!(ppu.cumulative_cpu_cycles, 0);
    }

    # [test]
    fn scan_line_mode_form_u16()
    {
        assert_eq!(ScanLineMode::from(0), ScanLineMode::Visible);
        assert_eq!(ScanLineMode::from(239), ScanLineMode::Visible);
        assert_eq!(ScanLineMode::from(240), ScanLineMode::PostRender);
        assert_eq!(ScanLineMode::from(241), ScanLineMode::VerticalBlanking);
        assert_eq!(ScanLineMode::from(260), ScanLineMode::VerticalBlanking);
        assert_eq!(ScanLineMode::from(261), ScanLineMode::PreRender);
    }

    # [test]
    fn execute_step_to_read_scroll()
    {
        let mut ppu = super::Ppu::default();
        let mut mem = memory::Memory::default();

        mem.write_u8(0x2005, 0x12);
        mem.write_u8(0x2005, 0x34);
        ppu.step(0, &mut mem);

        assert_eq!(ppu.fetch_scroll_x, 0x12u8);
        assert_eq!(ppu.fetch_scroll_y, 0x34u8);
    }

    # [test]
    fn execute_step_to_read_and_write_oam()
    {
        {
            let mut ppu = super::Ppu::default();
            let mut mem = memory::Memory::default();

            mem.ppu_registers[0x03] = 0xff;
            mem.write_u8(0x2004, 0x80);
            ppu.step(0, &mut mem);

            assert_eq!(ppu.oam[0xff], 0x80);
        }

        {
            let mut ppu = super::Ppu::default();
            let mut mem = memory::Memory::default();

            ppu.oam[0xff] = 0x0f;
            mem.ppu_registers[0x03] = 0xff;
            mem.ppu_registers[0x04] = 0x80;
            mem.read_u8(0x2004);
            ppu.step(0, &mut mem);

            assert_eq!(mem.ppu_registers[0x04], 0x0f);
        }
    }
}