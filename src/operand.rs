use crate::peripherals::Peripherals;
pub trait IO8<T: Copy> {
    fn read8(&mut self, bus: &Peripherals, src: T) -> Option<u8>;
    fn write8(&mut self, bus: &mut Peripherals, dst: T, val: u8) -> Option<()>;
}

pub trait IO16<T: Copy> {
    fn read16(&mut self, bus: &Peripherals, src: T) -> Option<u16>;
    fn write16(&mut self, bus: &mut Peripherals, dst: T, val: u16) -> Option<()>;
}

// 8 bit registers
#[derive(Clone, Copy, Debug)]
pub enum Reg8 {
    A,
    B,
    C,
    D,
    E,
    F,
    H,
    L,
}

// 16 bit registers
#[derive(Clone, Copy, Debug)]
pub enum Reg16 {
    AF,
    BC,
    DE,
    HL,
    SP,
}

// 8 bit immediate value indicated by the program counter
#[derive(Clone, Copy, Debug)]
pub struct Imm8;

// 16 bit immediate value indicated by the program counter
#[derive(Clone, Copy, Debug)]
pub struct Imm16;

// Indirect addressing
#[derive(Clone, Copy, Debug)]
pub enum Indirect {
    BC,
    DE,
    HL,
    CFF,
    HLD,
    HLI,
}

// Direct addressing
#[derive(Clone, Copy, Debug)]
pub enum Direct8 {
    D,
    DFF,
}

#[derive(Clone, Copy, Debug)]
// 16-bit direct address
pub struct Direct16;

#[derive(Clone, Copy, Debug)]
// Condition codes for conditional instructions
pub enum Cond {
    NZ, // Not Zero
    Z,  // Zero
    NC, // Not Carry
    C,  // Carry
}
