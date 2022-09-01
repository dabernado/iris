use std::ops::Deref;

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

pub fn expn<'guard>(
    val: ScopedPtr<'guard, Sum<()>>,
    div: Nat,
    mem: &'guard MutatorView
) -> Result<ScopedPtr<'guard, Sum<()>>, RuntimeError>
{
    if val.tag() == 0 {
        let cast_val = unsafe { val.cast::<Sum<Negative<()>>>(mem) };
        let inner = cast_val.data().get(mem)
            .data().get(mem);

        if div == 0 {
            val.data().set(inner);
            val.set_tag(1);
            Ok(val)
        } else {
            let cast_inner = unsafe { inner.cast::<Sum<()>>(mem) };
            let inner_tag = cast_inner.tag();
            cast_inner.set_tag(inner_tag + div);

            mem.dealloc(cast_val.data().get(mem))?;
            mem.dealloc(cast_val)?;
            Ok(cast_inner)
        }
    } else {
        let inner = val.data().get(mem);

        if div == 0 {
            let neg = mem.alloc(Negative::new(CellPtr::new_with(inner)))?;
            val.data().set(unsafe { neg.cast::<()>(mem) });

            Ok(val)
        } else {
            val.set_tag(val.tag() - div);
            let neg = mem.alloc(Negative::new(CellPtr::new_with(val)))?;
            let sum = mem.alloc(Sum::new(0, CellPtr::new_with(neg)))?;

            Ok(unsafe { sum.cast::<Sum<()>>(mem) })
        }
    }
}

/*
 * Arithmetic 
 */
pub fn add<'guard>(
    val: &ScopedPtr<'guard, Product<Nat, Nat>>,
    guard: &'guard dyn MutatorScope
) {
    let fst = val.fst().get(guard).as_ref(guard);
    let snd = val.snd().get(guard).as_ref(guard);
    let mut fst_raw = val.fst().get(guard).as_rawptr(guard);
    let fst_mut = fst_raw.as_mut();

    *fst_mut = fst + snd;
}

pub fn sub<'guard>(
    val: &ScopedPtr<'guard, Product<Nat, Nat>>,
    guard: &'guard dyn MutatorScope
) -> Result<(), RuntimeError> {
    let fst = val.fst().get(guard).as_ref(guard);
    let snd = val.snd().get(guard).as_ref(guard);
    
    // checking for underflow
    if snd <= fst {
        let mut fst_raw = val.fst().get(guard).as_rawptr(guard);
        let fst_mut = fst_raw.as_mut();

        *fst_mut = fst + snd;
        Ok(())
    } else {
        Err(RuntimeError::new(ErrorKind::IntOverflow))
    }
}

pub fn addi<'guard>(
    val: &ScopedPtr<'guard, Nat>,
    operand: Nat,
    guard: &'guard dyn MutatorScope
) {
    let num = val.as_ref(guard);
    let mut val_raw = val.as_rawptr(guard);
    let val_mut = val_raw.as_mut();

    *val_mut = num + operand;
}

pub fn subi<'guard>(
    val: &ScopedPtr<'guard, Nat>,
    operand: Nat,
    guard: &'guard dyn MutatorScope
) -> Result<(), RuntimeError> {
    let num = val.as_ref(guard);
    
    // checking for overflow
    if operand <= *num {
        let mut val_raw = val.as_rawptr(guard);
        let val_mut = val_raw.as_mut();

        *val_mut = num - operand;
        Ok(())
    } else {
        Err(RuntimeError::new(ErrorKind::IntOverflow))
    }
}

pub fn mul<'guard>(
    val: &ScopedPtr<'guard, Product<Nat, Nat>>,
    guard: &'guard dyn MutatorScope
) -> Result<(), RuntimeError> {
    let fst = val.fst().get(guard).as_ref(guard);
    let snd = val.snd().get(guard).as_ref(guard);
    
    // checking if multiplying by 0
    if *snd == 0 {
        let mut fst_raw = val.fst().get(guard).as_rawptr(guard);
        let fst_mut = fst_raw.as_mut();

        *fst_mut = fst * snd;
        Ok(())
    } else {
        Err(RuntimeError::new(ErrorKind::MulOrDivBy0))
    }
}

