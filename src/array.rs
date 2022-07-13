use std::ptr::{read, write, NonNull};
use std::cell::Cell;
use std::mem::size_of;
use std::slice::from_raw_parts_mut;

use crate::alloc::api::AllocObject;
use crate::context::StackContainer;
use crate::data::ITypeId;
use crate::error::{RuntimeError, ErrorKind};
use crate::memory::{MutatorView, MutatorScope};
use crate::safeptr::ScopedPtr;

pub type ArraySize = u32;
pub const DEFAULT_ARRAY_SIZE: ArraySize = 8;

type BorrowFlag = isize;
pub const INTERIOR_ONLY: BorrowFlag = 0;
pub const EXPOSED_MUTABLY: BorrowFlag = 1;

/* Safe Array */
#[derive(Clone)]
pub struct Array<T: Sized + Clone> {
    length: Cell<ArraySize>,
    data: Cell<RawArray<T>>,
    borrow: Cell<BorrowFlag>,
}

impl<T: Sized + Clone> Array<T> {
    pub fn alloc<'guard>(
        mem: &'guard MutatorView,
    ) -> Result<ScopedPtr<'guard, Array<T>>, RuntimeError>
        where Array<T>: AllocObject<ITypeId>
    {
        mem.alloc(Array::new())
    }

    pub fn alloc_clone<'guard>(
        mem: &'guard MutatorView,
        from_array: ScopedPtr<'guard, Array<T>>
    ) -> Result<ScopedPtr<'guard, Array<T>>, RuntimeError>
        where Array<T>: AllocObject<ITypeId> + ContainerFromSlice<T>
    {
        from_array.access_slice(mem, |items| ContainerFromSlice::from_slice(mem, items))
    }

    pub fn alloc_with_capacity<'guard>(
        mem: &'guard MutatorView,
        capacity: ArraySize
    ) -> Result<ScopedPtr<'guard, Array<T>>, RuntimeError>
        where Array<T>: AllocObject<ITypeId>
    {
        mem.alloc(Array::with_capacity(mem, capacity))
    }

    fn get_offset(&self, index: ArraySize) -> Result<*mut T, RuntimeError> {
        if index >= self.length.get() {
            Err(RuntimeError::new(ErrorKind::BoundsError))
        } else {
            let ptr = self
                .data
                .get()
                .as_ptr()
                .ok_or_else(|| RuntimeError::new(ErrorKind::BoundsError))?;

            let dest_ptr = unsafe { ptr.offset(index as isize) as *mut T };
            Ok(dest_ptr)
        }
    }

    fn write<'guard>(
        &self,
        _guard: &'guard dyn MutatorScope,
        index: ArraySize,
        item: T
    ) -> Result<&T, RuntimeError> {
        unsafe {
            let dest = self.get_offset(index)?;
            write(dest, item);
            Ok(&*dest as &T)
        }
    }

    fn read<'guard>(
        &self,
        _guard: &'guard dyn MutatorScope,
        index: ArraySize
    ) -> Result<T, RuntimeError> {
        unsafe {
            let dest = self.get_offset(index)?;
            Ok(read(dest))
        }
    }

    pub fn read_ref<'guard>(
        &self,
        _guard: &'guard dyn MutatorScope,
        index: ArraySize
    ) -> Result<&T, RuntimeError> {
        unsafe {
            let dest = self.get_offset(index)?;
            Ok(&*dest as &T)
        }
    }

    pub unsafe fn as_slice<'guard>(&self, _guard: &'guard dyn MutatorScope) -> &mut [T] {
        if let Some(ptr) = self.data.get().as_ptr() {
            from_raw_parts_mut(ptr as *mut T, self.length.get() as usize)
        } else {
            &mut []
        }
    }

    pub unsafe fn as_capacity_slice<'guard>(
        &self,
        _guard: &'guard dyn MutatorScope
    ) -> &mut [T] {
        if let Some(ptr) = self.data.get().as_ptr() {
            from_raw_parts_mut(ptr as *mut T, self.data.get().capacity() as usize)
        } else {
            &mut []
        }
    }
}

