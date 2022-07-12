// Bitmasks
pub const OP_MASK: u32 = 0xFFFFFFC0;
pub const I_MASK: u32 = 0x0000003F;
pub const C_OFF_MASK: u32 = 0xFFF8003F;
pub const C_CONST_MASK: u32 = 0x0007FFFF;
pub const S_TOTAL_MASK: u32 = 0xFFFFC03F;
pub const S_DIV_MASK: u32 = 0xFFC03FFF;
pub const S_OFF_MASK: u32 = 0xC03FFFFF;

// I-Type
pub const OP_ID: u8 = 0;
pub const OP_ID_R: u8 = !OP_ID ^ (OP_MASK as u8);
pub const OP_ZEROI: u8 = 1;
pub const OP_ZEROE: u8 = !OP_ZEROI ^ (OP_MASK as u8);
pub const OP_UNITI: u8 = 4;
pub const OP_UNITE: u8 = !OP_UNITI ^ (OP_MASK as u8);
pub const OP_SWAPP: u8 = 5;
pub const OP_SWAPP_R: u8 = !OP_SWAPP ^ (OP_MASK as u8);
pub const OP_ASSRP: u8 = 6;
pub const OP_ASSLP: u8 = !OP_ASSRP ^ (OP_MASK as u8);
pub const OP_DIST: u8 = 7;
pub const OP_FACT: u8 = !OP_DIST ^ (OP_MASK as u8);
pub const OP_EXPN: u8 = 8;
pub const OP_COLN: u8 = !OP_EXPN ^ (OP_MASK as u8);
pub const OP_ADD: u8 = 10;
pub const OP_SUB: u8 = !OP_ADD ^ (OP_MASK as u8);
pub const OP_ADDI: u8 = 11;
pub const OP_SUBI: u8 = !OP_ADDI ^ (OP_MASK as u8);
pub const OP_MUL: u8 = 12;
pub const OP_DIV: u8 = !OP_MUL ^ (OP_MASK as u8);
pub const OP_MULI: u8 = 13;
pub const OP_DIVI: u8 = !OP_MULI ^ (OP_MASK as u8);
pub const OP_UADD: u8 = 14;
pub const OP_USUB: u8 = !OP_ADD ^ (OP_MASK as u8);
pub const OP_UADDI: u8 = 15;
pub const OP_USUBI: u8 = !OP_ADDI ^ (OP_MASK as u8);
pub const OP_UMUL: u8 = 16;
pub const OP_UDIV: u8 = !OP_MUL ^ (OP_MASK as u8);
pub const OP_UMULI: u8 = 17;
pub const OP_UDIVI: u8 = !OP_MULI ^ (OP_MASK as u8);
pub const OP_XOR: u8 = 18;
pub const OP_XOR_R: u8 = !OP_XOR ^ (OP_MASK as u8);
pub const OP_XORI: u8 = 19;
pub const OP_XORI_R: u8 = !OP_XORI ^ (OP_MASK as u8);
pub const OP_NEG: u8 = 20;
pub const OP_NEG_R: u8 = !OP_NEG ^ (OP_MASK as u8);
pub const OP_CSWAP: u8 = 21;
pub const OP_CSWAP_R: u8 = !OP_CSWAP ^ (OP_MASK as u8);
pub const OP_CSWAPI: u8 = 22;
pub const OP_CSWAPI_R: u8 = !OP_CSWAPI ^ (OP_MASK as u8);
pub const OP_RR: u8 = 23;
pub const OP_RL: u8 = !OP_RR ^ (OP_MASK as u8);
pub const OP_RRI: u8 = 24;
pub const OP_RLI: u8 = !OP_RRI ^ (OP_MASK as u8);
pub const OP_CALL: u8 = 27;
pub const OP_UNCALL: u8 = !OP_CALL ^ (OP_MASK as u8);
pub const OP_FOLW: u8 = 28;
pub const OP_RET: u8 = !OP_RET ^ (OP_MASK as u8);
pub const OP_START: u8 = 29;
pub const OP_END: u8 = !OP_START ^ (OP_MASK as u8);
pub const OP_EVAL: u8 = 30;
pub const OP_DEVAL: u8 = !OP_EVAL ^ (OP_MASK as u8);

// C-Type
pub const OP_EXPF: u8 = 9;
pub const OP_COLF: u8 = !OP_EXPF ^ (OP_MASK as u8);
pub const OP_SUMC: u8 = 25;
pub const OP_SUMC_R: u8 = !OP_SUMC ^ (OP_MASK as u8);
pub const OP_PRODC: u8 = 26;
pub const OP_PRODC_R: u8 = !OP_PRODC ^ (OP_MASK as u8);

// S-Type
pub const OP_SWAPS: u8 = 2;
pub const OP_SWAPS_R: u8 = !OP_SWAPS ^ (OP_MASK as u8);
pub const OP_ASSRS: u8 = 3;
pub const OP_ASSLS: u8 = !OP_ASSRS ^ (OP_MASK as u8);
