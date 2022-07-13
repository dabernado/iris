use std::ptr::{NonNull, write};

use crate::alloc::constants;
use crate::alloc::api::AllocError;
use crate::alloc::BlockError;

pub type BlockPtr = NonNull<u8>;
pub type BlockSize = usize;

pub struct Block {
    ptr: BlockPtr,
    size: BlockSize,
}

impl Block {
    pub fn new(size: BlockSize) -> Result<Block, BlockError> {
        if !size.is_power_of_two() {
            return Err(BlockError::BadRequest);
        }

        Ok(Block {
            ptr: internal::alloc_block(size)?,
            size,
        })
    }

    pub fn into_mut_ptr(self) -> BlockPtr { self.ptr }
    pub fn size(&self) -> BlockSize { self.size }
    
    pub unsafe fn from_raw_parts(ptr: BlockPtr, size: BlockSize) -> Block {
        Block { ptr, size }
    }

    pub fn as_ptr(&self) -> *const u8 {
        self.ptr.as_ptr()
    }
}

impl Drop for Block {
    fn drop(&mut self) {
        internal::dealloc_block(self.ptr, self.size);
    }
}

// bumps downwards
pub struct BumpBlock {
    cursor: usize,
    limit: usize,
    block: Block,
    meta: Box<BlockMeta>,
}

impl BumpBlock {
    pub fn new() -> Result<BumpBlock, AllocError> {
        let mut block = BumpBlock {
            cursor: constants::BLOCK_SIZE,
            limit: constants::FIRST_OBJECT_OFFSET,
            block: Block::new(constants::BLOCK_SIZE)?,
            meta: BlockMeta::new_boxed(),
        };

        let meta_ptr: *const BlockMeta = &*block.meta;
        unsafe { block.write(meta_ptr, 0) };

        Ok(block)
    }

    unsafe fn write<T>(&mut self, object: T, offset: usize) -> *const T {
        let p = self.block.as_ptr().add(offset) as *mut T;
        write(p, object);
        p
    }

    pub fn inner_alloc(&mut self, alloc_size: usize) -> Option<*const u8> {
        let next_bump = self.cursor - alloc_size;

        if next_bump < self.limit {
            if self.limit > constants::FIRST_OBJECT_OFFSET {
                if let Some((cursor, limit)) = self.meta.find_next_available_hole(self.limit) {
                    self.cursor = cursor;
                    self.limit = limit;
                    if !(self.cursor == constants::FIRST_OBJECT_OFFSET) {
                        return self.inner_alloc(alloc_size);
                    }
                }
            }

            None
        } else {
            //let offset = self.cursor;
            self.cursor = next_bump;
            unsafe {
                Some(self.block.as_ptr().add(next_bump) as *const u8)
            }
        }
    }

    pub fn current_hole_size(&self) -> usize { self.cursor - self.limit }
}

pub struct BlockMeta {
    line_mark: [bool; constants::LINE_COUNT],
    //block_mark: bool,                         /* deallocation is automatic */
}

impl BlockMeta {
    pub fn new_boxed() -> Box<BlockMeta> {
        Box::new(BlockMeta {
            line_mark: [false; constants::LINE_COUNT],
            //block_mark: false,
        })
    }

    pub fn mark_line(&mut self, index: usize) {
        self.line_mark[index] = true;
    }

    /*
     * pub fn mark_block(&mut self) {
     *     self.block_mark = true;
     * }
     */

    pub fn reset(&mut self) {
        for bit in self.line_mark.iter_mut() {
            *bit = false
        }

        //self.block_mark = false;
    }

    pub fn line_iter(&self) -> impl Iterator<Item = &'_ bool> {
        self.line_mark.iter()
    }

    pub fn find_next_available_hole(&self, starting_at: usize) -> Option<(usize, usize)> {
        let mut count = 0;
        let mut start: Option<usize> = None;
        let mut stop: usize = 0;
        let starting_line = starting_at / constants::LINE_SIZE;

        for (index, marked) in self.line_mark[(constants::LINE_COUNT - starting_line)..]
            .iter().enumerate()
        {
            let abs_index = starting_line - index;

            // count unmarked lines
            if !*marked {
                count += 1;

                // if first line in hole (and not zeroth), skip to next line
                if count == 1 && abs_index < constants::LINE_COUNT {
                    continue;
                }

                // record first hole index
                if start.is_none() {
                    start = Some(abs_index);
                }

                stop = abs_index - 1;
            }

            // if reached marked line or end of block, check for valid hole
            if count > 0 && (*marked || stop <= constants::BLOCK_START) {
                if let Some(start) = start {
                    let cursor = start * constants::LINE_SIZE;
                    let limit = if stop == 0 {
                        constants::FIRST_OBJECT_OFFSET
                    } else {
                        stop * constants::LINE_SIZE
                    };

                    return Some((cursor, limit));
                }
            }

            // if line marked and no cursor/limit returned, reset hole state
            if *marked {
                count = 0;
                start = None;
            }
        }

        None
    }
}

mod internal {
    use super::{BlockError, BlockPtr, BlockSize};
    use std::alloc::{alloc, dealloc, Layout};
    use std::ptr::NonNull;

    pub fn alloc_block(size: BlockSize) -> Result<BlockPtr, BlockError> {
        unsafe {
            let layout = Layout::from_size_align_unchecked(size, size);

            let ptr = alloc(layout);
            if ptr.is_null() {
                Err(BlockError::OutOfMemory)
            } else {
                Ok(NonNull::new_unchecked(ptr))
            }
        }
    }

