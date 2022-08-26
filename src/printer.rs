use std::fmt;

use crate::alloc::api::AllocObject;
use crate::memory::MutatorScope;
use crate::safeptr::ScopedPtr;

pub trait Print {
    fn print<'guard>(
        &self,
        _guard: &'guard dyn MutatorScope,
        f: &mut fmt::Formatter,
    ) -> fmt::Result;
    
    fn debug<'guard>(
        &self,
        _guard: &'guard dyn MutatorScope,
        f: &mut fmt::Formatter,
    ) -> fmt::Result { self.print(_guard, f) }
}

pub fn print<T: AllocObject + Print>(
    value: ScopedPtr<'_, T>
) -> String {
    format!("{}", value)
}

pub fn debug<T: AllocObject + Print>(
    value: ScopedPtr<'_, T>
) -> String {
    format!("{:?}", value)
}
