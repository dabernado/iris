use std::fmt;

use crate::alloc::api::AllocObject;
use crate::data::ITypeId;
use crate::memory::MutatorScope;

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

pub fn print<T: AllocObject<ITypeId>>(value: T) -> String {
    format!("{}", value)
}

pub fn debug<T: AllocObject<ITypeId>>(value: T) -> String {
    format!("{:?}", value)
}
