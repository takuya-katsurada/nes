use crate::Cpu;

pub const CARRY_FLAG:     u8 = 0x01u8;
pub const ZERO_FLAG:      u8 = 0x02u8;
pub const INTERRUPT_FLAG: u8 = 0x04u8;
pub const DECIMAL_FLAG:   u8 = 0x08u8;
pub const BREAK_FLAG:     u8 = 0x10u8;
pub const RESERVED_FLAG:  u8 = 0x20u8;
pub const OVERFLOW_FLAG:  u8 = 0x40u8;
pub const NEGATIVE_FLAG:  u8 = 0x80u8;

impl Cpu {
    #[inline(always)]
    pub(crate) fn write_carry_flag(&mut self, is_active: bool) {
        self.write_status_flag(CARRY_FLAG, is_active);
    }
    #[inline(always)]
    pub(crate) fn read_carry_flag(&mut self) -> bool {
        self.read_status_flag(CARRY_FLAG)
    }

    #[inline(always)]
    pub(crate) fn write_zero_flag(&mut self, is_active: bool) {
        self.write_status_flag(ZERO_FLAG, is_active);
    }
    #[inline(always)]
    pub(crate) fn read_zero_flag(&mut self) -> bool {
        self.read_status_flag(ZERO_FLAG)
    }

    #[inline(always)]
    pub(crate) fn write_interrupt_flag(&mut self, is_active: bool) {
        self.write_status_flag(INTERRUPT_FLAG, is_active);
    }
    #[inline(always)]
    pub(crate) fn read_interrupt_flag(&mut self) -> bool {
        self.read_status_flag(INTERRUPT_FLAG)
    }

    #[inline(always)]
    pub(crate) fn write_decimal_flag(&mut self, is_active: bool) {
        self.write_status_flag(DECIMAL_FLAG, is_active);
    }
    #[inline(always)]
    pub(crate) fn read_decimal_flag(&mut self) -> bool {
        self.read_status_flag(DECIMAL_FLAG)
    }

    #[inline(always)]
    pub(crate) fn write_break_flag(&mut self, is_active: bool) {
        self.write_status_flag(BREAK_FLAG, is_active);
    }
    #[inline(always)]
    pub(crate) fn read_break_flag(&mut self) -> bool {
        self.read_status_flag(BREAK_FLAG)
    }

    #[inline(always)]
    pub(crate) fn write_reserved_flag(&mut self, is_active: bool) {
        self.write_status_flag(RESERVED_FLAG, is_active);
    }
    #[inline(always)]
    pub(crate) fn read_reserved_flag(&mut self) -> bool {
        self.read_status_flag(RESERVED_FLAG)
    }

    #[inline(always)]
    pub(crate) fn write_overflow_flag(&mut self, is_active: bool) {
        self.write_status_flag(OVERFLOW_FLAG, is_active);
    }
    #[inline(always)]
    pub(crate) fn read_overflow_flag(&mut self) -> bool {
        self.read_status_flag(OVERFLOW_FLAG)
    }

    #[inline(always)]
    pub(crate) fn write_negative_flag(&mut self, is_active: bool) {
        self.write_status_flag( NEGATIVE_FLAG, is_active);
    }
    #[inline(always)]
    pub(crate) fn read_negative_flag(&mut self) -> bool {
        self.read_status_flag(NEGATIVE_FLAG)
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