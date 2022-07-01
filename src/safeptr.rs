use std::cell::Cell;
use std::fmt;
use std::ops::Deref;
use std::ptr::NonNull;

use crate::alloc::api::RawPtr;
use crate::memory::MutatorScope;

pub type UntypedPtr = NonNull<()>;

/* Scoped Pointers */
pub struct ScopedPtr<'guard, T: Sized> {
    value: &'guard T,
}

impl<'guard, T: Sized> ScopedPtr<'guard, T> {
    pub fn new(_guard: &'guard dyn MutatorScope, value: &'guard T) -> ScopedPtr<'guard, T> {
        ScopedPtr { value }
    }
}

impl<'scope, T: Sized> MutatorScope for ScopedPtr<'scope, T> {}
impl<'guard, T: Sized> Copy for ScopedPtr<'guard, T> {}

impl<'guard, T: Sized> Clone for ScopedPtr<'guard, T> {
    fn clone(&self) -> ScopedPtr<'guard, T> {
        ScopedPtr { value: self.value }
    }
}

impl<'guard, T: Sized> Deref for ScopedPtr<'guard, T> {
    type Target = T;
    fn deref(&self) -> &T { self.value }
}

impl<'guard, T: Sized + Print> fmt::Display for ScopedPtr<'guard, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { self.value.print(self, f) }
}

impl<'guard, T: Sized + Print> fmt::Debug for ScopedPtr<'guard, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { self.value.print(self, f) }
}

impl<'guard, T: Sized + PartialEq> PartialEq for ScopedPtr<'guard, T> {
    fn eq(&self, rhs: &ScopedPtr<'guard, T>) -> bool { self.value == rhs.value }
}

/* Cell Pointers */
#[derive(Clone)]
pub struct CellPtr<T: Sized> {
    inner: Cell<RawPtr<T>>,
}

impl<T: Sized> CellPtr<T> {
    pub fn new_with(source: ScopedPtr<T>) -> CellPtr<T> {
        if source.value == () {
            CellPtr { inner: Cell::new(RawPtr::new_unit()) }
        } else {
            CellPtr { inner: Cell::new(RawPtr::new(source.value)) }
        }
    }

    pub fn get<'guard>(&self, guard: &'guard dyn MutatorScope) -> ScopedPtr<'guard, T> {
        ScopedPtr::new(guard, self.inner.get().scoped_ref(guard))
    }

    pub fn set(&self, source: ScopedPtr<T>) {
        if source.value == () {
            self.inner.set(RawPtr::new_unit());
        } else {
            self.inner.set(RawPtr::new(source.value));
        }
    }
}

impl<T: Sized> From<ScopedPtr<'_, T>> for CellPtr<T> {
    fn from(ptr: ScopedPtr<T>) -> CellPtr<T> { CellPtr::new_with(ptr) }
}

/* Scoped References */
pub trait ScopedRef<T> {
    fn scoped_ref<'scope>(&self, guard: &'scope dyn MutatorScope) -> &'scope T;
}

impl<T> ScopedRef<T> for RawPtr<T> {
    fn scoped_ref<'scope>(&self, _guard: &'scope dyn MutatorScope) -> &'scope T {
        match self.as_ptr() {
            Some(v) => unsafe { &*v },
            None => &(),
        }
    }
}