mod gameboy;

use self::gameboy::{Bootrom, Cartridge, GameBoy};
use ::std::fs;

fn main() {
    let bootrom_binary = fs::read("./dmg_bootrom.bin")
        .expect("failed to read bootrom")
        .into_boxed_slice();
    let bootrom = Bootrom::new(bootrom_binary);
    let cartridge_binary = fs::read("./cartridge.bin") // temp
        .expect("failed to read cartridge")
        .into_boxed_slice();
    let cartridge = Cartridge::new(cartridge_binary);
    let mut gameboy = GameBoy::new(bootrom, cartridge);
    gameboy.run();
}
