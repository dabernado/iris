use crate::alloc::api::{AllocObject, RawPtr};
use crate::data::*;
use crate::error::{RuntimeError, ErrorKind};
use crate::memory::MutatorView;
use crate::safeptr::{ScopedPtr, CellPtr};

pub fn zeroi<'guard, T: AllocObject<ITypeId>>(
    mem: &'guard MutatorView,
    val: RawPtr<T>
) -> Result<Sum<Zero, T>, RuntimeError> {
    Sum::Right(CellPtr::new_with(ScopedPtr::new(mem, val))?)
}


pub fn zeroe<'guard, T: AllocObject<ITypeId>>(
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

pub fn uniti<'guard, T: AllocObject<ITypeId>>(
    mem: &'guard MutatorView,
    val: RawPtr<T>
) -> Result<Product<T, Unit>, RuntimeError> {
    Product {
        fst: CellPtr::new_with(ScopedPtr::new(mem, val))?,
        snd: CellPtr::new_unit(),
    }
}

pub fn unite<'guard, T: AllocObject<ITypeId>>(
    mem: &'guard MutatorView,
    val: RawPtr<Product<T, Unit>>
) -> Result<ScopedPtr<'guard, T>, RuntimeError> {
    let &Product { first, second } = val.as_ref();
    if mem.get_header(second).type_id() == ITypeId::Unit {
        mem.dealloc(val);
        return first.get(mem);
    } else {
        Err(RuntimeError::new(ErrorKind::TypeError))
    }
}

pub fn swapp<'guard, F: AllocObject<ITypeId>, S: AllocObject<ITypeId>>(
    val: &RawPtr<Product<F, S>>
) {
    if let Some(prod_ref) = val.as_mut_ref() {
        let first = prod_ref.fst;
        let second = prod_ref.snd;

        prod_ref = Product::<S, F> { fst: second, snd: first };
    }
}

pub fn swaps<'guard, L: AllocObject<ITypeId>, R: AllocObject<ITypeId>>(
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

pub fn assrp<'guard, 
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

            inner_ref = Product::<F, S> { fst: first, snd: second };
            prod_ref = Product::<Product<F, S>, T> {
                fst: val.snd,
                snd: third
            };
        }
    }
}

pub fn asslp<'guard, 
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

            inner_ref = Product::<S, T> { fst: second, snd: third };
            prod_ref = Product::<F, Product<S, T>> {
                fst: first,
                snd: val.fst
            };
        }
    }
}

pub fn assrs<'guard, 
    F: AllocObject<ITypeId>,
    S: AllocObject<ITypeId>, 
    T: AllocObject<ITypeId>>
(
    mem: &'guard MutatorView,
    old_val: &mut RawPtr<Sum<F, Sum<S, T>>>
) -> Result<(), RuntimeError> {
    if let Some(sum_ref) = old_val.as_mut_ref() {
        match sum_ref {
            Sum::Left(val) => {
                let inner_val = val.get(mem);
                let new_val = mem.alloc(
                    Sum::<Sum<F, S>, T>::Left(
                        Sum::<F, S>::Left(CellPtr::new_with(inner_val)))
                )?;

                mem.dealloc(old_val)?;
                old_val = new_val;
            },
            Sum::Right(val) => {
                match val.get(mem).as_untyped().as_ref() {
                    Sum::Left(inner_val) => {
                        let new_val = mem.alloc(
                            Sum::<Sum<F, S>, T>::Left(
                                CellPtr::new_with(mem.alloc(
                                        Sum::<F, S>::Right(
                                            CellPtr::new_with(inner_val)
                                        )
                                ))?
                            )
                        )?;

                        mem.dealloc(old_val)?;
                        old_val = new_val;
                    },
                    Sum::Right(inner_val) => {
                        let new_val = mem.alloc(
                            Sum::<Sum<F, S>, T>::Right(
                                CellPtr::new_with(inner_val)
                            )
                        )?;

                        mem.dealloc(old_val)?;
                        old_val = new_val;
                    },
                }
            }
        }
    }
}

pub fn assls<'guard, 
    F: AllocObject<ITypeId>,
    S: AllocObject<ITypeId>, 
    T: AllocObject<ITypeId>>
(
    mem: &'guard MutatorView,
    old_val: &mut RawPtr<Sum<Sum<F, S>, T>>
) -> Result<(), RuntimeError> {
    if let Some(sum_ref) = old_val.as_mut_ref() {
        match sum_ref {
            Sum::Right(val) => {
                let inner_val = val.get(mem);
                let new_val = mem.alloc(
                    Sum::<F, Sum<S, T>>::Right(
                        Sum::<S, T>::Right(CellPtr::new_with(inner_val)))
                )?;

                mem.dealloc(old_val)?;
                old_val = new_val;
            },
            Sum::Left(val) => {
                match val.get(mem).as_untyped().as_ref() {
                    Sum::Right(inner_val) => {
                        let new_val = mem.alloc(
                            Sum::<F, Sum<S, T>>::Right(
                                CellPtr::new_with(mem.alloc(
                                        Sum::<S, T>::Left(
                                            CellPtr::new_with(inner_val)
                                        )
                                ))?
                            )
                        )?;

                        mem.dealloc(old_val)?;
                        old_val = new_val;
                    },
                    Sum::Left(inner_val) => {
                        let new_val = mem.alloc(
                            Sum::<F, Sum<S, T>>::Left(
                                CellPtr::new_with(inner_val)
                            )
                        )?;

                        mem.dealloc(old_val)?;
                        old_val = new_val;
                    },
                }
            }
        }
    }
}
