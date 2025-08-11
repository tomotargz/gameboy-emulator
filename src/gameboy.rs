mod cartridge;
mod cpu;
mod peripherals;

pub use peripherals::Bootrom;

use ::std::time;
use cpu::Cpu;
use peripherals::Peripherals;
use sdl2::{Sdl, event::Event};

const CPU_CLOCK_HZ: u128 = 4_194_304;
const M_CYCLE_CLOCK: u128 = 4;
const M_CYCLE_NANOS: u128 = M_CYCLE_CLOCK * 1_000_000_000 / CPU_CLOCK_HZ;

pub struct GameBoy {
    cpu: Cpu,
    peripherals: Peripherals,
    sdl: Sdl,
}

impl GameBoy {
    pub fn new(bootrom: Bootrom) -> Self {
        let cpu = Cpu::new();
        let sdl = sdl2::init().expect("failed to initialize SDL");
        let peripherals = Peripherals::new(bootrom, &sdl);
        Self {
            cpu,
            sdl,
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
                self.peripherals.ppu.emulate_cycle();
                emulated += M_CYCLE_NANOS;
            }
        }
    }
}
