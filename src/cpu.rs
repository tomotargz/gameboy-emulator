use crate::instructions::{go, step};
use crate::operand::{Cond, Direct8, Direct16, IO8, IO16, Imm8, Imm16, Indirect, Reg8, Reg16};
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
    pub fn new() -> Self {
        Cpu {
            regs: Registers::default(),
            ctx: Ctx::default(),
        }
    }

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
            self.cb_decode(bus);
            return;
        }
        match self.ctx.opcode {
            0x00 => self.nop(bus),
            0x01 => self.ld16(bus, Reg16::BC, Imm16),
            0x02 => self.ld(bus, Indirect::BC, Reg8::A),
            0x03 => self.inc16(bus, Reg16::BC),
            0x04 => self.inc(bus, Reg8::B),
            0x05 => self.dec(bus, Reg8::B),
            0x06 => self.ld(bus, Reg8::B, Imm8),
            // 0x07
            0x08 => self.ld16(bus, Direct16, Reg16::SP),
            // 0x09
            0x0a => self.ld(bus, Reg8::A, Indirect::BC),
            0x0b => self.dec16(bus, Reg16::BC),
            0x0c => self.inc(bus, Reg8::C),
            0x0d => self.dec(bus, Reg8::C),
            0x0e => self.ld(bus, Reg8::C, Imm8),
            // 0x0f
            // 0x10
            0x11 => self.ld16(bus, Reg16::DE, Imm16),
            0x12 => self.ld(bus, Indirect::DE, Reg8::A),
            0x13 => self.inc16(bus, Reg16::DE),
            0x14 => self.inc(bus, Reg8::D),
            0x15 => self.dec(bus, Reg8::D),
            0x16 => self.ld(bus, Reg8::D, Imm8),
            // 0x17
            0x18 => self.jr(bus),
            // 0x19
            0x1a => self.ld(bus, Reg8::A, Indirect::DE),
            0x1b => self.dec16(bus, Reg16::DE),
            0x1c => self.inc(bus, Reg8::E),
            0x1d => self.dec(bus, Reg8::E),
            0x1e => self.ld(bus, Reg8::E, Imm8),
            // 0x1f
            0x20 => self.jr_c(bus, Cond::NZ),
            0x21 => self.ld16(bus, Reg16::HL, Imm16),
            0x22 => self.ld(bus, Indirect::HLI, Reg8::A),
            0x23 => self.inc16(bus, Reg16::HL),
            0x24 => self.inc(bus, Reg8::H),
            0x25 => self.dec(bus, Reg8::H),
            0x26 => self.ld(bus, Reg8::H, Imm8),
            // 0x27
            0x28 => self.jr_c(bus, Cond::Z),
            // 0x29
            0x2a => self.ld(bus, Reg8::A, Indirect::HLI),
            0x2b => self.dec16(bus, Reg16::HL),
            0x2c => self.inc(bus, Reg8::L),
            0x2d => self.dec(bus, Reg8::L),
            0x2e => self.ld(bus, Reg8::L, Imm8),
            // 0x2f
            0x30 => self.jr_c(bus, Cond::NC),
            0x31 => self.ld16(bus, Reg16::SP, Imm16),
            0x32 => self.ld(bus, Indirect::HLD, Reg8::A),
            0x33 => self.inc16(bus, Reg16::SP),
            0x34 => self.inc16(bus, Reg16::HL),
            0x35 => self.dec16(bus, Reg16::HL),
            0x36 => self.ld16(bus, Reg16::HL, Imm16),
            // 0x37
            0x38 => self.jr_c(bus, Cond::C),
            // 0x39
            0x3a => self.ld(bus, Reg8::A, Indirect::HLD),
            0x3b => self.dec16(bus, Reg16::SP),
            0x3c => self.inc(bus, Reg8::A),
            0x3d => self.dec(bus, Reg8::A),
            0x3e => self.ld(bus, Reg8::A, Imm8),
            // 0x3f
            0x40 => self.ld(bus, Reg8::B, Reg8::B),
            0x41 => self.ld(bus, Reg8::B, Reg8::C),
            0x42 => self.ld(bus, Reg8::B, Reg8::D),
            0x43 => self.ld(bus, Reg8::B, Reg8::E),
            0x44 => self.ld(bus, Reg8::B, Reg8::H),
            0x45 => self.ld(bus, Reg8::B, Reg8::L),
            0x46 => self.ld(bus, Reg8::B, Indirect::HL),
            0x47 => self.ld(bus, Reg8::B, Reg8::A),
            0x48 => self.ld(bus, Reg8::C, Reg8::B),
            0x49 => self.ld(bus, Reg8::C, Reg8::C),
            0x4a => self.ld(bus, Reg8::C, Reg8::D),
            0x4b => self.ld(bus, Reg8::C, Reg8::E),
            0x4c => self.ld(bus, Reg8::C, Reg8::H),
            0x4d => self.ld(bus, Reg8::C, Reg8::L),
            0x4e => self.ld(bus, Reg8::C, Indirect::HL),
            0x4f => self.ld(bus, Reg8::C, Reg8::A),
            0x50 => self.ld(bus, Reg8::D, Reg8::B),
            0x51 => self.ld(bus, Reg8::D, Reg8::C),
            0x52 => self.ld(bus, Reg8::D, Reg8::D),
            0x53 => self.ld(bus, Reg8::D, Reg8::E),
            0x54 => self.ld(bus, Reg8::D, Reg8::H),
            0x55 => self.ld(bus, Reg8::D, Reg8::L),
            0x56 => self.ld(bus, Reg8::D, Indirect::HL),
            0x57 => self.ld(bus, Reg8::D, Reg8::A),
            0x58 => self.ld(bus, Reg8::E, Reg8::B),
            0x59 => self.ld(bus, Reg8::E, Reg8::C),
            0x5a => self.ld(bus, Reg8::E, Reg8::D),
            0x5b => self.ld(bus, Reg8::E, Reg8::E),
            0x5c => self.ld(bus, Reg8::E, Reg8::H),
            0x5d => self.ld(bus, Reg8::E, Reg8::L),
            0x5e => self.ld(bus, Reg8::E, Indirect::HL),
            0x5f => self.ld(bus, Reg8::E, Reg8::A),
            0x60 => self.ld(bus, Reg8::H, Reg8::B),
            0x61 => self.ld(bus, Reg8::H, Reg8::C),
            0x62 => self.ld(bus, Reg8::H, Reg8::D),
            0x63 => self.ld(bus, Reg8::H, Reg8::E),
            0x64 => self.ld(bus, Reg8::H, Reg8::H),
            0x65 => self.ld(bus, Reg8::H, Reg8::L),
            0x66 => self.ld(bus, Reg8::H, Indirect::HL),
            0x67 => self.ld(bus, Reg8::H, Reg8::A),
            0x68 => self.ld(bus, Reg8::L, Reg8::B),
            0x69 => self.ld(bus, Reg8::L, Reg8::C),
            0x6a => self.ld(bus, Reg8::L, Reg8::D),
            0x6b => self.ld(bus, Reg8::L, Reg8::E),
            0x6c => self.ld(bus, Reg8::L, Reg8::H),
            0x6d => self.ld(bus, Reg8::L, Reg8::L),
            0x6e => self.ld(bus, Reg8::L, Indirect::HL),
            0x6f => self.ld(bus, Reg8::L, Reg8::A),
            0x70 => self.ld(bus, Indirect::HL, Reg8::B),
            0x71 => self.ld(bus, Indirect::HL, Reg8::C),
            0x72 => self.ld(bus, Indirect::HL, Reg8::D),
            0x73 => self.ld(bus, Indirect::HL, Reg8::E),
            0x74 => self.ld(bus, Indirect::HL, Reg8::H),
            0x75 => self.ld(bus, Indirect::HL, Reg8::L),
            // 0x76
            0x77 => self.ld(bus, Indirect::HL, Reg8::A),
            0x78 => self.ld(bus, Reg8::A, Reg8::B),
            0x79 => self.ld(bus, Reg8::A, Reg8::C),
            0x7a => self.ld(bus, Reg8::A, Reg8::D),
            0x7b => self.ld(bus, Reg8::A, Reg8::E),
            0x7c => self.ld(bus, Reg8::A, Reg8::H),
            0x7d => self.ld(bus, Reg8::A, Reg8::L),
            0x7e => self.ld(bus, Reg8::A, Indirect::HL),
            0x7f => self.ld(bus, Reg8::A, Reg8::A),
            // 0xb0
            // 0xb1
            // 0xb2
            // 0xb3
            // 0xb4
            // 0xb5
            // 0xb6
            // 0xb7
            0xb8 => self.cp(bus, Reg8::B),
            0xb9 => self.cp(bus, Reg8::C),
            0xba => self.cp(bus, Reg8::D),
            0xbb => self.cp(bus, Reg8::E),
            0xbc => self.cp(bus, Reg8::H),
            0xbd => self.cp(bus, Reg8::L),
            0xbe => self.cp(bus, Indirect::HL),
            0xbf => self.cp(bus, Reg8::A),
            // 0xc0
            0xc1 => self.pop(bus, Reg16::BC),
            // 0xc2
            // 0xc3
            // 0xc4
            0xc5 => self.push(bus, Reg16::BC),
            // 0xc6
            // 0xc7
            // 0xc8
            0xc9 => self.ret(bus),
            // 0xca
            0xcb => self.cb_prefixed(bus),
            // 0xcc
            0xcd => self.call(bus),
            // 0xce
            // 0xcf
            // 0xd0
            0xd1 => self.pop(bus, Reg16::DE),
            // 0xd2
            // 0xd3
            // 0xd4
            0xd5 => self.push(bus, Reg16::DE),
            // 0xd6
            // 0xd7
            // 0xd8
            // 0xd9
            // 0xda
            // 0xdb
            // 0xdc
            // 0xdd
            // 0xde
            // 0xdf
            0xe0 => self.ld(bus, Direct8::DFF, Reg8::A),
            0xe1 => self.pop(bus, Reg16::HL),
            0xe2 => self.ld(bus, Indirect::CFF, Reg8::A),
            // 0xe3
            // 0xe4
            0xe5 => self.push(bus, Reg16::HL),
            // 0xe6
            // 0xe7
            // 0xe8
            // 0xe9
            0xea => self.ld(bus, Direct8::D, Reg8::A),
            // 0xeb
            // 0xec
            // 0xed
            // 0xee
            // 0xef
            0xf0 => self.ld(bus, Reg8::A, Direct8::DFF),
            0xf1 => self.pop(bus, Reg16::AF),
            0xf2 => self.ld(bus, Reg8::A, Indirect::CFF),
            // 0xf3
            // 0xf4
            0xf5 => self.push(bus, Reg16::AF),
            // 0xf6
            // 0xf7
            // 0xf8
            // 0xf9
            0xfa => self.ld(bus, Reg8::A, Direct8::D),
            // 0xfb
            // 0xfc
            // 0xfd
            0xfe => self.cp(bus, Imm8),
            // 0xff
            _ => panic!("Not implemented: {:02x}", self.ctx.opcode),
        }
    }

    pub fn cb_decode(&mut self, bus: &mut Peripherals) {
        match self.ctx.opcode {
            0x10 => self.rl(bus, Reg8::B),
            0x11 => self.rl(bus, Reg8::C),
            0x12 => self.rl(bus, Reg8::D),
            0x13 => self.rl(bus, Reg8::E),
            0x14 => self.rl(bus, Reg8::H),
            0x15 => self.rl(bus, Reg8::L),
            0x16 => self.rl(bus, Indirect::HL),
            0x17 => self.rl(bus, Reg8::A),
            // 0x18
            // 0x19
            // 0x1a
            // 0x1b
            // 0x1c
            // 0x1d
            // 0x1e
            // 0x1f
            0x40 => self.bit(bus, 0, Reg8::B),
            0x41 => self.bit(bus, 0, Reg8::C),
            0x42 => self.bit(bus, 0, Reg8::D),
            0x43 => self.bit(bus, 0, Reg8::E),
            0x44 => self.bit(bus, 0, Reg8::H),
            0x45 => self.bit(bus, 0, Reg8::L),
            0x46 => self.bit(bus, 0, Indirect::HL),
            0x47 => self.bit(bus, 0, Reg8::A),
            0x48 => self.bit(bus, 1, Reg8::B),
            0x49 => self.bit(bus, 1, Reg8::C),
            0x4a => self.bit(bus, 1, Reg8::D),
            0x4b => self.bit(bus, 1, Reg8::E),
            0x4c => self.bit(bus, 1, Reg8::H),
            0x4d => self.bit(bus, 1, Reg8::L),
            0x4e => self.bit(bus, 1, Indirect::HL),
            0x4f => self.bit(bus, 1, Reg8::A),
            0x50 => self.bit(bus, 2, Reg8::B),
            0x51 => self.bit(bus, 2, Reg8::C),
            0x52 => self.bit(bus, 2, Reg8::D),
            0x53 => self.bit(bus, 2, Reg8::E),
            0x54 => self.bit(bus, 2, Reg8::H),
            0x55 => self.bit(bus, 2, Reg8::L),
            0x56 => self.bit(bus, 2, Indirect::HL),
            0x57 => self.bit(bus, 2, Reg8::A),
            0x58 => self.bit(bus, 3, Reg8::B),
            0x59 => self.bit(bus, 3, Reg8::C),
            0x5a => self.bit(bus, 3, Reg8::D),
            0x5b => self.bit(bus, 3, Reg8::E),
            0x5c => self.bit(bus, 3, Reg8::H),
            0x5d => self.bit(bus, 3, Reg8::L),
            0x5e => self.bit(bus, 3, Indirect::HL),
            0x5f => self.bit(bus, 3, Reg8::A),
            0x60 => self.bit(bus, 4, Reg8::B),
            0x61 => self.bit(bus, 4, Reg8::C),
            0x62 => self.bit(bus, 4, Reg8::D),
            0x63 => self.bit(bus, 4, Reg8::E),
            0x64 => self.bit(bus, 4, Reg8::H),
            0x65 => self.bit(bus, 4, Reg8::L),
            0x66 => self.bit(bus, 4, Indirect::HL),
            0x67 => self.bit(bus, 4, Reg8::A),
            0x68 => self.bit(bus, 5, Reg8::B),
            0x69 => self.bit(bus, 5, Reg8::C),
            0x6a => self.bit(bus, 5, Reg8::D),
            0x6b => self.bit(bus, 5, Reg8::E),
            0x6c => self.bit(bus, 5, Reg8::H),
            0x6d => self.bit(bus, 5, Reg8::L),
            0x6e => self.bit(bus, 5, Indirect::HL),
            0x6f => self.bit(bus, 5, Reg8::A),
            0x70 => self.bit(bus, 6, Reg8::B),
            0x71 => self.bit(bus, 6, Reg8::C),
            0x72 => self.bit(bus, 6, Reg8::D),
            0x73 => self.bit(bus, 6, Reg8::E),
            0x74 => self.bit(bus, 6, Reg8::H),
            0x75 => self.bit(bus, 6, Reg8::L),
            0x76 => self.bit(bus, 6, Indirect::HL),
            0x77 => self.bit(bus, 6, Reg8::A),
            0x78 => self.bit(bus, 7, Reg8::B),
            0x79 => self.bit(bus, 7, Reg8::C),
            0x7a => self.bit(bus, 7, Reg8::D),
            0x7b => self.bit(bus, 7, Reg8::E),
            0x7c => self.bit(bus, 7, Reg8::H),
            0x7d => self.bit(bus, 7, Reg8::L),
            0x7e => self.bit(bus, 7, Indirect::HL),
            0x7f => self.bit(bus, 7, Reg8::A),
            _ => panic!("Not implemented: cb{:02x}", self.ctx.opcode),
        }
    }

    // あやしい
    fn cb_prefixed(&mut self, bus: &mut Peripherals) {
        if let Some(v) = self.read8(bus, Imm8) {
            self.ctx.opcode = v;
            self.ctx.cb = true;
            self.cb_decode(bus);
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
                        Indirect::CFF => bus.read(0xFF00 | (self.regs.c as u16)),
                        Indirect::HLD => {
                            let addr = self.regs.hl();
                            self.regs.write_hl(addr.wrapping_sub(1));
                            bus.read(addr)
                        },
                        Indirect::HLI => {
                            let addr = self.regs.hl();
                            self.regs.write_hl(addr.wrapping_add(1));
                            bus.read(addr)
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
                        let addr = self.regs.hl();
                        self.regs.write_hl(addr.wrapping_sub(1));
                        bus.write(addr, val);
                    },
                    Indirect::HLI => {
                        let addr = self.regs.hl();
                        self.regs.write_hl(addr.wrapping_add(1));
                        bus.write(addr, val);
                    },
                }
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
