use crate::instructions::{go, step};
use crate::operand::{Direct8, Direct16, IO8, IO16, Imm8, Imm16, Indirect, Reg8, Reg16};
use crate::peripherals::Peripherals;
use crate::registers::Registers;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::atomic::{AtomicU8, AtomicU16};

mod instructions;

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
    pub fn emulate_cycle(&mut self, bus: &mut Peripherals) {
        self.decode(bus);
    }

    pub fn fetch(&mut self, bus: &Peripherals) {
        self.ctx.opcode = bus.read(self.regs.pc);
        self.regs.pc = self.regs.pc.wrapping_add(1);
        self.ctx.cb = false;
    }

    pub fn decode(&mut self, bus: &mut Peripherals) {
        if self.ctx.cb {
            match self.ctx.opcode {
                _ => panic!("Not implemented: cb{:02x}", self.ctx.opcode),
            }
        } else {
            match self.ctx.opcode {
                0x00 => self.nop(bus),
                0x01 => self.ld16(bus, Reg16::BC, Imm16),
                0x02 => self.ld(bus, Indirect::BC, Reg8::A),
                0x03 => self.inc16(bus, Reg16::BC),
                0x04 => self.inc(bus, Reg8::B),
                0x05 => self.dec(bus, Reg8::B),
                0x06 => self.ld(bus, Reg8::B, Imm8),
                0x08 => self.ld16(bus, Direct16, Reg16::SP),
                0x0a => self.ld(bus, Reg8::A, Indirect::BC),
                0x0b => self.dec16(bus, Reg16::BC),
                0x0c => self.inc(bus, Reg8::C),
                0x0d => self.dec(bus, Reg8::C),
                0x0e => self.ld(bus, Reg8::C, Imm8),
                _ => panic!("Not implemented: {:02x}", self.ctx.opcode),
            }
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
        step!(None, {
            0: {
                VAL8.store(bus.read(self.regs.pc), Relaxed);
                self.regs.pc = self.regs.pc.wrapping_add(1);
                go!(1);
                return None;
            },
            1: {
                go!(0);
                return Some(VAL8.load(Relaxed));
            },
        });
    }

    fn write8(&mut self, _: &mut Peripherals, _: Imm8, _: u8) -> Option<()> {
        unreachable!()
    }
}

impl IO16<Imm16> for Cpu {
    fn read16(&mut self, bus: &Peripherals, _: Imm16) -> Option<u16> {
        step!(None, {
            0: if let Some(lo) = self.read8(bus, Imm8) {
                VAL8.store(lo, Relaxed);
                go!(1);
            },
            1: if let Some(hi) = self.read8(bus, Imm8) {
                VAL16.store(u16::from_le_bytes([VAL8.load(Relaxed), hi]), Relaxed);
                go!(2);
            },
            2: {
                go!(0);
                return Some(VAL16.load(Relaxed));
            },
        });
    }

    fn write16(&mut self, _: &mut Peripherals, _: Imm16, _: u16) -> Option<()> {
        unreachable!()
    }
}

impl IO8<Indirect> for Cpu {
    fn read8(&mut self, bus: &Peripherals, src: Indirect) -> Option<u8> {
        step!(None, {
            0: {
                VAL8.store(
                    match src {
                        Indirect::BC => bus.read(self.regs.bc()),
                        Indirect::DE => bus.read(self.regs.de()),
                        Indirect::HL => bus.read(self.regs.hl()),
                        Indirect::CFF => bus.read(0xFF00 | (self.regs.c) as u16),
                        Indirect::HLD => {
                            let hl = self.regs.hl();
                            self.regs.write_hl(hl.wrapping_sub(1));
                            bus.read(hl)
                        },
                        Indirect::HLI => {
                            let hl = self.regs.hl();
                            self.regs.write_hl(hl.wrapping_add(1));
                            bus.read(hl)
                        },
                    }, Relaxed);
                go!(1);
                return None;
            },
            1: {
                go!(0);
                return Some(VAL8.load(Relaxed));
            },
        });
    }

    fn write8(&mut self, bus: &mut Peripherals, dst: Indirect, val: u8) -> Option<()> {
        step!(None, {
            0: {
                match dst {
                    Indirect::BC => bus.write(self.regs.bc(), val),
                    Indirect::DE => bus.write(self.regs.de(), val),
                    Indirect::HL => bus.write(self.regs.hl(), val),
                    Indirect::CFF => bus.write(0xFF00 | (self.regs.c as u16), val),
                    Indirect::HLD => {
                        let hl = self.regs.hl();
                        self.regs.write_hl(hl.wrapping_sub(1));
                        bus.write(hl, val);
                    },
                    Indirect::HLI => {
                        let hl = self.regs.hl();
                        self.regs.write_hl(hl.wrapping_add(1));
                        bus.write(hl, val);
                    },
                };
                go!(1);
                return None;
            },
            1: return Some(go!(0)),
        });
    }
}

impl IO8<Direct8> for Cpu {
    fn read8(&mut self, bus: &Peripherals, src: Direct8) -> Option<u8> {
        step!(None, {
            0: if let Some(lo) = self.read8(bus, Imm8) {
                VAL8.store(lo, Relaxed);
                go!(1);
                if let Direct8::DFF = src {
                    VAL16.store(0xFF00 | (lo as u16), Relaxed);
                    go!(2);
                }
            },
            1: if let Some(hi) = self.read8(bus, Imm8){
                VAL16.store(u16::from_le_bytes([VAL8.load(Relaxed), hi]), Relaxed);
                go!(2);
            },
            2: {
                VAL8.store(bus.read(VAL16.load(Relaxed)), Relaxed);
                go!(3);
                return None;
            },
            3: {
                go!(0);
                return Some(VAL8.load(Relaxed));
            },
        });
    }

    fn write8(&mut self, bus: &mut Peripherals, dst: Direct8, val: u8) -> Option<()> {
        step!(None, {
            0: if let Some(lo) = self.read8(bus, Imm8) {
                VAL8.store(lo, Relaxed);
                go!(1);
                if let Direct8::DFF = dst {
                    VAL16.store(0xFF00 | (lo as u16), Relaxed);
                    go!(2);
                }
            },
            1: if let Some(hi) = self.read8(bus, Imm8) {
                VAL16.store(u16::from_le_bytes([VAL8.load(Relaxed), hi]), Relaxed);
                go!(2);
            },
            2: {
                bus.write(VAL16.load(Relaxed), val);
                go!(3);
                return None;
            },
            3: return Some(go!(0)),
        });
    }
}

impl IO16<Direct16> for Cpu {
    fn read16(&mut self, _: &Peripherals, _: Direct16) -> Option<u16> {
        unreachable!()
    }

    fn write16(&mut self, bus: &mut Peripherals, _: Direct16, val: u16) -> Option<()> {
        step!(None, {
            0: if let Some(lo) = self.read8(bus, Imm8) {
                VAL8.store(lo, Relaxed);
                go!(1);
                // なぜstep0とstep1にreturnがない？
            },
            1: if let Some(hi) = self.read8(bus, Imm8) {
                VAL16.store(u16::from_le_bytes([VAL8.load(Relaxed), hi]), Relaxed);
                go!(2);
            },
            2: {
                bus.write(VAL16.load(Relaxed), val as u8);
                go!(3);
                return None;
            },
            3: {
                bus.write(VAL16.load(Relaxed).wrapping_add(1), (val >> 8) as u8);
                go!(4);
                return None;
            },
            4: return Some(go!(0)),
        });
    }
}
