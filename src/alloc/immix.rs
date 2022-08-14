use std::cell::UnsafeCell;
use std::marker::PhantomData;
use std::mem::{replace, size_of};
use std::ptr::{write, NonNull};
use std::slice::from_raw_parts_mut;

use crate::array::ArraySize;
use crate::alloc::constants;
use crate::alloc::blocks::BumpBlock;
use crate::alloc::api::*;
use crate::data::ITypeHeader;

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

        let result = match blocks.head {
            Some(ref mut head) => {
                if size_class == SizeClass::Medium && alloc_size > head.current_hole_size() {
                    return blocks.overflow_alloc(alloc_size);
                }

                match head.inner_alloc(alloc_size) {
                    Some(space) => space,
                    None => {
                        let previous = replace(head, BumpBlock::new()?);

                        blocks.rest.push(previous);
                        let space = head.inner_alloc(alloc_size)
                            .expect("New bump block unable to allocate");

                        space
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

        Ok(result)
    }

    fn get_block(&self, word: usize) -> Option<&'static mut BumpBlock> {
        let blocks = self.blocks.get().as_ref().unwrap();

        if let Some(head) = blocks.head() {
            let block_start = head.as_ptr() as usize;
            let block_end = block_start + constants::BLOCK_SIZE;

            match word {
                block_start..block_end => return Some(&mut head),
                _ => {},
            }
        } else if let Some(overflow) = blocks.overflow() {
            let block_start = overflow.as_ptr() as usize;
            let block_end = block_start + constants::BLOCK_SIZE;

            match word {
                block_start..block_end => return Some(&mut overflow),
                _ => {},
            }
        } else {
            for block in blocks.rest() {
                let block_start = block.as_ptr() as usize;
                let block_end = block_start + constants::BLOCK_SIZE;

                match word {
                    block_start..block_end => return Some(&mut block),
                    _ => continue,
                }
            }
        }

        None
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
        let header = Self::Header::new::<T>(
            object_size as u32,
            size_class,
        );

        // write header into front of allocated space
        unsafe { write(space as *mut Self::Header, header); }

        // write object into space next to header
        let object_space = unsafe { space.offset(header_size as isize) };
        unsafe { write(object_space as *mut T, object); }

        Ok(RawPtr::new(object_space as *const T))
    }

    fn dealloc<T>(&self, object: RawPtr<T>) -> Result<(), AllocError>
        where T: AllocObject<<Self::Header as AllocHeader>::TypeId>,
    {
        let header_size = size_of::<Self::Header>();
        let object_size = size_of::<T>();
        let total_size = header_size + object_size;

        // mark block lines as unallocated
        let obj_ptr = object.as_ptr();
        let block = self.get_block(object.as_word()).unwrap();

        let cursor = obj_ptr.sub(block.as_ptr() as usize) as usize;
        block.inner_dealloc(cursor, total_size);

        Ok(())
    }

    fn alloc_array(&self, size_bytes: ArraySize) -> Result<RawPtr<u8>, AllocError> {
        let header_size = size_of::<Self::Header>();
        let total_size = header_size + size_bytes as usize;

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

    fn dealloc_array(&self, array: RawPtr<u8>) -> Result<(), AllocError> {
        let header_size = size_of::<Self::Header>();
        let header = StickyImmixHeap::<ITypeHeader>::get_header(
            array.as_untyped()).as_ref();
        let array_size = header.size() as usize;
        let total_size = array_size + header_size;

        // mark block lines as unallocated
        let array_ptr = array.as_ptr();
        let block = self.get_block(array.as_word()).unwrap();

        let cursor = array_ptr.sub(block.as_ptr() as usize) as usize;
        block.inner_dealloc(cursor, total_size);

        Ok(())
    }

    // to get header, subtract header size from object pointer
    fn get_header(object: NonNull<()>) -> NonNull<Self::Header> {
        unsafe { NonNull::new_unchecked(object.cast::<Self::Header>().as_ptr().offset(-1)) }
    }

    // to get object, add header size to header pointer
    fn get_object(header: NonNull<Self::Header>) -> NonNull<()> {
        unsafe {
            NonNull::new_unchecked(header.as_ptr().offset(1).cast::<()>())
        }
    }
}

impl<H> Default for StickyImmixHeap<H> {
    fn default() -> StickyImmixHeap<H> {
        StickyImmixHeap::new()
    }
}

pub struct BlockList {
    head: Option<BumpBlock>,
    overflow: Option<BumpBlock>,
    rest: Vec<BumpBlock>,
}

impl BlockList {
    pub fn new() -> BlockList {
        BlockList {
            head: None,
            overflow: None,
            rest: Vec::new(),
        }
    }

    pub fn overflow_alloc(&mut self, alloc_size: usize)
        -> Result<*const u8, AllocError>
    {
        assert!(alloc_size <= constants::BLOCK_CAPACITY);

        let result = match self.overflow {
            Some(ref mut overflow) => {
                match overflow.inner_alloc(alloc_size) {
                    Some(space) => space,
                    None => {
                        let previous = replace(overflow, BumpBlock::new()?);

                        self.rest.push(previous);
                        let space = overflow.inner_alloc(alloc_size)
                            .expect("Object size larger than block size");
                        
                        space
                    }
                }
            },
            None => {
                let mut overflow = BumpBlock::new()?;

                let space = overflow
                    .inner_alloc(alloc_size)
                    .expect("Object size larger than block size");

                self.overflow = Some(overflow);
                space
            }
        } as *const u8;

        Ok(result)
    }

    fn head(&self) -> &Option<BumpBlock> { &self.head }
    fn overflow(&self) -> &Option<BumpBlock> { &self.overflow }
    fn rest(&self) -> &Vec<BumpBlock> { &self.rest }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::*;
    use std::slice::from_raw_parts;

    #[test]
    fn test_alloc() {
        let mem = StickyImmixHeap::<ITypeHeader>::new();

        match mem.alloc(69 as i32) {
            Ok(i) => {
                let orig = unsafe { i.as_ref() };
                assert!(*orig == 69);
            },
            Err(_) => panic!("Allocation failed"),
        }
    }

    #[test]
    fn test_dealloc() {
        let mem = StickyImmixHeap::<ITypeHeader>::new();

        match mem.alloc(69 as i32) {
            Ok(ptr) => {
            },
            Err(_) => panic!("Allocation failed"),
        }
    }

    #[test]
    fn test_many_obs_alloc() {
        let mem = StickyImmixHeap::<ITypeHeader>::new();
        let mut obs = Vec::new();

        for i in 0..(constants::BLOCK_SIZE * 3) {
            match mem.alloc(i as i32) {
                Ok(ptr) => obs.push(ptr),
                Err(_) => assert!(false, "Allocation failed unexpectedly"),
            }
        }
        println!("Finished allocating");

        for (i, ob) in obs.iter().enumerate() {
            println!("{} {}", i, unsafe { ob.as_ref() });
            assert!(i as i32 == unsafe { *ob.as_ref() })
        }
    }

    #[test]
    fn test_many_obs_dealloc() {
        let mem = StickyImmixHeap::<ITypeHeader>::new();
        let mut obs = Vec::new();

        for i in 0..(constants::BLOCK_SIZE * 3) {
            match mem.alloc(i as i32) {
                Ok(ptr) => obs.push(ptr),
                Err(_) => assert!(false, "Allocation failed unexpectedly"),
            }
        }
        println!("Finished allocating");

        for (i, ob) in obs.iter().enumerate() {
        }
    }

    #[test]
    fn test_array_alloc() {
        let mem = StickyImmixHeap::<ITypeHeader>::new();
        let size = 2048;

        match mem.alloc_array(size) {
            Ok(ptr) => {
                let ptr = ptr.as_ptr();
                let array = unsafe { from_raw_parts(ptr, size as usize) };

                for byte in array {
                    assert!(*byte == 0);
                }
            },
            Err(_) => assert!(false, "Allocation failed unexpectedly"),
        }
    }

    #[test]
    fn test_array_dealloc() {
        let mem = StickyImmixHeap::<ITypeHeader>::new();
        let size = 2048;

        match mem.alloc_array(size) {
            Ok(ptr) => {
                let ptr = ptr.as_ptr();
            },
            Err(_) => assert!(false, "Allocation failed unexpectedly"),
        }
    }

    #[test]
    fn test_header() {
        let mem = StickyImmixHeap::<ITypeHeader>::new();

        match mem.alloc(69 as i32) {
            Ok(i) => {
                let untyped_ptr = i.as_untyped();
                let header_ptr = StickyImmixHeap::<ITypeHeader>::get_header(untyped_ptr);
                dbg!(header_ptr);
                let header = unsafe { &*header_ptr.as_ptr() as &ITypeHeader };

                assert!(header.type_id() == ITypeId::Int);
            },
            Err(_) => panic!("Allocation failed"),
        }
    }

    // Testing large allocations
    struct TestHeader {
        size_class: SizeClass,
        type_id: TestTypeId,
        size_bytes: u32,
    }

    #[derive(Copy, Clone, PartialEq)]
    enum TestTypeId {
        Big,
        Array,
    }

    impl AllocTypeId for TestTypeId {}
    impl AllocHeader for TestHeader {
        type TypeId = TestTypeId;

        fn new<O: AllocObject<Self::TypeId>>(size: u32, size_class: SizeClass) -> Self {
            TestHeader {
                size_class,
                type_id: O::TYPE_ID,
                size_bytes: size,
            }
        }

        fn new_array(size: u32, size_class: SizeClass) -> Self {
            TestHeader {
                size_class,
                type_id: TestTypeId::Array,
                size_bytes: size,
            }
        }

        fn size_class(&self) -> SizeClass { SizeClass::Small }
        fn size(&self) -> u32 { 8 }
        fn type_id(&self) -> TestTypeId { self.type_id }
    }

    struct Big {
        _huge: [u8; constants::BLOCK_SIZE + 1],
    }

    impl Big {
        fn make() -> Big {
            Big {
                _huge: [0u8; constants::BLOCK_SIZE + 1],
            }
        }
    }

    impl AllocObject<TestTypeId> for Big {
        const TYPE_ID: TestTypeId = TestTypeId::Big;
    }

    #[test]
    fn test_too_big() {
        let mem = StickyImmixHeap::<TestHeader>::new();
        assert!(mem.alloc(Big::make()) == Err(AllocError::BadRequest));
    }
}
