/// CPU cycles per line.
pub const CPU_CYCLES_PER_LINE: usize = (341 / 3);

/// Render screen area (width).
pub const RENDER_SCREEN_AREA_WIDTH: usize = 256;
/// Render screen area (height).
pub const RENDER_SCREEN_AREA_HEIGHT: usize = 240;

/// CPU cycles per draw frame.
pub const CPU_CYCLES_PER_DRAW_FRAME: usize = CPU_CYCLES_PER_LINE * ((RENDER_SCREEN_AREA_HEIGHT + 1) as usize);

pub struct Ppu {
    oam: [u8; 0x0100]
}

#[cfg(test)]
mod tests {
    use super::*;
}
