use std::mem::size_of;
use std::ptr::NonNull;

use crate::alloc::BlockError;
use crate::alloc::constants;

pub trait AllocObject {}

pub trait AllocRaw {
    fn alloc<T>(&self, object: T) -> Result<RawPtr<T>, AllocError>
        where T: AllocObject;
    fn alloc_array(&self, size_bytes: ArraySize)
        -> Result<RawPtr<u8>, AllocError>;

    fn dealloc<T>(&self, object: RawPtr<T>) -> Result<(), AllocError>
        where T: AllocObject;
    fn dealloc_with_size<T>(&self, object: RawPtr<T>, size: usize)
        -> Result<(), AllocError>
        where T: AllocObject;
    fn dealloc_array(&self, object: RawPtr<u8>, size_bytes: ArraySize)
        -> Result<(), AllocError>;
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum AllocError {
    BadRequest,
    OutOfMemory,
}

impl From<BlockError> for AllocError {
    fn from(error: BlockError) -> AllocError {
        match error {
            BlockError::BadRequest => AllocError::BadRequest,
            BlockError::OutOfMemory => AllocError::OutOfMemory,
        }
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SizeClass {
    Small,
    Medium,
    Large,
}

impl SizeClass {
    pub fn get_for_size(object_size: usize) -> Result<SizeClass, AllocError> {
        match object_size {
            constants::SMALL_OBJECT_MIN..=constants::SMALL_OBJECT_MAX => Ok(SizeClass::Small),
            constants::MEDIUM_OBJECT_MIN..=constants::MEDIUM_OBJECT_MAX => Ok(SizeClass::Medium),
            constants::LARGE_OBJECT_MIN..=constants::LARGE_OBJECT_MAX => Ok(SizeClass::Large),
            _ => Err(AllocError::BadRequest),
        }
    }
}

type ArraySize = u32;

pub fn alloc_size_of(object_size: usize) -> usize {
    let align = size_of::<usize>();
    (object_size + (align - 1)) & !(align - 1)
}

/*
 * RawPtr API
 */
#[derive(Debug)]
pub struct RawPtr<T: Sized> {
    ptr: NonNull<T>
}

pub type UntypedPtr = RawPtr<()>;

impl<T: Sized> RawPtr<T> {
    pub fn new(ptr: *const T) -> RawPtr<T> {
        RawPtr {
            ptr: unsafe { NonNull::new_unchecked(ptr as *mut T) },
        }
    }

    pub unsafe fn cast<U>(self) -> RawPtr<U> {
        RawPtr::new(self.ptr.cast::<U>().as_ptr())
    }

    pub fn from_usize(nat: usize) -> UntypedPtr {
        RawPtr {
            ptr: unsafe { NonNull::new_unchecked(nat as *mut ()) },
        }
    }

    pub fn as_ptr(self) -> *const T { self.ptr.as_ptr() }
    pub fn as_word(self) -> usize { self.ptr.as_ptr() as usize }
    pub fn as_untyped(self) -> UntypedPtr { unsafe { self.cast() } }
    pub fn as_ref(&self) -> &T { unsafe { self.ptr.as_ref() } }
    pub fn as_mut(&mut self) -> &mut T { unsafe { self.ptr.as_mut() } }
}

impl<T: Sized> AllocObject for RawPtr<T> {}
impl<T: Sized> Clone for RawPtr<T> {
    fn clone(&self) -> RawPtr<T> { RawPtr { ptr: self.ptr } }
}

impl<T: Sized> Copy for RawPtr<T> {}

impl<T: Sized> PartialEq for RawPtr<T> {
    fn eq(&self, other: &RawPtr<T>) -> bool {
        let RawPtr { ptr: other_ptr } = other;
        self.ptr == *other_ptr
    }
}
