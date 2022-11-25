use std::slice::from_raw_parts;

use crate::alloc::api::{AllocRaw, AllocObject, RawPtr};
use crate::alloc::immix::StickyImmixHeap;
use crate::array::ArraySize;
use crate::data::Fraction;
use crate::error::{RuntimeError, ErrorKind};
use crate::safeptr::{ScopedPtr, ScopedRef};

/* Immix Heap */
pub type Heap = StickyImmixHeap;

pub struct MutatorView<'memory> {
    heap: &'memory Heap,
}

/* Mutator */
pub trait MutatorScope {}

pub trait Mutator: Sized {
    type Input;
    type Output;

    fn run(&self, mem: &MutatorView, input: Self::Input) -> Result<Self::Output, RuntimeError>;
}

impl<'memory> MutatorView<'memory> {
    pub fn new(mem: &'memory Memory) -> MutatorView<'memory> {
        MutatorView { heap: &mem.heap }
    }

    pub fn alloc<T>(&self, object: T) -> Result<ScopedPtr<'_, T>, RuntimeError>
        where T: AllocObject,
    {
        Ok(ScopedPtr::new(
            self,
            self.heap.alloc(object)?.scoped_ref(self),
        ))
    }

    pub fn dealloc<T>(&self, object: ScopedPtr<'_, T>)
        -> Result<(), RuntimeError>
        where T: AllocObject,
    {
        self.heap.dealloc(object.as_rawptr(self))?;
        Ok(())
    }

    pub fn dealloc_with_size<T>(&self, object: ScopedPtr<'_, T>, size: u32)
        -> Result<(), RuntimeError>
        where T: AllocObject,
    {
        self.heap.dealloc_with_size(object.as_rawptr(self), size as usize)?;
        Ok(())
    }

    pub fn alloc_array(
        &self,
        capacity: ArraySize
    ) -> Result<RawPtr<u8>, RuntimeError> {
        Ok(self.heap.alloc_array(capacity)?)
    }

    pub fn dealloc_array(
        &self,
        array: RawPtr<u8>,
        size: ArraySize
    ) -> Result<(), RuntimeError> {
        self.heap.dealloc_array(array, size)?;
        Ok(())
    }

    // TODO: Can't just copy whole region of memory
    pub fn alloc_frac(&self, object: ScopedPtr<'_, ()>, size: u32)
        -> Result<ScopedPtr<'_, ()>, RuntimeError>
    {
        Ok(ScopedPtr::new(
            self,
            self.heap.make_copy(object.as_rawptr(self), size as usize)?
                .scoped_ref(self),
        ))
    }

    pub fn dealloc_frac(
        &self,
        fraction: ScopedPtr<'_, Fraction>,
        object: ScopedPtr<'_, ()>,
        size: u32
    ) -> Result<(), RuntimeError>
    {
        let frac_val = unsafe {
            from_raw_parts(
                fraction.ptr().get(self).as_rawptr(self).cast::<u8>().as_ptr(),
                size as usize
            )
        };
        let obj_val = unsafe {
            from_raw_parts(
                object.as_rawptr(self).cast::<u8>().as_ptr(),
                size as usize
            )
        };

        // TODO: Unification will never succeed (pointer comparison), fix it
        let mut same = false;
        for bytes in frac_val.iter().zip(obj_val.iter()) {
            let (frac_byte, obj_byte) = bytes;
            same = *frac_byte == *obj_byte;
        }

        if same {
            self.dealloc_with_size(object, fraction.size())
        } else {
            Err(RuntimeError::new(ErrorKind::FracUnification))
        }
    }
}

impl<'memory> MutatorScope for MutatorView<'memory> {}

pub struct Memory {
    heap: Heap,
}

impl Memory {
    pub fn new() -> Memory {
        Memory { heap: StickyImmixHeap::new() }
    }

    pub fn mutate<M: Mutator>(&self, m: &M, input: M::Input) -> Result<M::Output, RuntimeError> {
        let mut guard = MutatorView::new(self);
        m.run(&mut guard, input)
    }
}
