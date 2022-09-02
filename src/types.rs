use crate::alloc::api::AllocObject;

/* Type Enum */
#[derive(Clone)]
pub enum IType {
    Zero,
    Unit,
    Nat,
    Frac(Box<IType>),
    Neg(Box<IType>),
    Sum {
        left: Box<IType>,
        right: Box<IType>,
    },
    Prod {
        fst: Box<IType>,
        snd: Box<IType>,
    },
}

impl AllocObject for IType {}
