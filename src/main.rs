mod gameboy;

use self::gameboy::{Bootrom, GameBoy};
use ::std::fs;

fn main() {
    let bootrom_binary = fs::read("./dmg_bootrom.bin")
        .expect("failed to read bootrom")
        .into_boxed_slice();
    let bootrom = Bootrom::new(bootrom_binary);
    let mut gameboy = GameBoy::new(bootrom);
    gameboy.run();
}
