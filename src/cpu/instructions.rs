use crate::{
    cpu::Cpu,
    instructions::{go, step},
    operand::{IO8, IO16},
    peripherals::Peripherals,
};

use std::sync::atomic::{AtomicU8, AtomicU16, Ordering::Relaxed};

impl Cpu {
    pub fn ld<D: Copy, S: Copy>(&mut self, bus: &mut Peripherals, dst: D, src: S)
    where
        Self: IO8<D> + IO8<S>,
    {
        step!((), {
            0: if let Some(v) = self.read8(bus, src) {
                VAL8.store(v, Relaxed);
                go!(1);
            },
            1: if self.write8(bus, dst, VAL8.load(Relaxed)).is_some() {
                go!(2);
            },
            2: {
                go!(0);
                self.fetch(bus);
            },
        });
    }

    pub fn ld16<D: Copy, S: Copy>(&mut self, bus: &mut Peripherals, dst: D, src: S)
    where
        Self: IO16<D> + IO16<S>,
    {
        step!((), {
            0: if let Some(v) = self.read16(bus, src) {
                VAL16.store(v, Relaxed);
                go!(1);
            },
            1: if self.write16(bus, dst, VAL16.load(Relaxed)).is_some(){
                go!(2);
            },
            2: {
                go!(0);
                self.fetch(bus);
            },
        });
    }
}
