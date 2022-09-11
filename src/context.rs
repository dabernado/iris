use crate::array::{Array, ArraySize};
use crate::alloc::api::AllocObject;
use crate::data::{Product, Sum};
use crate::safeptr::{CellPtr, UntypedCellPtr};

pub type ContextStack = Array<Context>;

/* Context Type */
#[derive(Clone)]
pub enum Context {
    Nil,
    First {
        snd_op_index: ArraySize,
        snd_val: UntypedCellPtr,
        root_val: CellPtr<Product<(), ()>>,
    },
    Second {
        fst_op_index: ArraySize,
        fst_val: UntypedCellPtr,
        root_val: CellPtr<Product<(), ()>>,
    },
    Left {
        right_op_index: ArraySize,
        jump: ArraySize,
        root_val: CellPtr<Sum<()>>,
    },
    Right {
        left_op_index: ArraySize,
        jump: ArraySize,
        root_val: CellPtr<Sum<()>>,
    },
    Call {
        last: ArraySize,
        current: ArraySize,
    },
    Uncall {
        last: ArraySize,
        current: ArraySize,
    }
}

impl AllocObject for Context {}
