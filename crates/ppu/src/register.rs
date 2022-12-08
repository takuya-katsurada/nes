use crate::Ppu;

impl Ppu {
    
    #[inline(always)]
    pub(crate) fn read_oam_address(&mut self, video: &mut dyn memory::video::VideoBus) -> u8 {
        video.read_oam_address()
    }

    #[inline(always)]
    pub(crate) fn read_oam_data(&mut self, video: &mut dyn memory::video::VideoBus) -> (u8, bool, bool) {
        video.read_oam_data()
    }
    #[inline(always)]
    pub(crate) fn write_oam_data(&mut self, video: &mut dyn memory::video::VideoBus, data: u8) {
        video.write_oam_data(data)
    }
}