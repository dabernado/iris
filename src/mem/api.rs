use crate::mem::blocks::BlockError;

pub trait AllocRaw {
    type Header: AllocHeader;

    fn alloc<T>(&self, object: T) -> Result<RawPtr<T>, AllocError>
        where T: AllocObject<<Self::Header as AllocHeader>::TypeId>;
    fn alloc_array(&self, size_bytes: ArraySize) -> Result<RawPtr<u8>, AllocError>;

    fn get_header(object: NonNull<()>) -> NonNull<Self::Header>;
    fn get_object(header: NonNull<Self::Header>) -> NonNull<()>;
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

pub struct RawPtr<T: Sized> {
    ptr: NonNull<T>,
}

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
}
