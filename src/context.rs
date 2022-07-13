use crate::array::{
    Array,
    ArraySize,
    DEFAULT_ARRAY_SIZE,
    INTERIOR_ONLY,
    default_array_growth,
    Container,
};
use crate::alloc::api::AllocObject;
use crate::data::ITypeId;
use crate::error::{RuntimeError, ErrorKind};
use crate::safeptr::UntypedPtr;
use crate::memory::{MutatorView, MutatorScope};

/* Context Type */
pub enum Context {
    Nil,
    First {
        snd_op_index: ArraySize,
        snd_val: UntypedPtr,
        root_val: UntypedPtr,
    },
    Second {
        fst_op_index: ArraySize,
        fst_val: UntypedPtr,
        root_val: UntypedPtr,
    },
    Left(ArraySize),
    Right(ArraySize),
    Indirect {
        last: UntypedPtr,
        current: UntypedPtr,
    },
    Call {
        last: ArraySize,
        current: ArraySize,
    }
}

impl AllocObject<ITypeId> for Context {
    const TYPE_ID: ITypeId = ITypeId::Context;
}

// mostly for implementing the context stack
pub trait StackContainer<T: Sized + Clone>: Container<T> {
    fn push<'guard>(&self, mem: &'guard MutatorView, item: T) -> Result<(), RuntimeError>;
    fn pop<'guard>(&self, _guard: &'guard dyn MutatorScope) -> Result<T, RuntimeError>;
    fn top<'guard>(&self, _guard: &'guard dyn MutatorScope) -> Result<T, RuntimeError>;
}

impl <T: Sized + Clone> StackContainer<T> for Array<T> {
    fn push<'guard>(&self, mem: &'guard MutatorView, item: T) -> Result<(), RuntimeError> {
        if self.borrow.get() != INTERIOR_ONLY {
            return Err(RuntimeError::new(ErrorKind::MutableBorrowError));
        }

        let length = self.length.get();
        let mut array = self.data.get();
        let capacity = array.capacity();

        if length == capacity {
            if capacity == 0 {
                array.resize(mem, DEFAULT_ARRAY_SIZE)?;
            } else {
                array.resize(mem, default_array_growth(capacity)?)?;
            }

            self.data.set(array);
        }

        self.length.set(length + 1);
        self.write(mem, length, item);
        Ok(())
    }

    fn pop<'guard>(&self, mem: &'guard MutatorView, item: T) -> Result<T, RuntimeError> {
        if self.borrow.get() != INTERIOR_ONLY {
            return Err(RuntimeError::new(ErrorKind::MutableBorrowError));
        }

        let length = self.length.get();
        if length == 0 {
            return Err(RuntimeError::new(ErrorKind::BoundsError));
        } else {
            let last = length - 1;
            let item = self.read(mem, last)?;
            self.length.set(last);
            Ok(item)
        }
    }

    fn top<'guard>(&self, mem: &'guard MutatorView, item: T) -> Result<T, RuntimeError> {
        if self.borrow.get() != INTERIOR_ONLY {
            return Err(RuntimeError::new(ErrorKind::MutableBorrowError));
        }

        let length = self.length.get();
        if length == 0 {
            return Err(RuntimeError::new(ErrorKind::BoundsError));
        } else {
            let item = self.read(mem, length - 1)?;
            Ok(item)
        }
    }
}
