#[derive(Default)]
pub struct Interrupts {
    pub ime: bool,
    pub int_flags: u8,
    pub int_enable: u8,
}

pub const VBLANK: u8 = 1 << 0;
pub const STAT: u8 = 1 << 1;
pub const TIMER: u8 = 1 << 2;
pub const SERIAL: u8 = 1 << 3;
pub const JOYPAD: u8 = 1 << 4;

impl Interrupts {
    pub fn irq(&mut self, val: u8) {
        self.int_flags |= val;
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0xFF0F => self.int_flags,
            0xFFFF => self.int_enable,
            _ => unreachable!(),
        }
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        match addr {
            0xFF0F => self.int_flags = val,
            0xFFFF => self.int_enable = val,
            _ => unreachable!(),
        }
    }

    pub fn get_interrupts(&self) -> u8 {
        self.int_flags & self.int_enable & 0b11111
    }
}
