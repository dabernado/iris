use std::fmt;
use std::cell::Cell;

use crate::alloc::api::AllocObject;
use crate::memory::MutatorScope;
use crate::safeptr::{CellPtr, ScopedPtr, UntypedCellPtr};
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

// for rust typechecking
impl AllocObject for () {}

#[derive(Clone, PartialEq)]
pub struct Unit(u32); // has to represent some kind of data or alloc freaks out
impl AllocObject for Unit {}

impl Unit {
    pub fn new() -> Unit { Unit(0) }
}

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
 * nat + nat type
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
 * 1 + 1 type
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
#[derive(Clone)]
pub struct Fraction {
    ptr: UntypedCellPtr,
    size: Nat
}

impl AllocObject for Fraction {}
impl Fraction {
    pub fn new(ptr: UntypedCellPtr, size: Nat) -> Self {
        Fraction { ptr, size }
    }

    pub fn ptr(&self) -> &UntypedCellPtr { &self.ptr }
    pub fn size(&self) -> Nat { self.size }
}

impl Print for Fraction {
    fn print<'guard>(
        &self,
        _guard: &'guard dyn MutatorScope,
        f: &mut fmt::Formatter,
    ) -> fmt::Result
    {
        write!(f, "1/{{object, {} bytes}}", self.size)
    }
}

pub struct Negative<O: AllocObject>(CellPtr<O>);
impl<O: AllocObject> AllocObject for Negative<O> {}

impl<O: AllocObject> Negative<O> {
    pub fn new(data: CellPtr<O>) -> Negative<O> { Negative(data) }
}

impl<O: AllocObject> Negative<O> {
    pub fn data(&self) -> &CellPtr<O> { &self.0 }
}

impl<O: AllocObject + Print> Print for Negative<O> {
    fn print<'guard>(
        &self,
        guard: &'guard dyn MutatorScope,
        f: &mut fmt::Formatter,
    ) -> fmt::Result { write!(f, "-{}", self.0.get(guard)) }
}

#[derive(Clone)]
pub struct Sum<O: AllocObject> {
    tag: Cell<Nat>,
    data: CellPtr<O>,
}
impl<O: AllocObject> AllocObject for Sum<O> {}

impl<O: AllocObject> Sum<O> {
    pub fn new(tag: Nat, data: CellPtr<O>) -> Sum<O> {
        Sum { tag: Cell::new(tag), data }
    }

    pub fn set_tag(&self, tag: Nat) {
        self.tag.set(tag);
    }

    pub fn tag(&self) -> Nat { self.tag.get()}
    pub fn data(&self) -> &CellPtr<O> { &self.data }
}

impl<O: AllocObject + Print> Print for Sum<O> {
    fn print<'guard>(
        &self,
        guard: &'guard dyn MutatorScope,
        f: &mut fmt::Formatter,
    ) -> fmt::Result {
        write!(f, "e{} ", self.tag.get())?;
        write!(f, "({})", self.data.get(guard))
    }
}

#[derive(Clone)]
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
