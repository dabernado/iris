/* Type Enum */
pub enum IType {
    Zero,
    Unit,
    Nat,
    Frac(Box<IType>),
    Neg(Box<IType>)
    Sum {
        left: Box<IType>,
        right: Box<IType>,
    },
    Prod {
        fst: Box<IType>,
        snd: Box<IType>,
    },
    Iso {
        input: Box<IType>,
        output: Box<IType>,
    },
}
