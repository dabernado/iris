use crate::alloc::api::*;

#[derive(PartialEq, Copy, Clone)]
pub enum ITypeId {
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

pub struct ITypeHeader {
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
pub type Unit = ();
impl AllocObject<ITypeId> for Unit {
    const TYPE_ID: ITypeId = ITypeId::Unit;
}

pub type Int = i32;
impl AllocObject<ITypeId> for Int {
    const TYPE_ID: ITypeId = ITypeId::Int;
}

pub type UInt = u32;
impl AllocObject<ITypeId> for UInt {
    const TYPE_ID: ITypeId = ITypeId::UInt;
}

pub type Float = f32;
impl AllocObject<ITypeId> for Float {
    const TYPE_ID: ITypeId = ITypeId::Float;
}

pub type Bool = bool;
impl AllocObject<ITypeId> for Bool {
    const TYPE_ID: ITypeId = ITypeId::Bool;
}

/* Algebraic Data Types */
pub struct Fraction<O: AllocObject<Self::TypeId>>(RawPtr<O>);
impl<O: AllocObject<Self::TypeId>> AllocObject<ITypeId> for Fraction<O> {
    const TYPE_ID: ITypeId = ITypeId::Frac;
}

pub struct Negative<O: AllocObject<Self::TypeId>>(RawPtr<O>);
impl<O: AllocObject<Self::TypeId>> AllocObject<ITypeId> for Negative<O> {
    const TYPE_ID: ITypeId = ITypeId::Neg;
}

pub enum Sum<L: AllocObject<Self::TypeId>, R: AllocObject<Self::TypeId>> {
    Left(RawPtr<L>),
    Right(RawPtr<R>),
}

impl<L: AllocObject<Self::TypeId>,
     R: AllocObject<Self::TypeId>> AllocObject<ITypeId> for Sum<L, R> {
    const TYPE_ID: ITypeId = ITypeId::Sum;
}

pub struct Product<F: AllocObject<Self::TypeId>, S: AllocObject<Self::TypeId>> {
    fst: RawPtr<F>,
    snd: RawPtr<S>,
}

impl<F: AllocObject<Self::TypeId>, S: AllocObject<Self::TypeId>> Product<F, S> {
    pub fn new(first: RawPtr<F>, second: RawPtr<S>) -> Product<F, S> {
        Product { first, second }
    }
}

impl<F: AllocObject<Self::TypeId>,
     S: AllocObject<Self::TypeId>> AllocObject<ITypeId> for Product<F, S> {
    const TYPE_ID: ITypeId = ITypeId::Prod;
}

/*
 * Helper functions
 */
pub fn is_atom<T: AllocObject<Self::TypeId>>(object: &T) -> bool {
    match T::TYPE_ID {
        ITypeId::Unit => true,
        ITypeId::Int => true,
        ITypeId::UInt => true,
        ITypeId::Float => true,
        ITypeId::Bool => true,
        _ => false,
    }
}
