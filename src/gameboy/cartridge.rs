#[repr(C)]
pub struct CartridgeHeader {
    entry_point: [u8; 4],
    logo: [u8; 48],
    title: [u8; 11],
    maker: [u8; 4],
    cgb_flag: [u8; 1],
    new_licensee: [u8; 2],
    sgb_flag: [u8; 1],
    cartridge_type: [u8; 1],
    rom_size: [u8; 1],
    sram_size: [u8; 1],
    destination: [u8; 1],
    old_licensee: [u8; 1],
    game_version: [u8; 1],
    header_checksum: [u8; 1],
    global_checksum: [u8; 2],
}

impl CartridgeHeader {
    fn new(data: [u8; 0x50]) -> Self {
        let ret = unsafe { std::mem::transmute::<[u8; 0x50], Self>(data) };
        let mut checksum: u8 = 0;
        for i in 0x34..=0x4c {
            checksum = checksum.wrapping_sub(data[i]).wrapping_sub(1);
        }
        assert!(checksum == ret.header_checksum[0], "Checksum validation failed.");
        ret
    }

    fn rom_size(&self) -> usize {
        assert!(
            self.rom_size[0] <= 0x80,
            "Invalid rom size {}.",
            self.rom_size[0]
        );
        return 1 << (15 + self.rom_size[0]);
    }

    fn sram_size(&self) -> usize {
        match self.sram_size[0] {
            0x00 => 0,
            0x01 => 0x800,
            0x02 => 0x2000,
            0x03 => 0x8000,
            0x04 => 0x20000,
            0x05 => 0x10000,
            _ => panic!("Invalid sram size {}.", self.sram_size[0]),
        }
    }
}
