use crate::alloc::api::AllocObject;
use crate::data::*;
use crate::error::{RuntimeError, ErrorKind};
use crate::memory::MutatorScope;
use crate::safeptr::{CellPtr, ScopedPtr};

pub fn zeroi<'guard, T>(val: ScopedPtr<'guard, T>) -> Sum<Zero, T>
    where T: AllocObject
{
    Sum::Right(CellPtr::new_with(val))
}

pub fn zeroe<'guard, T>(
    val: ScopedPtr<'guard, Sum<Zero, T>>,
    guard: &'guard dyn MutatorScope
) -> Result<ScopedPtr<'guard, T>, RuntimeError>
    where T: AllocObject
{
    match val.as_ref(guard) {
        Sum::Right(ptr) => Ok(ptr.get(guard)),
        _ => Err(RuntimeError::new(ErrorKind::TypeError))
    }
}
