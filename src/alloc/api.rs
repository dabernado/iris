use std::mem::size_of;
use std::ptr::NonNull;

use crate::alloc::BlockError;
use crate::alloc::constants;

pub trait AllocRaw {
    type Header: AllocHeader;

    fn alloc<T>(&self, object: T) -> Result<RawPtr<T>, AllocError>
        where T: AllocObject<<Self::Header as AllocHeader>::TypeId>;
    fn alloc_array(&self, size_bytes: ArraySize) -> Result<RawPtr<u8>, AllocError>;

    fn get_header(object: NonNull<()>) -> NonNull<Self::Header>;
    fn get_object(header: NonNull<Self::Header>) -> NonNull<()>;

    fn dealloc<T>(&self, object: RawPtr<T>) -> Result<(), AllocError>
        where T: AllocObject<<Self::Header as AllocHeader>::TypeId>;
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

pub trait AllocTypeId: Copy + Clone {}
pub trait AllocObject<T: AllocTypeId> {
    const TYPE_ID: T;
}

pub trait AllocHeader: Sized {
    type TypeId: AllocTypeId;

    fn new<O: AllocObject<Self::TypeId>>(size: u32, size_class: SizeClass) -> Self;
    fn new_array(size: ArraySize, size_class: SizeClass) -> Self;
    fn size_class(&self) -> SizeClass;
    fn size(&self) -> u32;
    fn type_id(&self) -> Self::TypeId;
    fn get_block(&self) -> &BumpBlock
}

pub fn alloc_size_of(object_size: usize) -> usize {
    let align = size_of::<usize>();
    (object_size + (align - 1)) & !(align - 1)
}

/*
 * RawPtr API
 */
pub enum RawPtr<T: Sized> {
    Unit,
    Int(i32),
    UInt(u32),
    Ptr { ptr: NonNull<T> },
}

impl<T: Sized> RawPtr<T> {
    pub fn new(ptr: *const T) -> RawPtr<T> {
        RawPtr::Ptr {
            ptr: unsafe { NonNull::new_unchecked(ptr as *mut T) },
        }
    }

    pub fn new_unit() -> RawPtr<()> { RawPtr::Unit }
    pub fn new_int(i: i32) -> RawPtr<i32> { RawPtr::Int(i) }
    pub fn new_uint(i: u32) -> RawPtr<u32> { RawPtr::UInt(i) }

    pub fn from_unit(&self, ptr: *const T) -> Result<RawPtr<T>, AllocError> {
        match self {
            RawPtr::Ptr { ptr } => Err(AllocError::BadRequest),
            RawPtr::Unit => {
                Ok(RawPtr::Ptr {
                    ptr: unsafe { NonNull::new_unchecked(ptr as *mut T) },
                })
            },
        }
    }

    pub fn as_ptr(self) -> Option<*const T> {
        if let RawPtr::Ptr { ptr } = self {
            Some(ptr.as_ptr())
        }

        None
    }

    pub fn as_word(self) -> Option<usize> {
        if let RawPtr::Ptr { ptr } = self {
            Some(ptr.as_ptr() as usize)
        }

        None
    }

    pub fn as_untyped(self) -> Option<NonNull<()>> {
        if let RawPtr::Ptr { ptr } = self {
            Some(ptr.cast())
        }

        None
    }

    pub fn as_ref(&self) -> Option<&T> {
        if let RawPtr::Ptr { ptr } = self {
            Some(ptr.as_ref())
        }

        None
    }

    pub fn as_mut_ref(&mut self) -> Option<&mut T> {
        if let RawPtr::Ptr { ptr } = self {
            Some(ptr.as_mut())
        }

        None
    }
}

impl<T: Sized> Clone for RawPtr<T> {
    fn clone(&self) -> RawPtr<T> {
        match self {
            RawPtr::Unit => RawPtr::Unit,
            RawPtr::Ptr { ptr } => RawPtr::Ptr { ptr },
        }
    }
}

impl<T: Sized> Copy for RawPtr<T> {}

impl<T: Sized> PartialEq for RawPtr<T> {
    fn eq(&self, other: &RawPtr<T>) -> bool {
        match self {
            RawPtr::Unit => other == RawPtr::Unit,
            RawPtr::Ptr { ptr } => match other {
                RawPtr::Unit => false,
                RawPtr::Ptr { other_ptr } => ptr == other_ptr,
            }
        }
    }
}
