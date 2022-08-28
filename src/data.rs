use std::fmt;

use crate::alloc::api::AllocObject;
use crate::memory::MutatorScope;
use crate::safeptr::{CellPtr, ScopedPtr};
use crate::printer::*;

/* Primitive Types */
// This type should NEVER be instantiated
pub struct Zero;
impl AllocObject for Zero {}

impl Print for Zero {
    fn print<'guard>(
        &self,
        _guard: &'guard dyn MutatorScope,
        f: &mut fmt::Formatter,
    ) -> fmt::Result { write!(f, "*") }
}

pub type Unit = ();
impl AllocObject for Unit {}

impl Print for Unit {
    fn print<'guard>(
        &self,
        _guard: &'guard dyn MutatorScope,
        f: &mut fmt::Formatter,
    ) -> fmt::Result { write!(f, "()") }
}

pub type Nat = u32;
impl AllocObject for Nat {}

impl Print for Nat {
    fn print<'guard>(
        &self,
        _guard: &'guard dyn MutatorScope,
        f: &mut fmt::Formatter,
    ) -> fmt::Result { write!(f, "{}", self) }
}

/*
 * int is implemented in IRIS via the
 * nat + nat type, but is represented
 * as a normal signed integer
 */
pub type Int = i32;
impl AllocObject for Int {}

impl Print for Int {
    fn print<'guard>(
        &self,
        _guard: &'guard dyn MutatorScope,
        f: &mut fmt::Formatter,
    ) -> fmt::Result { write!(f, "{}", self) }
}

/*
 * bool is implemented in IRIS via the
 * 1 + 1 type, but is represented as
 * a traditional boolean
 */
pub type Bool = bool;
impl AllocObject for Bool {}

impl Print for Bool {
    fn print<'guard>(
        &self,
        _guard: &'guard dyn MutatorScope,
        f: &mut fmt::Formatter,
    ) -> fmt::Result { write!(f, "{}", self) }
}

/* Algebraic Data Types */
pub struct Fraction<O: AllocObject>(CellPtr<O>);
impl<O: AllocObject> AllocObject for Fraction<O> {}

impl<O: AllocObject + Print> Print for Fraction<O> {
    fn print<'guard>(
        &self,
        guard: &'guard dyn MutatorScope,
        f: &mut fmt::Formatter,
    ) -> fmt::Result { write!(f, "1/{}", self.0.get(guard)) }
}

pub struct Negative<O: AllocObject>(CellPtr<O>);
impl<O: AllocObject> AllocObject for Negative<O> {}

impl<O: AllocObject + Print> Print for Negative<O> {
    fn print<'guard>(
        &self,
        guard: &'guard dyn MutatorScope,
        f: &mut fmt::Formatter,
    ) -> fmt::Result { write!(f, "-{}", self.0.get(guard)) }
}

pub enum Sum<L: AllocObject, R: AllocObject> {
    Left(CellPtr<L>),
    Right(CellPtr<R>),
}
impl<L: AllocObject, R: AllocObject> AllocObject for Sum<L, R> {}

impl<L: AllocObject + Print, R: AllocObject + Print> Print for Sum<L, R> {
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

pub struct Product<F: AllocObject, S: AllocObject> {
    fst: CellPtr<F>,
    snd: CellPtr<S>,
}
impl<F: AllocObject, S: AllocObject> AllocObject for Product<F, S> {}

impl<F: AllocObject, S: AllocObject> Product<F, S> {
    pub fn new(fst: CellPtr<F>, snd: CellPtr<S>) -> Product<F, S> {
        Product { fst, snd }
    }

    pub fn fst(&self) -> &CellPtr<F> { &self.fst }
    pub fn snd(&self) -> &CellPtr<S> { &self.snd }
}

impl<F: AllocObject + Print, S: AllocObject + Print> Print for Product<F, S> {
    fn print<'guard>(
        &self,
        guard: &'guard dyn MutatorScope,
        f: &mut fmt::Formatter,
    ) -> fmt::Result {
        let prod = ScopedPtr::new(guard, self);

        write!(f, "({}", prod.fst.get(guard))?;
        write!(f, ", {})", prod.snd.get(guard))
    }
}
