// Bitmasks
pub const OP_MASK: u32 = 0x0000001F;
pub const I_MASK: u32 = 0xFFFFFFE0;
pub const S_LC_MASK: u32 = 0x0003FFE0;
pub const S_RC_MASK: u32 = 0x7FFC000;
pub const C_DIV_MASK: u32 = 0x00003FE0;
pub const C_LC_MASK: u32 = 0x007FC000;
pub const C_RC_MASK: u32 = 0xFF800000;

// I-Type
pub const OP_ID: u8 = 0;
pub const OP_ID_R: u8 = !OP_ID & (OP_MASK as u8);
pub const OP_ZEROI: u8 = 1;
pub const OP_ZEROE: u8 = !OP_ZEROI & (OP_MASK as u8);
pub const OP_ASSRS: u8 = 3;
pub const OP_ASSLS: u8 = !OP_ASSRS & (OP_MASK as u8);
pub const OP_UNITI: u8 = 4;
pub const OP_UNITE: u8 = !OP_UNITI & (OP_MASK as u8);
pub const OP_SWAPP: u8 = 5;
pub const OP_SWAPP_R: u8 = !OP_SWAPP & (OP_MASK as u8);
pub const OP_ASSRP: u8 = 6;
pub const OP_ASSLP: u8 = !OP_ASSRP & (OP_MASK as u8);
pub const OP_FOLD: u8 = 8;
pub const OP_UFOLD: u8 = !OP_FOLD & (OP_MASK as u8);
pub const OP_EXPN: u8 = 9;
pub const OP_COLN: u8 = !OP_EXPN & (OP_MASK as u8);
pub const OP_EXPF: u8 = 10;
pub const OP_COLF: u8 = !OP_EXPF & (OP_MASK as u8);
pub const OP_CALL: u8 = 11;
pub const OP_UNCALL: u8 = !OP_CALL & (OP_MASK as u8);
pub const OP_START: u8 = 12;
pub const OP_END: u8 = !OP_START & (OP_MASK as u8);
pub const OP_READ: u8 = 13;
pub const OP_WRITE: u8 = !OP_SYSC & (OP_MASK as u8);
pub const OP_PRODS: u8 = 15;
pub const OP_PRODE: u8 = !OP_PRODS & (OP_MASK as u8);

// S-Type
pub const OP_SWAPS: u8 = 2;
pub const OP_SWAPS_R: u8 = !OP_SWAPS & (OP_MASK as u8);
pub const OP_DIST: u8 = 7;
pub const OP_FACT: u8 = !OP_DIST & (OP_MASK as u8);

// C-Type
pub const OP_SUMS: u8 = 14;
pub const OP_SUME: u8 = !OP_SUMS & (OP_MASK as u8);

// Maximums/Minimums
pub const MAX_ITYPE_FIELD: u32 = 134217727;
pub const MAX_STYPE_FIELD: u16 = 8191;
pub const MAX_CTYPE_FIELD: u16 = 511;
