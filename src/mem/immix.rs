use std::cell::UnsafeCell;
use std::marker::PhantomData;

use crate::blocks::BlockList;

pub struct StickyImmixHeap<H> {
    blocks: UnsafeCell<BlockList>,

    _header_type: PhantomData<*const H>,
}

impl<H> StickyImmixHeap<H> {
    pub fn new() -> StickyImmixHeap<H> {
        StickyImmixHeap {
            blocks: UnsafeCell::new(BlockList::new()),
            _header_type: PhantomData,
        }
    }

    fn find_space(
        &self,
        alloc_size: usize,
        size_class: SizeClass
    ) -> Result<*const u8, AllocError> {
        let blocks = unsafe { &mut *self.blocks.get() };

        if size_class == SizeClass::Large {
            return Err(AllocError::BadRequest);
        }

        let space = match blocks.head {
            Some(ref mut head) => {
                if size_class == SizeClass::Medium && alloc_size > head.current_hole_size() {
                    return blocks.overflow_alloc(alloc_size);
                }

                match head.inner_alloc(alloc_size) {
                    Some(space) => space,
                    None => {
                        let previous = replace(head, BumpBlock::new()?);

                        blocks.rest.push(previous);
                        head.inner_alloc(alloc_size).expect("New bump block unable to allocate")
                    }
                }
            }
            None => {
                let mut head = BumpBlock::new()?;

                let space = head
                    .inner_alloc(alloc_size)
                    .expect("Object size larger than block size");

                blocks.head = Some(head);
                space
            }
        } as *const u8;

        Ok(space)
    }
}

impl<H: AllocHeader> AllocRaw for StickyImmixHeap<H> {
    type Header = H;

    fn alloc<T>(&self, object: T) -> Result<RawPtr<T>, AllocError>
        where T: AllocObject<<Self::Header as AllocHeader>::TypeId>,
    {
        let header_size = size_of::<Self::Header>();
        let object_size = size_of::<T>();
        let total_size = header_size + object_size;

        // round size to next word boundary for alignment
        let alloc_size = alloc_size_of(total_size);
        let size_class = SizeClass::get_for_size(alloc_size)?;

        let space = self.find_space(alloc_size, size_class)?;
        let header = Self::Header::new::<T>(object_size, size_class);

        // write header into front of allocated space
        unsafe { write(space as *mut Self::Header, header); }

        // write object into space next to header
        let object_space = unsafe { space.offset(header_size as isize) };
        unsafe { write(object_space as *mut T, object); }

        Ok(RawPtr::new(object_space as *const T))
    }

    fn alloc_array(&self, size_bytes: ArraySize) -> Result<RawPtr<u8>, AllocError> {
        let header_size = size_of::<Self::Header>();
        let object_size = size_of::<T>();
        let total_size = header_size + object_size;

        // round size to next word boundary for alignment
        let alloc_size = alloc_size_of(total_size);
        let size_class = SizeClass::get_for_size(alloc_size)?;

        let space = self.find_space(alloc_size, size_class)?;
        let header = Self::Header::new_array(size_bytes, size_class);

        // write header into front of allocated space
        unsafe { write(space as *mut Self::Header, header); }

        // get space for array
        let array_space = unsafe { space.offset(header_size as isize) };
        let array = unsafe { from_raw_parts_mut(array_space as *mut u8, size_bytes as usize) };
        // initialize array values to 0
        for byte in array {
            *byte = 0;
        }

        Ok(RawPtr::new(array_space as *const u8))
    }

    // to get header, subtract header size from object pointer
    fn get_header(object: NonNull<()>) -> NonNull<Self::Header> {
        unsafe { NonNull::new_unchecked(object.cast::<Self::Header>().as_ptr().offset(-1)) }
    }

    // to get object, add header size to header pointer
    fn get_header(object: NonNull<Self::Header>) -> NonNull<()> {
        unsafe { NonNull::new_unchecked(header.as_ptr().offset(1).cast::<()>()) }
    }
}