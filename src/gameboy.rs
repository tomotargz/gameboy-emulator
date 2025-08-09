use crate::{bootrom::Bootrom, cpu::Cpu, lcd::LCD, peripherals::Peripherals};
use sdl2;
use std::time;

const CPU_CLOCK_HZ: u128 = 4_194_304;
const M_CYCLE_CLOCK: u128 = 4;
const M_CYCLE_NANOS: u128 = M_CYCLE_CLOCK * 1_000_000_000 / CPU_CLOCK_HZ;

pub struct GameBoy {
    cpu: Cpu,
    peripherals: Peripherals,
    lcd: LCD,
}

impl GameBoy {
    pub fn new(bootrom: Bootrom) -> Self {
        let sdl = sdl2::init().expect("failed to initialize SDL");
        let lcd = LCD::new(&sdl, 4);
        let peripherals = Peripherals::new(bootrom);
        let cpu = Cpu::new();
        Self {
            cpu,
            peripherals,
            lcd,
        }
    }

    pub fn run(&mut self) {
        let time = time::Instant::now();
        let mut emulated: u128 = 0;
        loop {
            let elapsed= time.elapsed().as_nanos();
            for _ in 0..(elapsed - emulated) / M_CYCLE_NANOS {
                self.cpu.emulate_cycle(&mut self.peripherals);
                if self.peripherals.ppu.emulate_cycle() {
                    self.lcd.draw(self.peripherals.ppu.pixel_buffer());
                }
                emulated += M_CYCLE_NANOS;
            }
        }
    }
}
