use crate::alloc::api::AllocObject;
use crate::data::*;
use crate::error::{RuntimeError, ErrorKind};
use crate::memory::{MutatorScope, MutatorView};
use crate::safeptr::{CellPtr, ScopedPtr};

/*
 * I-Type
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

pub fn swaps<'guard, T: AllocObject>(
    val: &ScopedPtr<'guard, Sum<T>>,
    div: Nat,
    guard: &'guard dyn MutatorScope
) {
    let tag = val.tag();
    if tag <= div {
        val.set_tag(tag + div);
    } else {
        val.set_tag(tag - div);
    }
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

pub fn dist<'guard, T: AllocObject>(
    val: ScopedPtr<'guard, Product<Sum<T>, ()>>,
    div: Nat,
    mem: &'guard MutatorView
) -> Result<ScopedPtr<'guard, Sum<Product<(), ()>>>, RuntimeError>
{
    let sum = val.fst().get(mem);
    let snd = val.snd().get(mem);
    let tag = sum.tag();

    if div == 0 {
        let new_sum = unsafe {
            sum.cast::<Sum<Product<(), ()>>>(mem)
        };
        let new_val = unsafe {
            val.cast::<Product<(), ()>>(mem)
        };
        
        new_val.fst().set(sum.data().get(mem).as_untyped(mem));
        new_sum.data().set(new_val);

        Ok(new_sum)
    } else if tag < div {
        val.fst().set(sum);
        let new_sum = mem.alloc(Sum::new(0, CellPtr::new_with(val)))?;

        Ok(unsafe {
            new_sum.cast::<Sum<Product<(), ()>>>(mem)
        })
    } else {
        sum.set_tag(tag - div);
        val.fst().set(sum);
        let new_sum = mem.alloc(Sum::new(1, CellPtr::new_with(val)))?;

        Ok(unsafe {
            new_sum.cast::<Sum<Product<(), ()>>>(mem)
        })
    }
}

pub fn fact<'guard>(
    val: ScopedPtr<'guard, Sum<Product<(), ()>>>,
    div: Nat,
    mem: &'guard MutatorView
) -> Result<ScopedPtr<'guard, Product<(), ()>>, RuntimeError>
{
    let inner = val.data().get(mem);
    let fst = inner.fst().get(mem);
    let snd = inner.snd().get(mem);
    let tag = val.tag();
    
    if div == 0 {
        val.data().set(unsafe { fst.cast::<Product<(), ()>>(mem) });
        inner.fst().set(val.as_untyped(mem));

        Ok(inner)
    } else if tag == 0 {
        mem.dealloc(val)?;

        Ok(inner)
    } else {
        let cast_fst = unsafe { fst.cast::<Sum<()>>(mem) };
        cast_fst.set_tag(cast_fst.tag() + div);
        mem.dealloc(val)?;

        Ok(inner)
    }
}
