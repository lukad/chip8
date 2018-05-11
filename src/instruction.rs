use cpu::Opcode;
use std::fmt;

pub use self::Instruction::*;

pub enum Instruction {
    Call(u16),
    Return,
    LoadConstant(u8, u8),
    AddConstant(u8, u8),
    SetAddress(u16),
    Draw(u8, u8, u8),
    LoadDelay(u8),
    SetDelay(u8),
    SetFontLocation(u8),
    SetBCD(u8),
    NotImplemented(u16),
}

impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Call(addr) => write!(f, "CALL {:#06X}", addr),
            Return => write!(f, "RET"),
            SetAddress(addr) => write!(f, "LD I, {:#06X}", addr),
            LoadConstant(x, v) => write!(f, "LD V[{:#04X}], {:#04X}", x, v),
            AddConstant(x, v) => write!(f, "ADD V[{:#04X}], {:#04x}", x, v),
            Draw(x, y, n) => write!(f, "DRW V[{:#04X}], V[{:#04X}], {:#04X}", x, y, n),
            LoadDelay(x) => write!(f, "LD V[{:#04X}], DT", x),
            SetDelay(x) => write!(f, "LD DT, V[{:#04X}]", x),
            SetFontLocation(x) => write!(f, "LD F, V[{:#04X}]", x),
            SetBCD(x) => write!(f, "LD B, V[{:#04X}]", x),
            NotImplemented(opcode) => write!(f, "{:#06X}", opcode),
        }
    }
}

impl Instruction {
    pub fn decode(Opcode(high, low): Opcode) -> Instruction {
        match (high & 0xF0, low) {
            (0x00, 0xEE) => Return,
            (0x20, _) => Call(((high & 0x0F) as u16) << 8 | low as u16),
            (0x60, _) => LoadConstant(high & 0x0F, low),
            (0x70, _) => AddConstant(high & 0x0F, low),
            (0xA0, _) => SetAddress(((high & 0x0F) as u16) << 8 | low as u16),
            (0xD0, _) => Draw(high & 0x0F, low >> 4, low & 0x0F),
            (0xF0, 0x07) => LoadDelay(high & 0xF),
            (0xF0, 0x15) => SetDelay(high & 0xF),
            (0xF0, 0x29) => SetFontLocation(high & 0xF),
            (0xF0, 0x33) => SetBCD(high & 0xF),
            _ => NotImplemented((high as u16) << 8 | low as u16),
        }
    }
}
