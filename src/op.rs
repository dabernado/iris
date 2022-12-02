use crate::alloc::api::{AllocObject, RawPtr};
use crate::array::*;
use crate::data::*;
use crate::error::RuntimeError;
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
    lc: u16,
    rc: u16,
    _guard: &'guard dyn MutatorScope
) {
    let tag = val.tag();

    if tag <= (lc - 1) as u32 {
        val.set_tag(tag + rc as u32);
    } else {
        val.set_tag(tag - lc as u32);
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

pub fn dist<'guard>(
    val: ScopedPtr<'guard, Product<Sum<()>, ()>>,
    lc: u16,
    rc: u16,
    mem: &'guard MutatorView
) -> Result<ScopedPtr<'guard, Sum<Product<(), ()>>>, RuntimeError>
{
    let sum = val.fst(mem);
    let tag = sum.tag();

    if tag <= (lc - 1) as u32 {
        if lc == 1 {
            let new_sum = unsafe {
                sum.cast::<Sum<Product<(), ()>>>(mem)
            };
            let new_val = unsafe {
                val.cast::<Product<(), ()>>(mem)
            };

            new_val.set_fst(sum.data(mem));
            new_sum.set_data(new_val);
            new_sum.set_tag(0);

            Ok(new_sum)
        } else {
            val.set_fst(sum);
            let new_sum = mem.alloc(Sum::new(0, CellPtr::new_with(val)))?;

            Ok(unsafe {
                new_sum.cast::<Sum<Product<(), ()>>>(mem)
            })
        }
    } else {
        if rc == 1 {
            let new_sum = unsafe {
                sum.cast::<Sum<Product<(), ()>>>(mem)
            };
            let new_val = unsafe {
                val.cast::<Product<(), ()>>(mem)
            };

            new_val.set_fst(sum.data(mem));
            new_sum.set_data(new_val);
            new_sum.set_tag(1);

            Ok(new_sum)
        } else {
            sum.set_tag(tag - lc as u32);
            val.set_fst(sum);
            let new_sum = mem.alloc(Sum::new(1, CellPtr::new_with(val)))?;

            Ok(unsafe {
                new_sum.cast::<Sum<Product<(), ()>>>(mem)
            })
        }
    }
}

pub fn fact<'guard>(
    val: ScopedPtr<'guard, Sum<Product<(), ()>>>,
    lc: u16,
    rc: u16,
    mem: &'guard MutatorView
) -> Result<ScopedPtr<'guard, Product</*Sum*/(), ()>>, RuntimeError>
{
    let prod = val.data(mem);
    let tag = val.tag();

    if tag == 0 {
        if lc == 1 {
            let fst = prod.fst(mem);
            let cast_val = unsafe { val.cast::<Sum<()>>(mem) };
            cast_val.set_data(fst);
            prod.set_fst(cast_val.as_untyped(mem));

            Ok(prod)
        } else {
            mem.dealloc(val)?;

            Ok(prod)
        }
    } else {
        if rc == 1 {
            let cast_val = unsafe { val.cast::<Sum<()>>(mem) };
            let fst = prod.fst(mem);
            cast_val.set_data(fst);
            cast_val.set_tag(lc as u32);
            prod.set_fst(cast_val.as_untyped(mem));

            Ok(prod)
        } else {
            let fst = prod.fst(mem);
            let cast_fst = unsafe { fst.cast::<Sum<()>>(mem) };
            cast_fst.set_tag(cast_fst.tag() + lc as u32);
            mem.dealloc(val)?;

            Ok(prod)
        }
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

pub fn fold<'guard>(
    val: ScopedPtr<'guard, Sum<()>>,
    mem: &'guard MutatorView
) -> Result<ScopedPtr<'guard, Inductive<()>>, RuntimeError>
{
    if val.tag() == 0 {
        let cast_val = unsafe { val.cast::<Sum<Unit>>(mem) };

        mem.dealloc(cast_val.data(mem))?;
        mem.dealloc(cast_val)?;
        Array::alloc(mem)
    } else {
        let cast_val = unsafe {
            val.cast::<Sum<Product<(), Inductive<()>>>>(mem)
        };

        let data = cast_val.data(mem);
        let inductive = data.snd(mem);

        inductive.push(mem, CellPtr::new_with(data.fst(mem)))?;
        mem.dealloc(data)?;
        mem.dealloc(cast_val)?;

        Ok(inductive)
    }
}

pub fn fold_nat<'guard>(
    val: ScopedPtr<'guard, Sum<Nat>>,
    mem: &'guard MutatorView
) -> Result<ScopedPtr<'guard, Nat>, RuntimeError>
{
    let nat = val.data(mem);
    let mut binding = nat.as_rawptr(mem);
    let nat_mut = binding.as_mut();

    *nat_mut = *nat_mut + 1;
    mem.dealloc(val)?;

    Ok(nat)
}

pub fn unfold<'guard>(
    val: ScopedPtr<'guard, Inductive<()>>,
    mem: &'guard MutatorView
) -> Result<ScopedPtr<'guard, Sum<()>>, RuntimeError>
{
    if val.length() == 0 {
        mem.dealloc(val)?;
        let ptr = mem.alloc(
            Sum::new(0, CellPtr::new_with(mem.alloc(Unit::new())?))
        )?;
        
        Ok(unsafe {
            ptr.cast::<Sum<()>>(mem)
        })
    } else {
        // alloc product
        let new_val = val.pop(mem)?.get(mem);
        let prod = mem.alloc(Product::new(
            CellPtr::new_with(new_val),
            CellPtr::new_with(val),
        ))?;
        let sum = mem.alloc(
            Sum::new(1, CellPtr::new_with(prod))
        )?;
        
        Ok(unsafe {
            sum.cast::<Sum<()>>(mem)
        })
    }
}

pub fn unfold_nat<'guard>(
    val: ScopedPtr<'guard, Nat>,
    mem: &'guard MutatorView
) -> Result<ScopedPtr<'guard, Sum<Nat>>, RuntimeError>
{
    let mut binding = val.as_rawptr(mem);
    let val_mut = binding.as_mut();
    *val_mut = *val_mut - 1;

    if *val_mut == 0 {
        mem.alloc(Sum::new(0, CellPtr::new_with(val)))
    } else {
        mem.alloc(Sum::new(1, CellPtr::new_with(val)))
    }
}
