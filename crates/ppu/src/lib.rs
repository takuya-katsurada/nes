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
    oam: [u8; OAM_SIZE]
}

impl Default for Ppu {
    fn default() -> Self {
        Self {
            oam: [0; OAM_SIZE],
        }
    }
}

impl Ppu {
    pub fn step(&mut self) -> Option<cpu::Interrupt> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}