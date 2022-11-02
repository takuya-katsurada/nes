pub mod system;

pub const RAM_SIZE: usize = 0x0800;

#[derive(Clone)]
pub struct Memory {
    pub ram: [u8; RAM_SIZE]
}

impl Default for Memory {
    fn default() -> Self {
        Self {
            ram: [0; RAM_SIZE]
        }
    }
}