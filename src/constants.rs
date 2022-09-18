// Bitmasks
pub const OP_MASK: u32 = 0x0000003F;
pub const I_MASK: u32 = 0xFFFFFFC0;
pub const C_OFF_MASK: u32 = 0x0007FFC0;
pub const C_CONST_MASK: u32 = 0xFFF8000;
pub const S_DIV_MASK: u32 = 0x00003FC0;
pub const S_LC_MASK: u32 = 0x003FC000;
pub const S_RC_MASK: u32 = 0x3FC00000;

// I-Type
pub const OP_ID: u8 = 0;
pub const OP_ID_R: u8 = !OP_ID - 192;
pub const OP_ZEROI: u8 = 1;
pub const OP_ZEROE: u8 = !OP_ZEROI - 192;
pub const OP_SWAPS: u8 = 2;
pub const OP_SWAPS_R: u8 = !OP_SWAPS - 192;
pub const OP_ASSRS: u8 = 3;
pub const OP_ASSLS: u8 = !OP_ASSRS - 192;
pub const OP_UNITI: u8 = 4;
pub const OP_UNITE: u8 = !OP_UNITI - 192;
pub const OP_SWAPP: u8 = 5;
pub const OP_SWAPP_R: u8 = !OP_SWAPP - 192;
pub const OP_ASSRP: u8 = 6;
pub const OP_ASSLP: u8 = !OP_ASSRP - 192;
pub const OP_DIST: u8 = 7;
pub const OP_FACT: u8 = !OP_DIST - 192;
pub const OP_EXPN: u8 = 8;
pub const OP_COLN: u8 = !OP_EXPN - 192;
pub const OP_ADD: u8 = 10;
pub const OP_SUB: u8 = !OP_ADD - 192;
pub const OP_ADDI: u8 = 11;
pub const OP_SUBI: u8 = !OP_ADDI - 192;
pub const OP_XOR: u8 = 12;
pub const OP_XOR_R: u8 = !OP_XOR - 192;
pub const OP_XORI: u8 = 13;
pub const OP_XORI_R: u8 = !OP_XORI - 192;
pub const OP_CSWAP: u8 = 14;
pub const OP_CSWAP_R: u8 = !OP_CSWAP - 192;
pub const OP_CSWAPI: u8 = 15;
pub const OP_CSWAPI_R: u8 = !OP_CSWAPI - 192;
pub const OP_RR: u8 = 16;
pub const OP_RL: u8 = !OP_RR - 192;
pub const OP_RRI: u8 = 17;
pub const OP_RLI: u8 = !OP_RRI - 192;
pub const OP_LTI: u8 = 18;
pub const OP_LTE: u8 = !OP_LTI - 192;
pub const OP_LTII: u8 = 19;
pub const OP_LTEI: u8 = !OP_LTII - 192;
pub const OP_CALL: u8 = 22;
pub const OP_UNCALL: u8 = !OP_CALL - 192;
pub const OP_START: u8 = 23;
pub const OP_END: u8 = !OP_START - 192;
pub const OP_SYSC: u8 = 24;
pub const OP_RSYSC: u8 = !OP_SYSC - 192;

// C-Type
pub const OP_EXPF: u8 = 9;
pub const OP_COLF: u8 = (!OP_EXPF >> 2) - 1;
pub const OP_SUMS: u8 = 20;
pub const OP_SUME: u8 = (!OP_SUMS >> 2) - 1;
pub const OP_PRODS: u8 = 21;
pub const OP_PRODE: u8 = (!OP_PRODS >> 2) - 1;

// Maximums/Minimums
pub const MAX_ITYPE_FIELD: u32 = 67108864;
pub const MAX_CTYPE_FIELD: u16 = 8192;
