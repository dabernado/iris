trait DataType {
}

enum Data<T: DataType> {
    Zero,
    Unit,
    Int(i32),
    Bool(bool),
    Left(T),
    Right(T),
    Prod(T, T),
    Neg(T),
    Frac(T),
}

trait Code {
}

enum Op {
    Id,
    ZeroE,
    ZeroI,
    SwapS,
    AssoclS,
    AssocrS,
    UnitE,
    UnitI,
    IntE,
    IntI,
    SwapP,
    AssoclP,
    AssocrP,
    Distrib,
    Factor,
    ExpandN,
    CollapseN,
    ExpandF,
    CollapseF,
    Add,
    Sub,
    AddI(i32),
    SubI(i32),
    Mul,
    Div,
    MulI(i32),
    DivI(i32),
    Xor,
    XorI(i32),
    Neg,
    Cswap,
    CswapI(i32),
    RotateR,
    RotateL,
    RotateRI(i32),
    RotateLI(i32),
    Call(&Function),
    Uncall(&Function),
    Exch(i32, i32, i32),
}

enum Combinator<T: Code> {
    Sum {
        left: Vec<T>,
        right: Vec<T>,
    },
    Prod {
        fst: Vec<T>,
        snd: Vec<T>,
    },
}

struct Function<T: Code> {
    body: Vec<T>,
}
