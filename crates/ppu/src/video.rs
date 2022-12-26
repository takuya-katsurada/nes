#[derive(Clone)]
pub struct Video {}

impl Default for Video {
    fn default() -> Self {
        Self{}
    }
}

impl Video {
    pub fn read(&self, address: u16) -> u8 {
        return 0
    }

    pub fn write(&mut self, address: u16, data: u8) {
    }
}