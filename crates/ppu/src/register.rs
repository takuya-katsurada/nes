use crate::Ppu;

impl Ppu {

    #[inline(always)]
    pub(crate) fn read_oam_address(&mut self, control: &mut dyn memory::system_ppu_registers::PpuRegistersController) -> u8 {
        control.read_oam_address()
    }

    #[inline(always)]
    pub(crate) fn read_oam_data(&mut self, control: &mut dyn memory::system_ppu_registers::PpuRegistersController) -> (u8, bool, bool) {
        control.read_oam_data()
    }
    #[inline(always)]
    pub(crate) fn write_oam_data(&mut self, control: &mut dyn memory::system_ppu_registers::PpuRegistersController, data: u8) {
        control.write_oam_data(data)
    }
}