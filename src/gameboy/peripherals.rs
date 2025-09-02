mod bootrom;
mod hram;
pub mod mbc;
mod ppu;
mod wram;

pub use self::bootrom::Bootrom;
use self::hram::HRam;
use self::ppu::Ppu;
use self::wram::WRam;
use super::cartridge::Cartridge;
use super::cpu::interrupts::Interrupts;
use super::timer::Timer;
use ::sdl2::Sdl;

pub struct Peripherals {
    bootrom: Bootrom,
    wram: WRam,
    hram: HRam,
    pub ppu: Ppu,
    pub timer: Timer,
    cartridge: Cartridge,
}

impl Peripherals {
    pub fn new(bootrom: Bootrom, cartridge: Cartridge, sdl: &Sdl) -> Self {
        Self {
            bootrom,
            wram: WRam::new(),
            hram: HRam::new(),
            ppu: Ppu::new(sdl),
            timer: Timer::default(),
            cartridge,
            // serial: ' ',
        }
    }

    pub fn read(&self, interrupts: &Interrupts, addr: u16) -> u8 {
        match addr {
            0x0000..=0x00FF => {
                if self.bootrom.is_active() {
                    self.bootrom.read(addr)
                } else {
                    self.cartridge.read(addr)
                }
            }
            0x0100..=0x7FFF => self.cartridge.read(addr),
            0x8000..=0x9FFF => self.ppu.read(addr),
            0xA000..=0xBFFF => self.cartridge.read(addr),
            0xC000..=0xFDFF => self.wram.read(addr),
            0xFE00..=0xFE9F => self.ppu.read(addr),
            0xFF04..=0xFF07 => self.timer.read(addr),
            0xFF0F => interrupts.read(addr),
            0xFF40..=0xFF4B => self.ppu.read(addr),
            0xFF80..=0xFFFE => self.hram.read(addr),
            0xFFFF => interrupts.read(addr),
            _ => 0xFF,
        }
    }

    pub fn write(&mut self, interrupts: &mut Interrupts, addr: u16, val: u8) {
        match addr {
            0x0000..=0x00FF => {
                if !self.bootrom.is_active() {
                    self.cartridge.write(addr, val)
                }
            }
            0x0100..=0x7FFF => self.cartridge.write(addr, val),
            0x8000..=0x9FFF => self.ppu.write(addr, val),
            0xA000..=0xBFFF => self.cartridge.write(addr, val),
            0xC000..=0xFDFF => self.wram.write(addr, val),
            0xFE00..=0xFE9F => self.ppu.write(addr, val),
            0xFF04..=0xFF07 => self.timer.write(addr, val),
            0xFF0F => interrupts.write(addr, val),
            0xFF40..=0xFF4B => self.ppu.write(addr, val),
            0xFF50 => self.bootrom.write(addr, val),
            0xFF80..=0xFFFE => self.hram.write(addr, val),
            0xFFFF => interrupts.write(addr, val),
            _ => (),
        }
    }
}
