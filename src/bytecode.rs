use crate::alloc::api::{AllocObject, ITypeId};
use crate::safeptr::UntypedPtr;
use crate::array::ArraySize;

#[derive(Copy, Clone)]
pub enum Opcode {
    // I-Type
    Id,
    ZeroI,
    ZeroE,
    UnitI,
    UnitE,
    SwapP,
    AssrP,
    AsslP,
    Dist,
    Fact,
    ExpN,
    ColN,
    Add,
    Sub,
    Mul,
    Div,
    Xor,
    Neg,
    Cswap,
    Rr,
    Rl,
    Start,
    Return,
    Eval,
    Deval,
    AddI(u32),
    SubI(u32),
    MulI(u32),
    DivI(u32),
    XorI(u32),
    NegI(u32),
    CswapI(u32),
    RrI(u32),
    RlI(u32),
    Call(ArraySize),
    Uncall(ArraySize),

    // C-Type
    ExpF {
        ptr: UntypedPtr,
        size: u16,
    },
    ColF {
        ptr: UntypedPtr,
        size: u16,
    },
    SumC {
        offset: u16,
        div: u16,
    },
    RSumC {
        offset: u16,
        div: u16,
    },
    ProdC {
        offset: u16,
        ptr: UntypedPtr,
    },
    RProdC {
        offset: u16,
        ptr: UntypedPtr,
    },

    // S-Type
    SwapS {
        total: u8,
        div: u8,
        offset: u8,
    },
    AssrS {
        total: u8,
        div: u8,
        offset: u8,
    },
    AsslS {
        total: u8,
        div: u8,
        offset: u8,
    },
}

impl AllocObject<ITypeId> for Opcode {
    const TYPE_ID: ITypeId = ITypeId::Opcode;
}
