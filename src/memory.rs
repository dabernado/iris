pub const BLOCK_SIZE_BITS: usize = 15;
pub const BLOCK_SIZE: usize = 1 << BLOCK_SIZE_BITS;
pub const LINE_SIZE_BITS: usize = 7;
pub const LINE_SIZE: usize = 1 << LINE_SIZE_BITS;
pub const LINE_COUNT: usize = BLOCK_SIZE / LINE_SIZE;

pub struct BumpBlock {
    cursor: usize,
    limit: usize,
    block: Block,
    line_mark: [bool; constants::LINE_COUNT],
}

impl BumpBlock {
    pub fn inner_alloc(&mut self, alloc_size: usize) -> Option<*const u8> {
        let next_bump = self.cursor + alloc_size;

        if next_bump > constants::BLOCK_SIZE {
            None
        } else {
            let offset = self.cursor;
            self.cursor = next_bump;
            unsafe {
                Some(self.block.as_ptr().add(offset) as *const u8)
            }
        }
    }

    pub fn find_next_available_hole(&self, starting_at: usize) -> Option<(usize, usize)> {
        let mut count = 0;
        let mut start: Option<usize> = None;
        let mut stop: usize = 0;
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

trait AllocRaw {
    fn alloc<T>(&self, object: T) -> *const T;
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

    /* only use for fraction unification */
    fn dealloc_block(ptr: BlockPtr, size: BlockSize) {
        unsafe {
            let layout = Layout::from_size_align_unchecked(size, size);

            dealloc(ptr.as_ptr(), layout);
        }
    }
}
