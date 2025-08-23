mod gameboy;

use self::gameboy::{Bootrom, Cartridge, GameBoy};
use ::std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Need 1 argument");
    }
    let cartridge_path = &args[1];

    let bootrom_binary = fs::read("./dmg_bootrom.bin")
        .expect("failed to read bootrom")
        .into_boxed_slice();
    let bootrom = Bootrom::new(bootrom_binary);
    let cartridge_binary = fs::read(cartridge_path)
        .expect("failed to read cartridge")
        .into_boxed_slice();
    let cartridge = Cartridge::new(cartridge_binary);
    let mut gameboy = GameBoy::new(bootrom, cartridge);
    gameboy.run();
}
