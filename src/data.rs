use std::fmt;

use crate::alloc::api::*;
use crate::memory::MutatorScope;
use crate::safeptr::{CellPtr, ScopedPtr};
use crate::printer::*;

#[repr(u16)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ITypeId {
    Unit,
    Int,
    UInt,
    Float,
    Sum,
    Prod,
    Frac,
    Neg,
    Array,

    // Not exposed to programmer
    Zero,
    Bool,
    Func,
    Context,
    Continuation,
    Ptr,
}
impl AllocTypeId for ITypeId {}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct ITypeHeader {
    size: u32,
    size_class: SizeClass,
    type_id: ITypeId,
}

impl AllocHeader for ITypeHeader {
    type TypeId = ITypeId;

    fn new<O: AllocObject<Self::TypeId>>(
        size: u32,
        size_class: SizeClass,
    ) -> ITypeHeader {
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
    fn type_id(&self) -> Self::TypeId { self.type_id }
    fn size(&self) -> u32 { self.size }
}

/* Primitive Types */
// This type should NEVER be instantiated
pub struct Zero;

impl AllocObject<ITypeId> for Zero {
    const TYPE_ID: ITypeId = ITypeId::Zero;
}

impl Print for Zero {
    fn print<'guard>(
        &self,
        guard: &'guard dyn MutatorScope,
        f: &mut fmt::Formatter,
    ) -> fmt::Result { write!(f, "*") }
}

pub type Unit = ();

impl AllocObject<ITypeId> for Unit {
    const TYPE_ID: ITypeId = ITypeId::Unit;
}

impl Print for Unit {
    fn print<'guard>(
        &self,
        guard: &'guard dyn MutatorScope,
        f: &mut fmt::Formatter,
    ) -> fmt::Result { write!(f, "()") }
}

pub type Int = i32;

impl AllocObject<ITypeId> for Int {
    const TYPE_ID: ITypeId = ITypeId::Int;
}

impl Print for Int {
    fn print<'guard>(
        &self,
        guard: &'guard dyn MutatorScope,
        f: &mut fmt::Formatter,
    ) -> fmt::Result { write!(f, "{}", self) }
}

pub type UInt = u32;

impl AllocObject<ITypeId> for UInt {
    const TYPE_ID: ITypeId = ITypeId::UInt;
}

impl Print for UInt {
    fn print<'guard>(
        &self,
        guard: &'guard dyn MutatorScope,
        f: &mut fmt::Formatter,
    ) -> fmt::Result { write!(f, "{}", self) }
}

pub type Float = f32;

impl AllocObject<ITypeId> for Float {
    const TYPE_ID: ITypeId = ITypeId::Float;
}

impl Print for Float {
    fn print<'guard>(
        &self,
        guard: &'guard dyn MutatorScope,
        f: &mut fmt::Formatter,
    ) -> fmt::Result { write!(f, "{}", self) }
}

/*
 * bool is implemented in IRIS via the
 * 1 + 1 type, so this is not exposed
 * to the programmer
 */
pub type Bool = bool;

impl AllocObject<ITypeId> for Bool {
    const TYPE_ID: ITypeId = ITypeId::Bool;
}

impl Print for Bool {
    fn print<'guard>(
        &self,
        guard: &'guard dyn MutatorScope,
        f: &mut fmt::Formatter,
    ) -> fmt::Result { write!(f, "{}", self) }
}

/* Algebraic Data Types */
pub struct Fraction<O: AllocObject<ITypeId>>(CellPtr<O>);

impl<O: AllocObject<ITypeId>> AllocObject<ITypeId> for Fraction<O> {
    const TYPE_ID: ITypeId = ITypeId::Frac;
}

impl<O: AllocObject<ITypeId> + Print> Print for Fraction<O> {
    fn print<'guard>(
        &self,
        guard: &'guard dyn MutatorScope,
        f: &mut fmt::Formatter,
    ) -> fmt::Result { write!(f, "1/{}", self.0.get(guard)) }
}

pub struct Negative<O: AllocObject<ITypeId>>(CellPtr<O>);

impl<O: AllocObject<ITypeId>> AllocObject<ITypeId> for Negative<O> {
    const TYPE_ID: ITypeId = ITypeId::Neg;
}

impl<O: AllocObject<ITypeId> + Print> Print for Negative<O> {
    fn print<'guard>(
        &self,
        guard: &'guard dyn MutatorScope,
        f: &mut fmt::Formatter,
    ) -> fmt::Result { write!(f, "-{}", self.0.get(guard)) }
}

pub enum Sum<L: AllocObject<ITypeId>, R: AllocObject<ITypeId>> {
    Left(CellPtr<L>),
    Right(CellPtr<R>),
}

impl<L: AllocObject<ITypeId>, R: AllocObject<ITypeId>> AllocObject<ITypeId>
for Sum<L, R> {
    const TYPE_ID: ITypeId = ITypeId::Sum;
}

impl<L: AllocObject<ITypeId> + Print, R: AllocObject<ITypeId> + Print>
Print for Sum<L, R> {
    fn print<'guard>(
        &self,
        guard: &'guard dyn MutatorScope,
        f: &mut fmt::Formatter,
    ) -> fmt::Result {
        match self {
            Sum::Left(ptr) => write!(f, "left {}", ptr.get(guard)),
            Sum::Right(ptr) => write!(f, "right {}", ptr.get(guard)),
        }
    }
}

pub struct Product<F: AllocObject<ITypeId>, S: AllocObject<ITypeId>> {
    fst: CellPtr<F>,
    snd: CellPtr<S>,
}

impl<F: AllocObject<ITypeId>, S: AllocObject<ITypeId>> AllocObject<ITypeId>
for Product<F, S> {
    const TYPE_ID: ITypeId = ITypeId::Prod;
}

impl<F: AllocObject<ITypeId> + Print, S: AllocObject<ITypeId> + Print>
Print for Product<F, S> {
    fn print<'guard>(
        &self,
        guard: &'guard dyn MutatorScope,
        f: &mut fmt::Formatter,
    ) -> fmt::Result {
        let mut prod = ScopedPtr::new(guard, self);

        write!(f, "({}", prod.fst.get(guard))?;
        write!(f, ", {})", prod.snd.get(guard))
    }
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
