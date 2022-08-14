use crate::array::{Array, ArraySize};
use crate::alloc::api::AllocObject;
use crate::data::ITypeId;
use crate::safeptr::UntypedPtr;

pub type ContextStack = Array<Context>;

/* Context Type */
#[derive(Clone)]
pub enum Context {
    Nil,
    First {
        snd_op_index: ArraySize,
        snd_val: UntypedPtr,
        root_val: UntypedPtr,
    },
    Second {
        fst_op_index: ArraySize,
        fst_val: UntypedPtr,
        root_val: UntypedPtr,
    },
    Left(ArraySize),
    Right(ArraySize),
    Indirect {
        last: UntypedPtr,
        current: UntypedPtr,
    },
    Call {
        last: ArraySize,
        current: ArraySize,
    }
}

impl AllocObject<ITypeId> for Context {
    const TYPE_ID: ITypeId = ITypeId::Context;
}
