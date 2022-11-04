use crate::Cpu;

impl Cpu {
    fn write_status_flag(&mut self, status: u8, is_active: bool) {
        if is_active {
            self.p = self. p | status;
        } else {
            self.p = self.p & (!status);
        }
    }
}