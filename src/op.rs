use crate::alloc::api::AllocObject;
use crate::data::*;
use crate::error::{RuntimeError, ErrorKind};
use crate::memory::{MutatorScope, MutatorView};
use crate::safeptr::{CellPtr, ScopedPtr};

/*
 * Functions
 */

pub fn zeroi<'guard, T>(val: ScopedPtr<'guard, T>) -> Sum<T>
    where T: AllocObject
{
    Sum::new(1, CellPtr::new_with(val))
}

pub fn zeroe<'guard, T>(
    val: ScopedPtr<'guard, Sum<T>>,
    guard: &'guard dyn MutatorScope
) -> ScopedPtr<'guard, T>
    where T: AllocObject
{
    val.data().get(guard)
}

pub fn uniti<'guard, T>(val: ScopedPtr<'guard, T>) -> Product<Unit, T>
    where T: AllocObject
{
    Product::new(CellPtr::<Unit>::new_unit(), CellPtr::new_with(val))
}

pub fn unite<'guard, T>(
    val: ScopedPtr<'guard, Product<Unit, T>>,
    mem: &'guard MutatorView
) -> Result<ScopedPtr<'guard, T>, RuntimeError>
    where T: AllocObject
{
    mem.dealloc(val.fst().get(mem))?;
    Ok(val.snd().get(mem))

}

pub fn swapp<'guard>(
    val: &ScopedPtr<'guard, Product<(), ()>>,
    guard: &'guard dyn MutatorScope
) {
    let fst = val.fst().get(guard);
    let snd = val.snd().get(guard);

    val.fst().set(snd);
    val.snd().set(fst);
}

pub fn assrp<'guard>(
    val: &ScopedPtr<'guard, Product<(), ()>>,
    guard: &'guard dyn MutatorScope
) {
    let inner_ptr = val.fst().get(guard);
    let inner = unsafe { inner_ptr.cast::<Product<(), ()>>(guard) };

    let a = inner.fst().get(guard);
    let b = inner.snd().get(guard);
    let c = val.snd().get(guard);

    inner.fst().set(b);
    inner.snd().set(c);
    val.fst().set(a);
    val.snd().set(inner.as_untyped(guard));
}

pub fn asslp<'guard>(
    val: &ScopedPtr<'guard, Product<(), ()>>,
    guard: &'guard dyn MutatorScope
) {
    let inner_ptr = val.snd().get(guard);
    let inner = unsafe { inner_ptr.cast::<Product<(), ()>>(guard) };

    let a = val.fst().get(guard);
    let b = inner.fst().get(guard);
    let c = inner.snd().get(guard);

    inner.fst().set(a);
    inner.snd().set(b);
    val.fst().set(inner.as_untyped(guard));
    val.snd().set(c);
}

/*
pub fn swaps<'guard>(
    val: &ScopedPtr<'guard, Sum<(), ()>>,
    guard: &'guard dyn MutatorScope
) {
}
*/
