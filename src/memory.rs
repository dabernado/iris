use std::ptr::NonNull;

use crate::alloc::api::{AllocRaw, AllocObject, RawPtr};
use crate::alloc::immix::StickyImmixHeap;
use crate::array::ArraySize;
use crate::data::{ITypeId, ITypeHeader};
use crate::error::RuntimeError;
use crate::safeptr::{ScopedPtr, ScopedRef, UntypedPtr};

/* Immix Heap */
pub type Heap = StickyImmixHeap<ITypeHeader>;

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
    fn new(mem: &'memory Memory) -> MutatorView<'memory> {
        MutatorView { heap: &mem.heap }
    }

    pub fn alloc<T>(&self, object: T) -> Result<ScopedPtr<'_, T>, RuntimeError>
        where T: AllocObject<ITypeId>,
    {
        Ok(ScopedPtr::new(
            self,
            self.heap.alloc(object)?.scoped_ref(self),
        ))
    }

    pub fn dealloc<T>(&self, object: ScopedPtr<'_, T>)
        -> Result<(), RuntimeError>
        where T: AllocObject<ITypeId>,
    {
        self.heap.dealloc(object.as_rawptr(self))?;
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
        array: RawPtr<u8>
    ) -> Result<(), RuntimeError> {
        self.heap.dealloc_array(array)?;
        Ok(())
    }

    pub fn get_header(
        &self,
        object: UntypedPtr,
    ) -> NonNull<ITypeHeader> {
        StickyImmixHeap::<ITypeHeader>::get_header(object)
    }

    pub fn get_object(
        &self,
        header: NonNull<ITypeHeader>,
    ) -> UntypedPtr {
        StickyImmixHeap::<ITypeHeader>::get_object(header)
    }
}

impl<'memory> MutatorScope for MutatorView<'memory> {}

pub struct Memory {
    heap: Heap,
}

impl Memory {
    pub fn new() -> Memory {
        Memory { heap: StickyImmixHeap::<ITypeHeader>::new() }
    }

    pub fn mutate<M: Mutator>(&self, m: &M, input: M::Input) -> Result<M::Output, RuntimeError> {
        let mut guard = MutatorView::new(self);
        m.run(&mut guard, input)
    }
}
