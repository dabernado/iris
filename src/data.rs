use crate::alloc::api::*;
use crate::safeptr::CellPtr;

#[repr(u16)]
#[derive(Debug, PartialEq, Copy, Clone)]
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
    Func,
    Context,
    Ptr,
}
impl AllocTypeId for ITypeId {}

pub struct ITypeHeader {
    size: u32,
    size_class: SizeClass,
    type_id: ITypeId,
}

impl AllocHeader for ITypeHeader {
    type TypeId = ITypeId;

    fn new<O: AllocObject<Self::TypeId>>(size: u32, size_class: SizeClass) -> ITypeHeader {
        ITypeHeader {
            size,
            size_class,
            type_id: O::TYPE_ID,
        }
    }

    fn new_array(size: u32, size_class: SizeClass) -> Self {
        ITypeHeader {
            size,
            size_class,
            type_id: ITypeId::Array,
        }
    }

    fn size_class(&self) -> SizeClass { self.size_class }
    fn size(&self) -> u32 { self.size }
    fn type_id(&self) -> Self::TypeId { self.type_id }
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

/*
 * bool is implemented via the 1 + 1 type, so
 * is not exposed to the programmer
 */
pub type Bool = bool;
impl AllocObject<ITypeId> for Bool {
    const TYPE_ID: ITypeId = ITypeId::Bool;
}

/* Algebraic Data Types */
pub struct Fraction<O: AllocObject<ITypeId>>(CellPtr<O>);
impl<O: AllocObject<ITypeId>> AllocObject<ITypeId> for Fraction<O> {
    const TYPE_ID: ITypeId = ITypeId::Frac;
}

pub struct Negative<O: AllocObject<ITypeId>>(CellPtr<O>);
impl<O: AllocObject<ITypeId>> AllocObject<ITypeId> for Negative<O> {
    const TYPE_ID: ITypeId = ITypeId::Neg;
}

pub enum Sum<L: AllocObject<ITypeId>, R: AllocObject<ITypeId>> {
    Left(CellPtr<L>),
    Right(CellPtr<R>),
}

impl<L: AllocObject<ITypeId>, R: AllocObject<ITypeId>> AllocObject<ITypeId>
for Sum<L, R> {
    const TYPE_ID: ITypeId = ITypeId::Sum;
}

pub struct Product<F: AllocObject<ITypeId>, S: AllocObject<ITypeId>> {
    fst: CellPtr<F>,
    snd: CellPtr<S>,
}

impl<F: AllocObject<ITypeId>, S: AllocObject<ITypeId>> AllocObject<ITypeId>
for Product<F, S> {
    const TYPE_ID: ITypeId = ITypeId::Prod;
}

/*
 * Helper functions
 */
pub fn is_atom<T: AllocObject<ITypeId>>(_object: &T) -> bool {
    match T::TYPE_ID {
        ITypeId::Unit => true,
        ITypeId::Int => true,
        ITypeId::UInt => true,
        ITypeId::Float => true,
        ITypeId::Bool => true,
        _ => false,
    }
}