    pub fn dealloc_block(ptr: BlockPtr, size: BlockSize) {
        unsafe {
            let layout = Layout::from_size_align_unchecked(size, size);

            dealloc(ptr.as_ptr(), layout);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn alloc_dealloc(size: BlockSize) -> Result<(), BlockError> {
        let block = Block::new(size)?;

        // block address and alignment bits (size-1) should be mutually exclusive
        let mask = size - 1;
        assert!((block.ptr.as_ptr() as usize & mask) ^ mask == mask);

        drop(block);
        Ok(())
    }

    /*
     * Block tests
     */
    #[test]
    fn test_bad_sizealign() {
        assert!(alloc_dealloc(999) == Err(BlockError::BadRequest))
    }

    #[test]
    fn test_4k() {
        assert!(alloc_dealloc(4096).is_ok())
    }

    #[test]
    fn test_32k() {
        assert!(alloc_dealloc(32768).is_ok())
    }

    #[test]
    fn test_16m() {
        assert!(alloc_dealloc(16*1024*1024).is_ok())
    }

    /*
     * BlockMeta tests
     */
    #[test]
    fn test_find_next_hole() {
        let mut meta = BlockMeta::new_boxed();

        meta.mark_line(0);
        meta.mark_line(1);
        meta.mark_line(2);
        meta.mark_line(4);
        meta.mark_line(10);

        let expect = Some((
                (constants::LINE_COUNT - 6) * constants::LINE_SIZE,
                (constants::LINE_COUNT - 10) * constants::LINE_SIZE
        ));
        let got = meta.find_next_available_hole(constants::BLOCK_SIZE);

        println!("test_find_next_hole got {:?}, expected {:?}", got, expect);
        assert!(got == expect)
    }

    #[test]
    fn test_find_next_hole_at_first_line() {
        let mut meta = BlockMeta::new_boxed();

        meta.mark_line(3);
        meta.mark_line(4);
        meta.mark_line(5);

        let expect = Some((
                constants::LINE_COUNT * constants::LINE_SIZE,
                (constants::LINE_COUNT - 3) * constants::LINE_SIZE
        ));
        let got = meta.find_next_available_hole(constants::BLOCK_SIZE);

        println!("test_find_next_hole_at_first_line got {:?}, expected {:?}", got, expect);
        assert!(got == expect)
    }

    #[test]
    fn test_find_next_hole_at_block_end() {
        let mut meta = BlockMeta::new_boxed();
        let halfway = constants::LINE_COUNT / 2;

        for i in 0..halfway {
            meta.mark_line(i);
        }

        let expect = Some(((halfway - 1) * constants::LINE_SIZE, constants::FIRST_OBJECT_OFFSET));
        let got = meta.find_next_available_hole(constants::BLOCK_SIZE);

        println!("test_find_next_hole_at_block_end got {:?}, expected {:?}", got, expect);
        assert!(got == expect)
    }

    #[test]
    fn test_find_hole_all_conservatively_marked() {
        let mut meta = BlockMeta::new_boxed();

        for i in 0..constants::LINE_COUNT {
            if i % 2 == 0 {
                meta.mark_line(i);
            }
        }

        let got = meta.find_next_available_hole(constants::BLOCK_SIZE);
        
        println!("test_find_next_hole_all_conservatively_marked got {:?}, expected None", got);
        assert!(got == None);
    }

    #[test]
    fn test_find_entire_block() {
        let meta = BlockMeta::new_boxed();

        let expect = Some((constants::BLOCK_SIZE, constants::FIRST_OBJECT_OFFSET));
        let got = meta.find_next_available_hole(constants::BLOCK_SIZE);

        println!("test_find_entire_block got {:?}, expected {:?}", got, expect);
        assert!(got == expect);
    }

    /*
     * BumpBlock tests
     */
    const TEST_UNIT_SIZE: usize = 8;

    // helper function: fill all holes with u32 values and return number of values allocated
    // and assert values stay unchanged as allocation continues
    fn loop_check_allocated(block: &mut BumpBlock) -> usize {
        let mut v = Vec::new();
        let mut index = 0;

        loop {
            //println!("cursor={}, limit={}", block.cursor, block.limit);
            if let Some(ptr) = block.inner_alloc(TEST_UNIT_SIZE) {
                let u32ptr = ptr as *mut u32;
                assert!(!v.contains(&u32ptr));

                v.push(u32ptr);
                unsafe { *u32ptr = index }
                index += 1;
            } else {
                break;
            }
        }

        for (index, u32ptr) in v.iter().enumerate() {
            unsafe { assert!(**u32ptr == index as u32); }
        }

        index as usize
    }

    #[test]
    fn test_empty_block() {
        let mut block = BumpBlock::new().unwrap();

        let count = loop_check_allocated(&mut block);
        let expect = (constants::BLOCK_SIZE - constants::FIRST_OBJECT_OFFSET) / TEST_UNIT_SIZE;

        println!("expect={}, count={}", expect, count);
        assert!(count == expect);
    }

    #[test]
    fn test_half_block() {
        let mut block = BumpBlock::new().unwrap();

        for i in 0..(constants::LINE_COUNT / 2) {
            block.meta.mark_line(i);
        }

        block.limit = block.cursor;     // block is recycled

        let count = loop_check_allocated(&mut block);
        let expect = ((((constants::LINE_COUNT / 2) - 1)
            * constants::LINE_SIZE)
            - constants::FIRST_OBJECT_OFFSET)
            / TEST_UNIT_SIZE;

        println!("expect={}, count={}", expect, count);
        assert!(count == expect);
    }

    #[test]
    fn test_conservatively_marked_block() {
        let mut block = BumpBlock::new().unwrap();

        for i in 0..constants::LINE_COUNT {
            if i % 2 == 0 {
                block.meta.mark_line(i);
            }
        }

        block.limit = block.cursor;     // block is recycled

        let count = loop_check_allocated(&mut block);

        println!("count={}", count);
        assert!(count == 0);
    }
}
