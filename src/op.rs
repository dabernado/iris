use crate::alloc::api::{AllocObject, RawPtr, UntypedPtr};
use crate::data::*;
use crate::error::{RuntimeError, ErrorKind};
use crate::safeptr::CellPtr;

pub fn zeroi<T>(val: RawPtr<T>) -> Sum<Zero, T>
    where T: AllocObject
{
    Sum::Right(CellPtr::new_with(val))
}

pub fn zeroe<T>(val: &RawPtr<Sum<Zero, T>>) -> Result<CellPtr<T>, RuntimeError>
    where T: AllocObject
{
    match val {
        Sum::Right(ptr) => ptr,
        _ => Err(RuntimeError::new(ErrorKind::TypeError))
    }
}