pub fn div<'guard>(
    val: &ScopedPtr<'guard, Product<Nat, Nat>>,
    guard: &'guard dyn MutatorScope
) -> Result<(), RuntimeError> {
    let fst = val.fst().get(guard).as_ref(guard);
    let snd = val.snd().get(guard).as_ref(guard);
    
    // checking if multiplying by 0
    if *snd == 0 {
        let mut fst_raw = val.fst().get(guard).as_rawptr(guard);
        let fst_mut = fst_raw.as_mut();

        *fst_mut = fst / snd;
        Ok(())
    } else {
        Err(RuntimeError::new(ErrorKind::MulOrDivBy0))
    }
}

pub fn muli<'guard>(
    val: &ScopedPtr<'guard, Nat>,
    operand: Nat,
    guard: &'guard dyn MutatorScope
) {
    let num = val.as_ref(guard);
    let mut val_raw = val.as_rawptr(guard);
    let val_mut = val_raw.as_mut();

    *val_mut = num * operand;
}

pub fn divi<'guard>(
    val: &ScopedPtr<'guard, Nat>,
    operand: Nat,
    guard: &'guard dyn MutatorScope
) {
    let num = val.as_ref(guard);
    let mut val_raw = val.as_rawptr(guard);
    let val_mut = val_raw.as_mut();

    *val_mut = num / operand;
}

pub fn xor<'guard>(
    val: &ScopedPtr<'guard, Product<Nat, Nat>>,
    guard: &'guard dyn MutatorScope
) {
    let fst = val.fst().get(guard).as_ref(guard);
    let snd = val.snd().get(guard).as_ref(guard);
    let mut fst_raw = val.fst().get(guard).as_rawptr(guard);
    let fst_mut = fst_raw.as_mut();

    *fst_mut = fst ^ snd;
}

pub fn xori<'guard>(
    val: &ScopedPtr<'guard, Nat>,
    operand: Nat,
    guard: &'guard dyn MutatorScope
) {
    let num = val.as_ref(guard);
    let mut val_raw = val.as_rawptr(guard);
    let val_mut = val_raw.as_mut();

    *val_mut = num ^ operand;
}

pub fn cswap<'guard>(
    val: &ScopedPtr<'guard, Product<Product<Nat, Nat>, Nat>>,
    guard: &'guard dyn MutatorScope
) {
    let inner = val.fst().get(guard).as_ref(guard);
    let a = inner.fst().get(guard).as_ref(guard);
    let b = inner.snd().get(guard).as_ref(guard);
    let c = val.snd().get(guard).as_ref(guard);
    
    let mut a_raw = inner.fst().get(guard).as_rawptr(guard);
    let a_mut = a_raw.as_mut();
    let mut b_raw = inner.snd().get(guard).as_rawptr(guard);
    let b_mut = b_raw.as_mut();

    let s = (a ^ b) & c;
    *a_mut = a ^ s;
    *b_mut = b ^ s;
}

pub fn cswapi<'guard>(
    val: &ScopedPtr<'guard, Product<Nat, Nat>>,
    operand: Nat,
    guard: &'guard dyn MutatorScope
) {
    let fst = val.fst().get(guard).as_ref(guard);
    let snd = val.snd().get(guard).as_ref(guard);
    
    let mut fst_raw = val.fst().get(guard).as_rawptr(guard);
    let fst_mut = fst_raw.as_mut();
    let mut snd_raw = val.snd().get(guard).as_rawptr(guard);
    let snd_mut = snd_raw.as_mut();

    let s = (fst ^ snd) & operand;
    *fst_mut = fst ^ s;
    *snd_mut = snd ^ s;
}
