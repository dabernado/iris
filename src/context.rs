use crate::array::{Array, ArraySize};
use crate::alloc::api::AllocObject;
use crate::safeptr::UntypedCellPtr;

pub type ContextStack = Array<Context>;

/* Context Type */
#[derive(Clone)]
pub enum Context {
    Nil,
    First {
        snd_op_index: ArraySize,
        snd_val: UntypedCellPtr,
        root_val: UntypedCellPtr,
    },
    Second {
        fst_op_index: ArraySize,
        fst_val: UntypedCellPtr,
        root_val: UntypedCellPtr,
    },
    Left(ArraySize),
    Right(ArraySize),
    Indirect {
        last: UntypedCellPtr,
        current: UntypedCellPtr,
    },
    Call {
        last: ArraySize,
        current: ArraySize,
    }
}

impl AllocObject for Context {}