/* Raw Array */
pub struct RawArray<T: Sized> {
    capacity: ArraySize,
    ptr: Option<NonNull<T>>,
}

impl<T: Sized> RawArray<T> {
    pub fn new() -> RawArray<T> {
        RawArray {
            capacity: 0,
            ptr: None,
        }
    }

    pub fn with_capacity<'scope>(
        mem: &'scope MutatorView,
        capacity: u32
    ) -> Result<RawArray<T>, RuntimeError> {
        let capacity_bytes = capacity
            .checked_mul(size_of::<T>() as ArraySize)
            .ok_or(RuntimeError::new(ErrorKind::BadAllocationRequest))?;

        Ok(RawArray {
            capacity: capacity,
            ptr: NonNull::new(mem.alloc_array(capacity_bytes)?.as_ptr() as *mut T),
        })
    }

    // if new size < size, returns pointer to eliminated values
    pub fn resize<'scope>(
        &mut self,
        mem: &'scope MutatorView,
        new_capacity: ArraySize,
    ) -> Result<Option<RawArray<T>>, RuntimeError> {
        if new_capacity == 0 {
            let old_array = RawArray {
                capacity: self.capacity,
                ptr: self.ptr,
            };
            self.capacity = 0;
            self.ptr = None;

            return Ok(Some(old_array));
        }

        match self.ptr {
            Some(old_ptr) => {
                let old_capacity = self.capacity;
                let old_capacity_bytes = size_of::<T>() as ArraySize * self.capacity;
                let old_ptr = old_ptr.as_ptr();

                let new_capacity_bytes = new_capacity
                    .checked_mul(size_of::<T>() as ArraySize)
                    .ok_or(RuntimeError::new(ErrorKind::BadAllocationRequest))?;
                let new_ptr = mem.alloc_array(new_capacity_bytes)?.as_ptr() as *mut T;

                let (old_slice, new_slice) = unsafe {
                    (
                        from_raw_parts_mut(old_ptr as *mut u8, old_capacity_bytes as usize),
                        from_raw_parts_mut(new_ptr as *mut u8, new_capacity_bytes as usize),
                    )
                };

                for (src, dest) in old_slice.iter().zip(new_slice) {
                    *dest = *src;
                }

                if old_capacity <= new_capacity {
                    // new array is larger or equal, no information is lost
                    self.ptr = NonNull::new(new_ptr);
                    self.capacity = new_capacity;

                    Ok(None)
                } else {
                    // return remaining values as separate array
                    let leftovers = RawArray {
                        capacity: old_capacity - new_capacity,
                        ptr: NonNull::new(
                            unsafe { old_ptr.offset(new_capacity as isize) as *mut T }
                        ),
                    };
                    self.ptr = NonNull::new(new_ptr);
                    self.capacity = new_capacity;

                    Ok(Some(leftovers))
                }
            },

            None => {
                *self = Self::with_capacity(mem, new_capacity)?;
                Ok(None)
            }
        }
    }

    pub fn capacity(&self) -> ArraySize { self.capacity }
    
    pub fn as_ptr(&self) -> Option<*const T> {
        match self.ptr {
            Some(ptr) => Some(ptr.as_ptr()),
            None => None,
        }
    }
}

impl<T: Sized> Copy for RawArray<T> {}
impl<T: Sized> Clone for RawArray<T> {
    fn clone(&self) -> Self {
        RawArray {
            capacity: self.capacity,
            ptr: self.ptr,
        }
    }
}

