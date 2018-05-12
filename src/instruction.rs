use cpu::Opcode;
use std::fmt;

pub use self::Instruction::*;

pub enum Instruction {
    Return,
    Jump(u16),
    Call(u16),
    SkipIfEqual(u8, u8),
    SkipIfNotEqual(u8, u8),
    LoadConstant(u8, u8),
    AddConstant(u8, u8),
    Load(u8, u8),
    And(u8, u8),
    Add(u8, u8),
    Sub(u8, u8),
    SetAddress(u16),
    RandomAnd(u8, u8),
    Draw(u8, u8, u8),
    SkipIfNotPressed(u8),
    LoadDelay(u8),
    SetDelay(u8),
    SetFontLocation(u8),
    SetBCD(u8),
    LoadRegisters(u8),
    NotImplemented(u16),
}

impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Return => write!(f, "RET"),
            Jump(addr) => write!(f, "JP {:#06X}", addr),
            Call(addr) => write!(f, "CALL {:#06X}", addr),
            SkipIfEqual(x, kk) => write!(f, "SE V[{:#04X}], {:#04X}", x, kk),
            SkipIfNotEqual(x, kk) => write!(f, "SNE V[{:#04X}], {:#04X}", x, kk),
            LoadConstant(x, kk) => write!(f, "LD V[{:#04X}], {:#04X}", x, kk),
            AddConstant(x, kk) => write!(f, "ADD V[{:#04X}], {:#04X}", x, kk),
            Load(x, y) => write!(f, "LD V[{:#04X}], V[{:#04X}]", x, y),
            And(x, y) => write!(f, "AND V[{:#04X}], V[{:#04X}]", x, y),
            Add(x, y) => write!(f, "ADD V[{:#04X}], V[{:#04X}]", x, y),
            Sub(x, y) => write!(f, "SUB V[{:#04X}], V[{:#04X}]", x, y),
            SetAddress(addr) => write!(f, "LD I, {:#06X}", addr),
            RandomAnd(x, kk) => write!(f, "RND V[{:#04X}], {:#04X}", x, kk),
            Draw(x, y, n) => write!(f, "DRW V[{:#04X}], V[{:#04X}], {:#04X}", x, y, n),
            SkipIfNotPressed(x) => write!(f, "SKNP v[{:#04X}]", x),
            LoadDelay(x) => write!(f, "LD V[{:#04X}], DT", x),
            SetDelay(x) => write!(f, "LD DT, V[{:#04X}]", x),
            SetFontLocation(x) => write!(f, "LD F, V[{:#04X}]", x),
            SetBCD(x) => write!(f, "LD B, V[{:#04X}]", x),
            LoadRegisters(x) => write!(f, "LD V[0...{:#04X}], [I]", x),
            NotImplemented(opcode) => write!(f, "{:#06X}", opcode),
        }
    }
}

impl Instruction {
    pub fn decode(Opcode(high, low): Opcode) -> Instruction {
        match (high & 0xF0, low) {
            (0x00, 0xEE) => Return,
            (0x10, _) => Jump(((high & 0x0F) as u16) << 8 | low as u16),
            (0x20, _) => Call(((high & 0x0F) as u16) << 8 | low as u16),
            (0x30, _) => SkipIfEqual(high & 0x0F, low),
            (0x40, _) => SkipIfNotEqual(high & 0x0F, low),
            (0x60, _) => LoadConstant(high & 0x0F, low),
            (0x70, _) => AddConstant(high & 0x0F, low),
            (0x80, _) => match low & 0x0F {
                0x00 => Load(high & 0x0F, low >> 4),
                0x02 => And(high & 0x0F, low >> 4),
                0x04 => Add(high & 0x0F, low >> 4),
                0x05 => Sub(high & 0x0F, low >> 4),
                _ => NotImplemented((high as u16) << 8 | low as u16),
            },
            (0xA0, _) => SetAddress(((high & 0x0F) as u16) << 8 | low as u16),
            (0xC0, _) => RandomAnd(high & 0x0F, low),
            (0xD0, _) => Draw(high & 0x0F, low >> 4, low & 0x0F),
            (0xE0, 0xA1) => SkipIfNotPressed(high & 0xF),
            (0xF0, 0x07) => LoadDelay(high & 0xF),
            (0xF0, 0x15) => SetDelay(high & 0xF),
            (0xF0, 0x29) => SetFontLocation(high & 0xF),
            (0xF0, 0x33) => SetBCD(high & 0xF),
            (0xF0, 0x65) => LoadRegisters(high & 0xF),
            _ => NotImplemented((high as u16) << 8 | low as u16),
        }
    }
}
