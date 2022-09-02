use std::cell::UnsafeCell;
use std::mem::{replace, size_of};
use std::ptr::write;
use std::slice::from_raw_parts_mut;

use crate::array::ArraySize;
use crate::alloc::constants;
use crate::alloc::blocks::BumpBlock;
use crate::alloc::api::*;

pub struct StickyImmixHeap {
    blocks: UnsafeCell<BlockList>,
}

impl StickyImmixHeap {
    pub fn new() -> StickyImmixHeap {
        StickyImmixHeap {
            blocks: UnsafeCell::new(BlockList::new()),
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

    fn get_block(&self, word: usize) -> Option<&mut BumpBlock> {
        if let Some(head) = unsafe {
            self.blocks.get().as_mut().unwrap().head()
        } {
            let block_start = head.as_ptr() as usize;
            let block_end = block_start + constants::BLOCK_SIZE;
            println!("{} - {}", block_start, block_end);

            if (word >= block_start) && (word < block_end) {
                return Some(head);
            }
        }

        if let Some(overflow) = unsafe {
            self.blocks.get().as_mut().unwrap().overflow()
        } {
            let block_start = overflow.as_ptr() as usize;
            let block_end = block_start + constants::BLOCK_SIZE;

            if (word >= block_start) && (word < block_end) {
                return Some(overflow);
            }
        }

        for block in unsafe {
            self.blocks.get().as_mut().unwrap().rest()
        } {
            let block_start = block.as_ptr() as usize;
            let block_end = block_start + constants::BLOCK_SIZE;

            if (word >= block_start) && (word < block_end) {
                return Some(block);
            }
        }

        None
    }
}

impl AllocRaw for StickyImmixHeap {
    fn alloc<T>(&self, object: T) -> Result<RawPtr<T>, AllocError>
        where T: AllocObject,
    {
        let total_size = size_of::<T>();

        // round size to next word boundary for alignment
        let alloc_size = alloc_size_of(total_size);
        let size_class = SizeClass::get_for_size(alloc_size)?;

        let space = self.find_space(alloc_size, size_class)?;

        // write object into space next to header
        unsafe { write(space as *mut T, object); }

        Ok(RawPtr::new(space as *const T))
    }

    fn dealloc<T>(&self, object: RawPtr<T>) -> Result<(), AllocError>
        where T: AllocObject,
    {
        let object_size = size_of::<T>();
        let alloc_size = alloc_size_of(object_size);

        self.dealloc_with_size(object, alloc_size)
    }

    fn dealloc_with_size<T>(&self, object: RawPtr<T>, size: usize)
        -> Result<(), AllocError>
        where T: AllocObject,
    {
        // mark block lines as unallocated
        let obj_ptr = object.as_ptr();
        let block = self.get_block(object.as_word()).unwrap();

        let cursor = obj_ptr as usize - block.as_ptr() as usize;
        block.inner_dealloc(cursor, size);

        Ok(())
    }

    fn alloc_array(&self, size_bytes: ArraySize) -> Result<RawPtr<u8>, AllocError> {
        let total_size = size_bytes as usize;

        // round size to next word boundary for alignment
        let alloc_size = alloc_size_of(total_size);
        let size_class = SizeClass::get_for_size(alloc_size)?;

        let space = self.find_space(alloc_size, size_class)?;

        // get space for array
        let array = unsafe { from_raw_parts_mut(space as *mut u8, size_bytes as usize) };
        // initialize array values to 0
        for byte in array {
            *byte = 0;
        }

        Ok(RawPtr::new(space as *const u8))
    }

    fn dealloc_array(&self, array: RawPtr<u8>, array_size: ArraySize)
        -> Result<(), AllocError>
    {
        let total_size = array_size as usize;

        // round size to next word boundary for alignment
        let alloc_size = alloc_size_of(total_size);

        // mark block lines as unallocated
        let array_ptr = array.as_ptr();
        let block = self.get_block(array.as_word()).unwrap();

        let cursor = array_ptr as usize - block.as_ptr() as usize;
        block.inner_dealloc(cursor, alloc_size);

        Ok(())
    }
}

impl Default for StickyImmixHeap {
    fn default() -> StickyImmixHeap {
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

    fn head(&mut self) -> Option<&mut BumpBlock> { self.head.as_mut() }
    fn overflow(&mut self) -> Option<&mut BumpBlock> { self.overflow.as_mut() }
    fn rest(&mut self) -> &mut Vec<BumpBlock> { self.rest.as_mut() }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::alloc::api::AllocObject;
    use std::slice::from_raw_parts;

    #[test]
    fn test_alloc() {
        let mem = StickyImmixHeap::new();

        match mem.alloc(69 as i32) {
            Ok(i) => {
                let orig = i.as_ref();
                assert!(*orig == 69);
            },
            Err(_) => panic!("Allocation failed"),
        }
    }

    #[test]
    fn test_dealloc() {
        let mem = StickyImmixHeap::new();

        match mem.alloc(69 as i32) {
            Ok(ptr) => {
                let total_size = size_of::<i32>();

                let orig = ptr.as_ptr();
                let block = mem.get_block(ptr.as_word()).unwrap();
                let cursor = orig as usize - block.as_ptr() as usize;

                // deallocate object
                mem.dealloc(ptr);

                // assert that lines are unmarked
                let lines = block.get_lines(cursor, total_size);
                for line in lines {
                    assert!(line == false);
                }
            },
            Err(_) => panic!("Allocation failed"),
        }
    }

    #[test]
    fn test_realloc() {
        let mem = StickyImmixHeap::new();

        match mem.alloc(69 as i32) {
            Ok(ptr) => {
                assert!(*ptr.as_ref() == 69);
                
                let total_size = size_of::<i32>();
                let alloc_size = alloc_size_of(total_size);

                let orig = ptr.as_ptr();
                let block = mem.get_block(ptr.as_word()).unwrap();
                let cursor = orig as usize - block.as_ptr() as usize;

                // assert that lines are marked
                let alloc_lines = block.get_lines(cursor, alloc_size);
                for line in alloc_lines {
                    assert!(line == true);
                }

                // deallocate object
                mem.dealloc(ptr);

                // assert that lines are unmarked
                let dealloc_lines = block.get_lines(cursor, alloc_size);
                for line in dealloc_lines {
                    assert!(line == false);
                }

                // reallocate object
                match mem.alloc(420 as i32) {
                    Ok(new_ptr) => {
                        let new_orig = new_ptr.as_ptr();
                        let new_block = mem.get_block(new_ptr.as_word())
                            .unwrap();
                        let new_cursor = new_orig as usize - new_block.as_ptr() as usize;

                        assert!(block.as_ptr() == new_block.as_ptr());
                        assert!(*new_ptr.as_ref() == 420);
                        assert!(cursor == new_cursor);

                        let realloc_lines = block.get_lines(new_cursor, alloc_size);
                        for line in realloc_lines {
                            assert!(line == true);
                        }
                    },
                    Err(_) => panic!("Allocation failed"),
                }
            },
            Err(_) => panic!("Allocation failed"),
        }
    }

    #[test]
    fn test_many_obs_alloc() {
        let mem = StickyImmixHeap::new();
        let mut obs = Vec::new();

        for i in 0..(constants::BLOCK_SIZE * 3) {
            match mem.alloc(i as i32) {
                Ok(ptr) => obs.push(ptr),
                Err(_) => assert!(false, "Allocation failed unexpectedly"),
            }
        }
        println!("Finished allocating");

        for (i, ob) in obs.iter().enumerate() {
            println!("{} {}", i, ob.as_ref());
            assert!(i as i32 == *ob.as_ref())
        }
    }

    #[test]
    fn test_many_obs_dealloc() {
        let mem = StickyImmixHeap::new();
        let mut obs = Vec::new();

        for i in 0..(constants::BLOCK_SIZE * 3) {
            match mem.alloc(i as i32) {
                Ok(ptr) => obs.push(ptr),
                Err(_) => assert!(false, "Allocation failed unexpectedly"),
            }
        }
        println!("Finished allocating");

        for (i, ob) in obs.iter().enumerate() {
            let total_size = size_of::<i32>();

            let orig = ob.as_ptr();
            let block = mem.get_block(ob.as_word()).unwrap();
            let cursor = orig as usize - block.as_ptr() as usize;

            // deallocate object
            mem.dealloc(*ob);

            // assert that lines are unmarked
            let lines = block.get_lines(cursor, total_size);
            for line in lines {
                assert!(line == false);
            }
        }
    }

    #[test]
    fn test_array_alloc() {
        let mem = StickyImmixHeap::new();
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
        let mem = StickyImmixHeap::new();
        let size = 2048;

        match mem.alloc_array(size) {
            Ok(ptr) => {
                let orig = ptr.as_ptr();
                let block = mem.get_block(ptr.as_word()).unwrap();
                let cursor = orig as usize - block.as_ptr() as usize;

                // deallocate object
                mem.dealloc_array(ptr, size);

                // assert that lines are unmarked
                let lines = block.get_lines(cursor, size as usize);
                for line in lines {
                    assert!(line == false);
                }
            },
            Err(_) => assert!(false, "Allocation failed unexpectedly"),
        }
    }

    // Testing large allocations
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

    impl AllocObject for Big {}

    #[test]
    fn test_too_big() {
        let mem = StickyImmixHeap::new();
        assert!(mem.alloc(Big::make()) == Err(AllocError::BadRequest));
    }
}