/* Container Traits */
pub trait Container<T: Sized + Clone>: Sized {
    fn new() -> Self;
    fn length(&self) -> ArraySize;
    fn clear<'guard>(&self, _guard: &'guard MutatorView) -> Result<(), RuntimeError>;
    fn with_capacity<'guard>(
        mem: &'guard MutatorView,
        capacity: ArraySize
    ) -> Result<Self, RuntimeError>;
}

impl<T: Sized + Clone> Container<T> for Array<T> {
    fn new() -> Array<T> {
        Array {
            length: Cell::new(0),
            data: Cell::new(RawArray::new()),
            borrow: Cell::new(INTERIOR_ONLY),
        }
    }

    fn length(&self) -> ArraySize { self.length.get() }

    fn clear<'guard>(&self, _guard: &'guard MutatorView) -> Result<(), RuntimeError> {
        if self.borrow.get() != INTERIOR_ONLY {
            Err(RuntimeError::new(ErrorKind::MutableBorrowError))
        } else {
            self.length.set(0);
            Ok(())
        }
    }

    fn with_capacity<'guard>(
        mem: &'guard MutatorView,
        capacity: ArraySize
    ) -> Result<Self, RuntimeError> {
        Ok(Array {
            length: Cell::new(0),
            data: Cell::new(RawArray::with_capacity(mem, capacity)?),
            borrow: Cell::new(INTERIOR_ONLY),
        })
    }
}

// for filling an array with a constant in one operation
pub trait FillContainer<T: Sized + Clone>: Container<T> {
    fn fill<'guard>(
        &self,
        mem: &'guard MutatorView,
        size: ArraySize,
        item: T,
    ) -> Result<(), RuntimeError>;
}

impl<T: Sized + Clone> FillContainer<T> for Array<T> {
    fn fill<'guard>(
        &self,
        mem: &'guard MutatorView,
        size: ArraySize,
        item: T,
    ) -> Result<(), RuntimeError> {
        let length = self.length();
        if length > size {
            Ok(())
        } else {
            let mut array = self.data.get();
            let capacity = array.capacity();

            if size > capacity {
                if capacity == 0 {
                    array.resize(mem, DEFAULT_ARRAY_SIZE)?;
                } else {
                    array.resize(mem, default_array_growth(capacity)?)?;
                }

                self.data.set(array);
            }

            self.length.set(size);

            for index in length..size {
                self.write(mem, index, item.clone())?;
            }

            Ok(())
        }
    }
}

// useful for typical vector indexing
pub trait IndexedContainer<T: Sized + Clone>: Container<T> {
    fn get<'guard>(
        &self,
        _guard: &'guard dyn MutatorScope,
        index: ArraySize,
    ) -> Result<T, RuntimeError>;

    fn set<'guard>(
        &self,
        _guard: &'guard dyn MutatorScope,
        index: ArraySize,
        item: T,
    ) -> Result<(), RuntimeError>;
}

impl<T: Sized + Clone> IndexedContainer<T> for Array<T> {
    fn get<'guard>(
        &self,
        guard: &'guard dyn MutatorScope,
        index: ArraySize,
    ) -> Result<T, RuntimeError> { self.read(guard, index) }

    fn set<'guard>(
        &self,
        guard: &'guard dyn MutatorScope,
        index: ArraySize,
        item: T,
    ) -> Result<(), RuntimeError> {
        self.write(guard, index, item)?;
        Ok(())
    }
}

// for accessing arrays as slices
pub trait SliceableContainer<T: Sized + Clone>: IndexedContainer<T> {
    fn access_slice<'guard, F, R>(&self, _guard: &'guard dyn MutatorScope, f: F) -> R
        where F: FnOnce(&mut [T]) -> R;
}

impl<T: Sized + Clone> SliceableContainer<T> for Array<T> {
    fn access_slice<'guard, F, R>(&self, guard: &'guard dyn MutatorScope, f: F) -> R
        where F: FnOnce(&mut [T]) -> R
    {
        self.borrow.set(EXPOSED_MUTABLY);
        let slice = unsafe { self.as_slice(guard) };
        let result = f(slice);
        self.borrow.set(INTERIOR_ONLY);
        result
    }
}

