use crate::alloc::api::{AllocObject, RawPtr};
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
    val.data(guard)
}

pub fn swaps<'guard, T: AllocObject>(
    val: &ScopedPtr<'guard, Sum<T>>,
    div: Nat,
    _guard: &'guard dyn MutatorScope
) {
    let tag = val.tag();

    if tag <= div {
        if div != 0 {
            val.set_tag(tag + div);
        } else {
            val.set_tag(tag + 1);
        }
    } else {
        val.set_tag((tag - div) - 1);
    }
}

pub fn uniti<'guard, T>(
    val: ScopedPtr<'guard, T>,
    mem: &'guard MutatorView
) -> Result<Product<Unit, T>, RuntimeError>
    where T: AllocObject
{
    Ok(Product::new(
        CellPtr::new_with(mem.alloc(Unit::new())?),
        CellPtr::new_with(val)
    ))
}

pub fn unite<'guard, T>(
    val: ScopedPtr<'guard, Product<Unit, T>>,
    mem: &'guard dyn MutatorScope
) -> ScopedPtr<'guard, T>
    where T: AllocObject
{
    val.snd(mem)

}

pub fn swapp<'guard>(
    val: &ScopedPtr<'guard, Product<(), ()>>,
    guard: &'guard dyn MutatorScope
) {
    let fst = val.fst(guard);
    let snd = val.snd(guard);

    val.set_fst(snd);
    val.set_snd(fst);
}

pub fn assrp<'guard>(
    val: &ScopedPtr<'guard, Product<(), ()>>,
    guard: &'guard dyn MutatorScope
) {
    let inner_ptr = val.fst(guard);
    let inner = unsafe { inner_ptr.cast::<Product<(), ()>>(guard) };

    let a = inner.fst(guard);
    let b = inner.snd(guard);
    let c = val.snd(guard);

    inner.set_fst(b);
    inner.set_snd(c);
    val.set_fst(a);
    val.set_snd(inner.as_untyped(guard));
}

pub fn asslp<'guard>(
    val: &ScopedPtr<'guard, Product<(), ()>>,
    guard: &'guard dyn MutatorScope
) {
    let inner_ptr = val.snd(guard);
    let inner = unsafe { inner_ptr.cast::<Product<(), ()>>(guard) };

    let a = val.fst(guard);
    let b = inner.fst(guard);
    let c = inner.snd(guard);

    inner.set_fst(a);
    inner.set_snd(b);
    val.set_fst(inner.as_untyped(guard));
    val.set_snd(c);
}

pub fn dist<'guard, T: AllocObject>(
    val: ScopedPtr<'guard, Product<Sum<T>, ()>>,
    div: Nat,
    mem: &'guard MutatorView
) -> Result<ScopedPtr<'guard, Sum<Product<(), ()>>>, RuntimeError>
{
    let sum = val.fst(mem);
    let tag = sum.tag();

    if div == 0 {
        let new_sum = unsafe {
            sum.cast::<Sum<Product<(), ()>>>(mem)
        };
        let new_val = unsafe {
            val.cast::<Product<(), ()>>(mem)
        };
        
        new_val.set_fst(sum.data(mem).as_untyped(mem));
        new_sum.set_data(new_val);

        Ok(new_sum)
    } else if tag < div {
        val.set_fst(sum);
        let new_sum = mem.alloc(Sum::new(0, CellPtr::new_with(val)))?;

        Ok(unsafe {
            new_sum.cast::<Sum<Product<(), ()>>>(mem)
        })
    } else {
        sum.set_tag(tag - div);
        val.set_fst(sum);
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
    let inner = val.data(mem);
    let fst = inner.fst(mem);
    let tag = val.tag();
    
    if div == 0 {
        val.set_data(unsafe { fst.cast::<Product<(), ()>>(mem) });
        inner.set_fst(val.as_untyped(mem));

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
        let inner = cast_val.data(mem)
            .data(mem);

        if div == 0 {
            val.set_data(inner);
            val.set_tag(1);
            Ok(val)
        } else {
            let cast_inner = unsafe { inner.cast::<Sum<()>>(mem) };
            let inner_tag = cast_inner.tag();
            cast_inner.set_tag(inner_tag + div);

            mem.dealloc(cast_val.data(mem))?;
            mem.dealloc(cast_val)?;
            Ok(cast_inner)
        }
    } else {
        let inner = val.data(mem);

        if div == 0 {
            let neg = mem.alloc(Negative::new(CellPtr::new_with(inner)))?;
            val.set_data(unsafe { neg.cast::<()>(mem) });

            Ok(val)
        } else {
            val.set_tag(val.tag() - div);
            let neg = mem.alloc(Negative::new(CellPtr::new_with(val)))?;
            let sum = mem.alloc(Sum::new(0, CellPtr::new_with(neg)))?;

            Ok(unsafe { sum.cast::<Sum<()>>(mem) })
        }
    }
}

pub fn expf<'guard>(frac: &Fraction, mem: &'guard MutatorView)
    -> Result<Product<Fraction, ()>, RuntimeError>
{
    let val = mem.alloc_frac(frac.ptr().get(mem), frac.size())?;
    
    Ok(Product::new(
        CellPtr::new(RawPtr::new(frac)),
        CellPtr::new_with(val)
    ))
}

pub fn colf<'guard>(
    prod: ScopedPtr<'guard, Product<Fraction, ()>>,
    mem: &'guard MutatorView
) -> Result<(), RuntimeError>
{
    let frac = prod.fst(mem);
    let val = prod.snd(mem);

    mem.dealloc_frac(frac, val, frac.size())?;
    mem.dealloc(prod)
}

/*
 * Arithmetic 
 */
