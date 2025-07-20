use crate::operand::{IO8, IO16, Imm8, Reg8, Reg16};
use crate::peripherals::Peripherals;
use crate::registers::Registers;
use std::sync::atomic::AtomicU8;
use std::sync::atomic::Ordering::Relaxed;

#[derive(Default)]
struct Ctx {
    opcode: u8,
    cb: bool,
}

pub struct Cpu {
    regs: Registers,
    ctx: Ctx,
}

impl Cpu {
    pub fn emulate_cycle(&mut self, bus: &Peripherals) {
        self.fetch(bus);
    }

    pub fn fetch(&mut self, bus: &Peripherals) {
        self.ctx.opcode = bus.read(self.regs.pc);
        self.regs.pc = self.regs.pc.wrapping_add(1);
        self.ctx.cb = false;
    }

    pub fn decode(&mut self, bus: &Peripherals) {
        match self.ctx.opcode {
            0x00 => self.nop(bus),
            _ => panic!("Not implemented: {:02x}", self.ctx.opcode),
        }
    }

    pub fn nop(&mut self, bus: &Peripherals) {
        self.fetch(bus);
    }
}

impl IO8<Reg8> for Cpu {
    fn read8(&mut self, _: &Peripherals, src: Reg8) -> Option<u8> {
        Some(match src {
            Reg8::A => self.regs.a,
            Reg8::B => self.regs.b,
            Reg8::C => self.regs.c,
            Reg8::D => self.regs.d,
            Reg8::E => self.regs.e,
            Reg8::F => self.regs.f,
            Reg8::H => self.regs.h,
            Reg8::L => self.regs.l,
        })
    }

    fn write8(&mut self, _: &mut Peripherals, dst: Reg8, val: u8) -> Option<()> {
        Some(match dst {
            Reg8::A => self.regs.a = val,
            Reg8::B => self.regs.b = val,
            Reg8::C => self.regs.c = val,
            Reg8::D => self.regs.d = val,
            Reg8::E => self.regs.e = val,
            Reg8::F => self.regs.f = val,
            Reg8::H => self.regs.h = val,
            Reg8::L => self.regs.l = val,
        })
    }
}

impl IO16<Reg16> for Cpu {
    fn read16(&mut self, _: &Peripherals, src: Reg16) -> Option<u16> {
        Some(match src {
            Reg16::AF => self.regs.af(),
            Reg16::BC => self.regs.bc(),
            Reg16::DE => self.regs.de(),
            Reg16::HL => self.regs.hl(),
            Reg16::SP => self.regs.sp,
        })
    }

    fn write16(&mut self, _: &mut Peripherals, dst: Reg16, val: u16) -> Option<()> {
        Some(match dst {
            Reg16::AF => self.regs.write_af(val),
            Reg16::BC => self.regs.write_bc(val),
            Reg16::DE => self.regs.write_de(val),
            Reg16::HL => self.regs.write_hl(val),
            Reg16::SP => self.regs.sp = val,
        })
    }
}

impl IO8<Imm8> for Cpu {
    fn read8(&mut self, bus: &Peripherals, _: Imm8) -> Option<u8> {
        static STEP: AtomicU8 = AtomicU8::new(0);
        static VAL8: AtomicU8 = AtomicU8::new(0);
        match STEP.load(Relaxed) {
            0 => {
                VAL8.store(bus.read(self.regs.pc), Relaxed);
                self.regs.pc = self.regs.pc.wrapping_add(1);
                STEP.fetch_add(1, Relaxed);
                None
            }
            1 => {
                STEP.store(0, Relaxed);
                Some(VAL8.load(Relaxed))
            }
            _ => {
                unreachable!()
            }
        }
    }

    fn write8(&mut self, _: &mut Peripherals, _: Imm8, _: u8) -> Option<()> {
        unreachable!()
    }
}
