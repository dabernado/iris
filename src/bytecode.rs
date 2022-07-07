use crate::alloc::api::{AllocObject, ITypeId};
use crate::safeptr::UntypedPtr;
use crate::array::ArraySize;

pub type Opcode = u32;

mod const {
    // Bitmasks
    pub const OP_MASK: u32 = 0xFFFFFFC0;
    pub const I_MASK: u32 = 0x0000003F;
    pub const C_OFF_MASK: u32 = 0xFFF8003F;
    pub const C_CONST_MASK: u32 = 0x0007FFFF;
    pub const S_TOTAL_MASK: u32 = 0xFFFFC03F;
    pub const S_DIV_MASK: u32 = 0xFFC03FFF;
    pub const S_OFF_MASK: u32 = 0xC03FFFFF;

    // I-Type
    pub const OP_ID: u8 = 0x00;
    pub const OP_ID_R: u8 = !OP_ID - OP_MASK;
    pub const OP_ZEROI: u8 = 0x01;
    pub const OP_ZEROE: u8 = !OP_ZEROI - OP_MASK;
    pub const OP_UNITI: u8 = 0x04;
    pub const OP_UNITE: u8 = !OP_UNITI - OP_MASK;
    pub const OP_SWAPP: u8 = 0x05;
    pub const OP_SWAPP_R: u8 = !OP_SWAPP - OP_MASK;
    pub const OP_ASSRP: u8 = 0x06;
    pub const OP_ASSLP: u8 = !OP_ASSRP - OP_MASK;
    pub const OP_DIST: u8 = 0x07;
    pub const OP_FACT: u8 = !OP_DIST - OP_MASK;
    pub const OP_EXPN: u8 = 0x08;
    pub const OP_COLN: u8 = !OP_EXPN - OP_MASK;
    pub const OP_ADD: u8 = 0x0A;
    pub const OP_SUB: u8 = !OP_ADD - OP_MASK;
    pub const OP_ADDI: u8 = 0x0B;
    pub const OP_SUBI: u8 = !OP_ADDI - OP_MASK;
    pub const OP_MUL: u8 = 0x0C;
    pub const OP_DIV: u8 = !OP_MUL - OP_MASK;
    pub const OP_MULI: u8 = 0x0D;
    pub const OP_DIVI: u8 = !OP_MULI - OP_MASK;
    pub const OP_XOR: u8 = 0x0E;
    pub const OP_XOR_R: u8 = !OP_XOR - OP_MASK;
    pub const OP_XORI: u8 = 0x0F;
    pub const OP_XORI_R: u8 = !OP_XORI - OP_MASK;
    pub const OP_NEG: u8 = 0x10;
    pub const OP_NEG_R: u8 = !OP_NEG - OP_MASK;
    pub const OP_CSWAP: u8 = 0x11;
    pub const OP_CSWAP_R: u8 = !OP_CSWAP - OP_MASK;
    pub const OP_CSWAPI: u8 = 0x12;
    pub const OP_CSWAPI_R: u8 = !OP_CSWAPI - OP_MASK;
    pub const OP_RR: u8 = 0x13;
    pub const OP_RL: u8 = !OP_RR - OP_MASK;
    pub const OP_RRI: u8 = 0x14;
    pub const OP_RLI: u8 = !OP_RRI - OP_MASK;
    pub const OP_CALL: u8 = 0x19;
    pub const OP_UNCALL: u8 = !OP_CALL - OP_MASK;
    pub const OP_FOLW: u8 = 0x1A;
    pub const OP_RET: u8 = !OP_RET - OP_MASK;
    pub const OP_START: u8 = 0x1B;
    pub const OP_END: u8 = !OP_START - OP_MASK;
    pub const OP_EVAL: u8 = 0x1C;
    pub const OP_DEVAL: u8 = !OP_EVAL - OP_MASK;

    // C-Type
    pub const OP_EXPF: u8 = 0x09;
    pub const OP_COLF: u8 = !OP_EXPF - OP_MASK;
    pub const OP_LSUMC: u8 = 0x15;
    pub const OP_LSUMC_R: u8 = !OP_LSUMC - OP_MASK;
    pub const OP_RSUMC: u8 = 0x16;
    pub const OP_RSUMC_R: u8 = !OP_RSUMC - OP_MASK;
    pub const OP_LPRODC: u8 = 0x17;
    pub const OP_LPRODC_R: u8 = !OP_LPRODC - OP_MASK;
    pub const OP_RPRODC: u8 = 0x18;
    pub const OP_RPRODC_R: u8 = !OP_RPRODC - OP_MASK;

    // S-Type
    pub const OP_SWAPS: u8 = 0x02;
    pub const OP_SWAPS_R: u8 = !OP_SWAPS - OP_MASK;
    pub const OP_ASSRS: u8 = 0x03;
    pub const OP_ASSLS: u8 = !OP_ASSRS - OP_MASK;
}

impl AllocObject<ITypeId> for Opcode {
    const TYPE_ID: ITypeId = ITypeId::Opcode;
}

// Decoding Functions
pub fn get_opcode(instr: &Opcode) -> u8 { instr ^ const::OP_MASK }
pub fn decode_i(instr: &Opcode) -> u32 {
    (instr ^ const::I_MASK) >>> const::I_MASK
}

pub fn decode_c(instr: &Opcode) -> (u16, u16) {
    (
        (instr ^ const::C_OFF_MASK) >>> 0x3F,
        (instr ^ const::C_CONST_MASK) >>> const::C_CONST_MASK
    )
}

pub fn decode_s(instr: &Opcode) -> (u8, u8, u8) {
    (
        (instr ^ const::S_TOTAL_MASK) >>> 0x3F,
        (instr ^ const::S_DIV_MASK) >>> 0x3FFF,
        (instr ^ const::S_OFF_MASK) >>> const::S_OFF_MASK 
    )
}

#[cfg(test)]
mod test {
    use super::*;
}
