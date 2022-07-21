use crate::array::{Array, ArraySize};
use crate::data::*;
use crate::error::{RuntimeError, ErrorKind};
use crate::memory::MutatorView;
use crate::safeptr::{UntypedPtr, ScopedPtr, FuncPtr, CellPtr};

pub fn zeroi<T: AllocObject<ITypeId>>(
    mem: &'guard MutatorView,
    val: RawPtr<T>
) -> Result<Sum<Zero, T>, RuntimeError> {
    Sum::Right(CellPtr::new_with(ScopedPtr::new(mem, val))?)
}


pub fn zeroe<T: AllocObject<ITypeId>>(
    mem: &'guard MutatorView,
    val: RawPtr<Sum<Zero, T>>
) -> Result<ScopedPtr<'guard, T>, RuntimeError> {
    if let &Sum::Right(ptr) = val.as_ref() {
        mem.dealloc(val);

        return ptr.get(mem);
    } else {
        Err(RuntimeError::new(ErrorKind::TypeError))
    }
}

pub fn uniti<T: AllocObject<ITypeId>>(
    mem: &'guard MutatorView,
    val: RawPtr<T>
) -> Result<Product<T, Unit>, RuntimeError> {
    Product {
        fst: CellPtr::new_with(ScopedPtr::new(mem, val))?,
        snd: CellPtr::new_unit(),
    }
}

pub fn unite<T: AllocObject<ITypeId>>(
    mem: &'guard MutatorView,
    val: RawPtr<Product<T, Unit>>
) -> Result<ScopedPtr<'guard, T>, RuntimeError> {
    let &Product { first, second } = val.as_ref();
    mem.dealloc(val);

    return ptr.get(mem);
}

pub fn swapp<F: AllocObject<ITypeId>, S: AllocObject<ITypeId>>(
    val: &RawPtr<Product<F, S>>
) {
    if let Some(prod_ref) = val.as_mut_ref() {
        let first = prod_ref.fst;
        let second = prod_ref.snd;

        prod_ref = Product::<S, F>::{ fst: second, snd: first };
    }
}

pub fn swaps<L: AllocObject<ITypeId>, R: AllocObject<ITypeId>>(
    val: &RawPtr<Sum<L, R>>
) {
    if let Some(sum_ref) = val.as_mut_ref() {
        match sum_ref {
            Sum::Left(val) => {
                sum_ref = Sum::<R, L>::Right(val);
            },
            Sum::Right(val) => {
                sum_ref = Sum::<R, L>::Left(val);
            },
        }
    }
}

pub fn assrp<
    F: AllocObject<ITypeId>,
    S: AllocObject<ITypeId>, 
    T: AllocObject<ITypeId>>
(
    mem: &'guard MutatorView,
    val: &RawPtr<Product<F, Product<S, T>>>
) {
    if let Some(prod_ref) = val.as_mut_ref() {
        if let Some(inner_ref) = val.snd.get(mem).as_untyped().as_mut_ref() {
            let first = prod_ref.fst;
            let second = inner_ref.fst;
            let third = inner_ref.snd;

            inner_ref = Product::<F, S>::{ fst: first, snd: second };
            prod_ref = Product::<Product<F, S>, T>::{
                fst: val.snd,
                snd: third
            };
        }
    }
}

pub fn asslp<
    F: AllocObject<ITypeId>,
    S: AllocObject<ITypeId>, 
    T: AllocObject<ITypeId>>
(
    mem: &'guard MutatorView,
    val: &RawPtr<Product<Product<F, S>, T>>
) {
    if let Some(prod_ref) = val.as_mut_ref() {
        if let Some(inner_ref) = val.fst.get(mem).as_untyped().as_mut_ref() {
            let first = inner_ref.fst;
            let second = inner_ref.snd;
            let third = prod_ref.snd;

            inner_ref = Product::<S, T>::{ fst: second, snd: third };
            prod_ref = Product::<F, Product<S, T>>::{
                fst: first,
                snd: val.fst
            };
        }
    }
}