pub fn add<'guard>(
    val: &ScopedPtr<'guard, Product<Nat, Nat>>,
    guard: &'guard dyn MutatorScope
) {
    let fst = val.fst(guard).as_ref(guard);
    let snd = val.snd(guard).as_ref(guard);
    let mut fst_raw = val.fst(guard).as_rawptr(guard);
    let fst_mut = fst_raw.as_mut();

    *fst_mut = fst + snd;
}

pub fn sub<'guard>(
    val: &ScopedPtr<'guard, Product<Nat, Nat>>,
    guard: &'guard dyn MutatorScope
) -> Result<(), RuntimeError> {
    let fst = val.fst(guard).as_ref(guard);
    let snd = val.snd(guard).as_ref(guard);
    
    // checking for underflow
    if snd <= fst {
        let mut fst_raw = val.fst(guard).as_rawptr(guard);
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

pub fn xor<'guard>(
    val: &ScopedPtr<'guard, Product<Nat, Nat>>,
    guard: &'guard dyn MutatorScope
) {
    let fst = val.fst(guard).as_ref(guard);
    let snd = val.snd(guard).as_ref(guard);
    let mut fst_raw = val.fst(guard).as_rawptr(guard);
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
    let inner = val.fst(guard).as_ref(guard);
    let a = inner.fst(guard).as_ref(guard);
    let b = inner.snd(guard).as_ref(guard);
    let c = val.snd(guard).as_ref(guard);
    
    let mut a_raw = inner.fst(guard).as_rawptr(guard);
    let a_mut = a_raw.as_mut();
    let mut b_raw = inner.snd(guard).as_rawptr(guard);
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
    let fst = val.fst(guard).as_ref(guard);
    let snd = val.snd(guard).as_ref(guard);
    
    let mut fst_raw = val.fst(guard).as_rawptr(guard);
    let fst_mut = fst_raw.as_mut();
    let mut snd_raw = val.snd(guard).as_rawptr(guard);
    let snd_mut = snd_raw.as_mut();

    let s = (fst ^ snd) & operand;
    *fst_mut = fst ^ s;
    *snd_mut = snd ^ s;
}

pub fn rr<'guard>(
    val: &ScopedPtr<'guard, Product<Nat, Nat>>,
    guard: &'guard dyn MutatorScope
) {
    let fst = val.fst(guard).as_ref(guard);
    let snd = val.snd(guard).as_ref(guard);
    let mut fst_raw = val.fst(guard).as_rawptr(guard);
    let fst_mut = fst_raw.as_mut();

    *fst_mut = (fst >> snd) | (fst << (32 - snd));
}

pub fn rl<'guard>(
    val: &ScopedPtr<'guard, Product<Nat, Nat>>,
    guard: &'guard dyn MutatorScope
) {
    let fst = val.fst(guard).as_ref(guard);
    let snd = val.snd(guard).as_ref(guard);
    let mut fst_raw = val.fst(guard).as_rawptr(guard);
    let fst_mut = fst_raw.as_mut();

    *fst_mut = (fst << snd) | (fst >> (32 - snd));
}

pub fn rri<'guard>(
    val: &ScopedPtr<'guard, Nat>,
    operand: Nat,
    guard: &'guard dyn MutatorScope
) {
    let num = val.as_ref(guard);
    let mut val_raw = val.as_rawptr(guard);
    let val_mut = val_raw.as_mut();

    *val_mut = (num >> operand) | (num << (32 - operand));
}

pub fn rli<'guard>(
    val: &ScopedPtr<'guard, Nat>,
    operand: Nat,
    guard: &'guard dyn MutatorScope
) {
    let num = val.as_ref(guard);
    let mut val_raw = val.as_rawptr(guard);
    let val_mut = val_raw.as_mut();

    *val_mut = (num << operand) | (num >> (32 - operand));
}

pub fn lti<'guard>(
    val: ScopedPtr<'guard, Product<Nat, Nat>>,
    guard: &'guard dyn MutatorScope
) -> Sum<Product<Nat, Nat>> {
    let fst = val.fst(guard).as_ref(guard);
    let snd = val.snd(guard).as_ref(guard);

    if fst < snd {
        Sum::new(0, CellPtr::new_with(val))
    } else {
        Sum::new(1, CellPtr::new_with(val))
    }
}

pub fn lte<'guard>(
    val: ScopedPtr<'guard, Sum<Product<Nat, Nat>>>,
    guard: &'guard dyn MutatorScope
) -> Result<ScopedPtr<'guard, Product<Nat, Nat>>, RuntimeError> {
    let inner = val.data(guard);
    let fst = inner.fst(guard).as_ref(guard);
    let snd = inner.snd(guard).as_ref(guard);

    if (fst < snd && val.tag() == 0) || (fst >= snd && val.tag() == 1) {
        Ok(inner)
    } else {
        Err(RuntimeError::new(ErrorKind::LessThanElim))
    }
}

pub fn ltii<'guard>(
    val: ScopedPtr<'guard, Nat>,
    div: Nat,
    guard: &'guard dyn MutatorScope
) -> Sum<Nat> {
    let num = val.as_ref(guard);

    if *num < div {
        Sum::new(0, CellPtr::new_with(val))
    } else {
        Sum::new(1, CellPtr::new_with(val))
    }
}

pub fn ltei<'guard>(
    val: ScopedPtr<'guard, Sum<Nat>>,
    div: Nat,
    guard: &'guard dyn MutatorScope
) -> Result<ScopedPtr<'guard, Nat>, RuntimeError> {
    let num = val.data(guard).as_ref(guard);

    if (*num < div && val.tag() == 0) || (*num >= div && val.tag() == 1) {
        Ok(val.data(guard))
    } else {
        Err(RuntimeError::new(ErrorKind::LessThanElim))
    }
}
