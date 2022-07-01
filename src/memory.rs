use crate::data::{ITypeId, ITypeHeader};
use crate::safeptr::ScopedPtr;
use crate::alloc::api::AllocObject;
use crate::alloc::immix::StickyImmixHeap;

/* Immix Heap */
pub type Heap = StickyImmixHeap<ITypeHeader>;

pub struct MutatorView<'memory> {
    heap: &'memory Heap,
}

impl<'memory> MutatorView<'memory> {
    pub fn alloc<T>(&self, object: T) -> Result<ScopedPtr<'_, T>, RuntimeError>
        where T: AllocObject<ITypeId>,
    {
        Ok(ScopedPtr::new(
            self,
            self.heap.alloc(object)?.scoped_ref(self),
        ))
    }
}

/* Mutator */
pub trait MutatorScope {}

pub trait Mutator: Sized {
    type Input;
    type Output;

    fn run(&self, mem: &MutatorView, input: Self::Input) -> Result<Self::Output, RuntimeError>;
}

pub struct Memory {
    heap: Heap,
}

impl Memory {
    pub fn mutate<M: Mutator>(&self, m: &M, input: M::Input) -> Result<M::Output, RuntimeError> {
        let mut guard = MutatorView::new(self);
        m.run(&mut guard, input)
    }
}
