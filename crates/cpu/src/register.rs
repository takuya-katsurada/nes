use crate::Cpu;

pub const CARRY_FLAG: u8 = 0x01u8;

impl Cpu {
    #[inline(always)]
    pub(crate) fn write_carry_flag(&mut self, is_active: bool) {
        self.write_status_flag(CARRY_FLAG, is_active);
    }
    #[inline(always)]
    pub(crate) fn read_carry_flag(&mut self) -> bool {
        self.read_status_flag(CARRY_FLAG)
    }

    fn read_status_flag(&mut self, status: u8) -> bool {
        (self.p & status) == status
    }
    fn write_status_flag(&mut self, status: u8, is_active: bool) {
        if is_active {
            self.p = self. p | status;
        } else {
            self.p = self.p & (!status);
        }
    }
}