// for converting a slice into an array
pub trait ContainerFromSlice<T: Sized + Clone>: Container<T> {
    fn from_slice<'guard>(
        mem: &'guard MutatorView,
        data: &[T],
    ) -> Result<ScopedPtr<'guard, Self>, RuntimeError>;
}

impl<T: Sized + Clone> ContainerFromSlice<T> for Array<T>
    where Array<T>: AllocObject<ITypeId>,
{
    fn from_slice<'guard>(
        mem: &'guard MutatorView,
        data: &[T],
    ) -> Result<ScopedPtr<'guard, Self>, RuntimeError> {
        let array = Array::alloc_with_capacity(mem, data.len() as ArraySize)?;
        let slice = unsafe { array.as_capacity_slice(mem) };
        slice.clone_from_slice(data);
        array.length.set(data.len() as ArraySize);
        Ok(array)
    }
}

/* Helper functions */
pub fn default_array_growth(capacity: ArraySize) -> Result<ArraySize, RuntimeError> {
    if capacity == 0 {
        Ok(DEFAULT_ARRAY_SIZE)
    } else {
        capacity
            .checked_add(capacity / 2)
            .ok_or(RuntimeError::new(ErrorKind::BadAllocationRequest))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::memory::{Memory, Mutator, MutatorView};

    #[test]
    fn array_generic_push_and_pop() {
        let mem = Memory::new();

        struct Test {}
        impl Mutator for Test {
            type Input = ();
            type Output = ();

            fn run(
                &self,
                view: &MutatorView,
                _input: Self::Input,
            ) -> Result<Self::Output, RuntimeError> {
                let array: Array<i32> = Array::new();

                for i in 0..1000 {
                    array.push(view, i)?;
                }

                for i in 0..1000 {
                    assert!(array.pop(view)? == 999 - i);
                }

                Ok(())
            }
        }

        let test = Test {};
        mem.mutate(&test, ()).unwrap();
    }

    #[test]
    fn array_generic_indexing() {
        let mem = Memory::new();

        struct Test {}
        impl Mutator for Test {
            type Input = ();
            type Output = ();

            fn run(
                &self,
                view: &MutatorView,
                _input: Self::Input,
            ) -> Result<Self::Output, RuntimeError> {
                let array: Array<i32> = Array::new();

                for i in 0..12 {
                    array.push(view, i)?;
                }

                assert!(array.get(view, 0) == Ok(0));
                assert!(array.get(view, 4) == Ok(4));

                for i in 12..1000 {
                    match array.get(view, i) {
                        Ok(_) => panic!("Array index should have been out of bounds!"),
                        Err(e) => assert!(*e.error_kind() == ErrorKind::BoundsError),
                    }
                }

                Ok(())
            }
        }

        let test = Test {};
        mem.mutate(&test, ()).unwrap();
    }

    #[test]
    fn array_with_capacity_and_realloc() {
        let mem = Memory::new();

        struct Test {}
        impl Mutator for Test {
            type Input = ();
            type Output = ();

            fn run(
                &self,
                view: &MutatorView,
                _input: Self::Input,
            ) -> Result<Self::Output, RuntimeError> {
                let array: Array<i32> = Array::with_capacity(view, 256)?;

                let ptr_before = array.data.get().as_ptr();

                for _ in 0..256 {
                    StackContainer::push(&array, view, 0)?;
                }

                let ptr_after = array.data.get().as_ptr();
                assert!(ptr_before == ptr_after);

                StackContainer::push(&array, view, 0)?;
                let ptr_realloc = array.data.get().as_ptr();
                assert!(ptr_before != ptr_realloc);

                Ok(())
            }
        }

        let test = Test {};
        mem.mutate(&test, ()).unwrap();
    }
}
