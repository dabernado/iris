use crate::alloc::api::*;

#[derive(PartialEq, Copy, Clone)]
enum ITypeId {
    Unit,
    Int,
    UInt,
    Float,
    Bool,
    Sum,
    Prod,
    Frac,
    Neg,
    Array,
}
impl AllocTypeId for ITypeId {}

struct ITypeHeader {
    size: u32,
    size_class: SizeClass,
    type_id: ITypeId,
}

impl AllocHeader for ITypeHeader {
    type TypeId = ITypeId;

    fn new<O: AllocObject<Self::TypeId>>(size: u32, size_class: SizeClass) -> {
        ITypeHeader {
            size,
            size_class,
            type_id: O::TYPE_ID,
        }
    }
}

/* Primitive Types */
type Unit = ();
impl AllocObject<ITypeId> for Unit {
    const TYPE_ID: ITypeId = ITypeId::Unit;
}

struct Int { value: i32 }
impl AllocObject<ITypeId> for Int {
    const TYPE_ID: ITypeId = ITypeId::Int;
}

struct UInt { value: u32 }
impl AllocObject<ITypeId> for UInt {
    const TYPE_ID: ITypeId = ITypeId::UInt;
}

struct Float { value: f32 }
impl AllocObject<ITypeId> for Float {
    const TYPE_ID: ITypeId = ITypeId::Float;
}

type Bool = bool;
impl AllocObject<ITypeId> for Bool {
    const TYPE_ID: ITypeId = ITypeId::Bool;
}

/* Negative and Fractional Types */
struct Fraction<O: AllocObject<Self::TypeId>>(O);
impl AllocObject<ITypeId> for Fraction {
    const TYPE_ID: ITypeId = ITypeId::Frac;
}

struct Negative<O: AllocObject<Self::TypeId>>(O);
impl AllocObject<ITypeId> for Negative {
    const TYPE_ID: ITypeId = ITypeId::Neg;
}

/* Sum Types */
enum Sum<L: AllocObject<Self::TypeId>, R: AllocObject<Self::TypeId>> {
    Left(L),
    Right(R),
}

impl AllocObject<ITypeId> for Sum<L: AllocObject<Self::TypeId>, R: AllocObject<Self::TypeId>> {
    const TYPE_ID: ITypeId = ITypeId::Sum;
}

/* Product Types */
enum ProductValue<T: AllocObject<Self::TypeId>> {
    Unit,
    Int(i32),
    UInt(u32),
    Bool(bool),
    Pointer(RawPtr<T>),
}

struct Product<F: AllocObject<Self::TypeId>, S: AllocObject<Self::TypeId>> {

    fst: ProductValue<F>,
    snd: ProductValue<S>,
}

impl AllocObject<ITypeId> for Product<F: AllocObject<Self::TypeId>, S: AllocObject<Self::TypeId>> {
    const TYPE_ID: ITypeId = ITypeId::Prod;
}
