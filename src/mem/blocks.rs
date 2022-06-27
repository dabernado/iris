use std::mem::{replace, size_of};

const BLOCK_SIZE_BITS: usize = 15;
const BLOCK_SIZE: usize = 1 << BLOCK_SIZE_BITS;
const BLOCK_START: usize = 0;

const LINE_SIZE_BITS: usize = 7;
const LINE_SIZE: usize = 1 << LINE_SIZE_BITS;
const LINE_COUNT: usize = BLOCK_SIZE / LINE_SIZE;

struct BlockList {
    head: Option<BumpBlock>,
    overflow: Option<BumpBlock>,
    rest: Vec<BumpBlock>,
}

impl BlockList {
    fn new() -> BlockList {
        BlockList {
            head: None,
            overflow: None,
            rest: Vec::new(),
        }
    }

    fn overflow_alloc(&mut self, alloc_size: usize) -> Result<*const u8, AllocError> {
        match self.overflow {
            Some(ref mut overflow) => {
                match overflow.inner_alloc(alloc_size) {
                    Some(space) => space,
                    None => {
                        let previous = replace(overflow, BumpBlock::new()?);

                        self.rest.push(previous);
                        overflow.inner_alloc(alloc_size)
                            .expect("Object size larger than block size")
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
        }
    }
}

// bumps downwards
pub struct BumpBlock {
    cursor: usize,
    limit: usize,
    block: Block,
    meta: BlockMeta,
}

impl BumpBlock {
    pub fn new() -> Result<BumpBlock, AllocError> {
        let mut block = BumpBlock {
            cursor: BLOCK_SIZE,
            limit: FIRST_OBJECT_OFFSET,
            block: Block::new(BLOCK_SIZE)?,
            meta: BlockMeta::new_boxed(),
        };

        let meta_ptr: *const BlockMeta = &*block.meta;
        unsafe { block.write(meta_ptr, 0) };

        Ok(block)
    }

    pub fn inner_alloc(&mut self, alloc_size: usize) -> Option<*const u8> {
        let next_bump = self.cursor - alloc_size;

        if next_bump < self.limit {
            if self.limit > BLOCK_START {
                if let Some((cursor, limit)) = self.meta.find_next_available_hole(self.limit) {
                    self.cursor = cursor;
                    self.limit = limit;
                    return self.inner_alloc(alloc_size);
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
}

pub struct BlockMeta {
    line_mark: [bool; constants::LINE_COUNT],
    //block_mark: bool,                         /* deallocation is automatic */
}

impl BlockMeta {
    pub fn find_next_available_hole(&self, starting_at: usize) -> Option<(usize, usize)> {
        let mut count = 0;
        let mut start: Option<usize> = None;
        let mut stop: usize = 0;

        let starting_line = starting_at / constants::LINE_SIZE;
        for (index, marked) in self.line_mark[..starting_line].iter().rev().enumerate() {
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
            if count > 0 && (*marked || stop <= BLOCK_START) {
                if let Some(start) = start {
                    let cursor = start * constants::LINE_SIZE;
                    let limit = stop * constants::LINE_SIZE;

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

    pub fn as_ptr(&self) -> *const u8 {
        self.ptr.as_ptr()
    }
}

#[derive(Debug, PartialEq)]
pub enum BlockError {
    BadRequest,
    OutOfMemory,
}

pub type BlockPtr = NonNull<u8>;
pub type BlockSize = usize;

mod internal {
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

    fn dealloc_block(ptr: BlockPtr, size: BlockSize) {
        unsafe {
            let layout = Layout::from_size_align_unchecked(size, size);

            dealloc(ptr.as_ptr(), layout);
        }
    }
}
