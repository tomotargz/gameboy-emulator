use crate::{bootrom::Bootrom, cpu::Cpu, lcd::LCD, peripherals::Peripherals};
use sdl2::{self, Sdl, event::Event};
use std::time;

const CPU_CLOCK_HZ: u128 = 4_194_304;
const M_CYCLE_CLOCK: u128 = 4;
const M_CYCLE_NANOS: u128 = M_CYCLE_CLOCK * 1_000_000_000 / CPU_CLOCK_HZ;

pub struct GameBoy {
    cpu: Cpu,
    peripherals: Peripherals,
    lcd: LCD,
    sdl: Sdl,
}

impl GameBoy {
    pub fn new(bootrom: Bootrom) -> Self {
        let cpu = Cpu::new();
        let sdl = sdl2::init().expect("failed to initialize SDL");
        let lcd = LCD::new(&sdl, 4);
        let peripherals = Peripherals::new(bootrom);
        Self {
            cpu,
            sdl,
            lcd,
            peripherals,
        }
    }

    pub fn run(&mut self) {
        let mut event_pump = self.sdl.event_pump().unwrap();
        let time = time::Instant::now();
        let mut emulated: u128 = 0;
        'running: loop {
            let elapsed = time.elapsed().as_nanos();
            for _ in 0..(elapsed - emulated) / M_CYCLE_NANOS {
                for event in event_pump.poll_iter() {
                    match event {
                        Event::Quit { .. } => break 'running,
                        _ => (),
                    }
                }
                self.cpu.emulate_cycle(&mut self.peripherals);
                if self.peripherals.ppu.emulate_cycle() {
                    self.lcd.draw(self.peripherals.ppu.pixel_buffer());
                }
                emulated += M_CYCLE_NANOS;
            }
        }
    }
}
