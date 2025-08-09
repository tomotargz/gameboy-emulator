mod bootrom;
mod cpu;
mod gameboy;
mod hram;
mod instructions;
mod lcd;
mod operand;
mod peripherals;
mod ppu;
mod registers;
mod wram;

use std::fs;

use bootrom::Bootrom;
use gameboy::GameBoy;

fn main() {
    let bootrom_binary = fs::read("./dmg_bootrom.bin").expect("failed to read bootrom").into_boxed_slice();
    let bootrom = Bootrom::new(bootrom_binary);
    let mut gameboy = GameBoy::new(bootrom);
    gameboy.run();
}
