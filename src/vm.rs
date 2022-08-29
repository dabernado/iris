use crate::alloc::api::AllocObject;
use crate::array::{Array, ArraySize};
use crate::bytecode::{Function, Continuation, get_opcode, decode_i};
use crate::constants::*;
use crate::context::{Context, ContextStack};
use crate::data::*;
use crate::error::{RuntimeError, ErrorKind};
use crate::memory::{MutatorView, MutatorScope};
use crate::op::*;
use crate::safeptr::{ScopedPtr, FuncPtr, CellPtr, UntypedCellPtr};

#[derive(PartialEq)]
pub enum EvalStatus {
    Pending,
    Ok,
    Err,
}

pub struct Thread {
    functions: CellPtr<Array<FuncPtr>>,
    continuation: CellPtr<Continuation>,
    cxt_stack: CellPtr<ContextStack>,
    data: UntypedCellPtr,
}

impl AllocObject for Thread {}

impl Thread {
    pub fn alloc_with_arg<'guard>(
        mem: &'guard MutatorView,
        data: UntypedCellPtr
    )
        -> Result<ScopedPtr<'guard, Thread>, RuntimeError>
    {
        let funcs = Array::<FuncPtr>::alloc_with_capacity(mem, 128)?;
        let cxts = Array::<Context>::alloc_with_capacity(mem, 256)?;

        let empty_fn = Function::alloc(mem)?;
        let cont = Continuation::alloc(mem, empty_fn)?;

        mem.alloc(Thread {
            functions: CellPtr::new_with(funcs),
            continuation: CellPtr::new_with(cont),
            cxt_stack: CellPtr::new_with(cxts),
            data: data,
        })
    }

    fn lookup_frac<'guard>(
        &self,
        guard: &'guard dyn MutatorScope,
        index: ArraySize
    ) -> Result<UntypedCellPtr, RuntimeError>
    {
        self.continuation.get(guard)
            .as_ref(guard)
            .get_frac(guard, index)
    }

    fn eval_next_instr<'guard>(&self, mem: &'guard MutatorView)
        -> Result<EvalStatus, RuntimeError>
    {
        let cont = self.continuation.get(mem)
            .as_ref(mem);
        let cxt_stack = self.cxt_stack.get(mem);
        let data = self.data.get(mem);

        let op = cont.get_next_op(mem)?;
        let opcode = get_opcode(&op);

        match opcode {
            OP_ID | OP_ID_R => {},
            OP_ZEROI => {
                let new_data = mem.alloc(zeroi(data))?;
                self.data.set(new_data.as_untyped(mem));
            },
            OP_ZEROE => {
                let cast_ptr = unsafe { data.cast::<Sum<()>>(mem) };
                let inner = zeroe(cast_ptr, mem);

                self.data.set(inner.as_untyped(mem));
                mem.dealloc(cast_ptr)?;
            },
            OP_UNITI => {
                let new_data = mem.alloc(uniti(data))?;
                self.data.set(new_data.as_untyped(mem));
            },
            OP_UNITE => {
                let cast_ptr = unsafe { data.cast::<Product<Unit, ()>>(mem) };
                let inner = unite(cast_ptr, mem)?;

                self.data.set(inner.as_untyped(mem));
                mem.dealloc(cast_ptr)?;
            },
            OP_SWAPP | OP_SWAPP_R => {
                let cast_ptr = unsafe { data.cast::<Product<(), ()>>(mem) };

                swapp(&cast_ptr, mem);
                self.data.set(cast_ptr.as_untyped(mem));
            },
            OP_ASSRP => {
                let cast_ptr = unsafe {
                    data.cast::<Product<(), ()>>(mem)
                };

                assrp(&cast_ptr, mem);
                self.data.set(cast_ptr.as_untyped(mem));
            },
            OP_ASSLP => {
                let cast_ptr = unsafe {
                    data.cast::<Product<(), ()>>(mem)
                };

                asslp(&cast_ptr, mem);
                self.data.set(cast_ptr.as_untyped(mem));
            },
            OP_SWAPS | OP_SWAPS_R => {
                let div = decode_i(&op);
                let cast_ptr = unsafe {
                    data.cast::<Sum<()>>(mem)
                };

                swaps(&cast_ptr, div, mem);
                self.data.set(cast_ptr.as_untyped(mem));
            },
            OP_ASSRS | OP_ASSLS => {},
            OP_DIST => {
                let div = decode_i(&op);
                let cast_ptr = unsafe {
                    data.cast::<Product<Sum<()>, ()>>(mem)
                };

                let sum = dist(cast_ptr, div, mem)?;
                self.data.set(sum.as_untyped(mem));
            },
            OP_FACT => {
                let div = decode_i(&op);
                let cast_ptr = unsafe {
                    data.cast::<Sum<Product<(), ()>>>(mem)
                };

                let prod = fact(cast_ptr, div, mem)?;
                self.data.set(prod.as_untyped(mem));
            },
            OP_EXPN => {},
            OP_COLN => {},
            OP_ADD => {},
            OP_SUB => {},
            OP_ADDI => {},
            OP_SUBI => {},
            OP_MUL => {},
            OP_DIV => {},
            OP_MULI => {},
            OP_DIVI => {},
            OP_XOR | OP_XOR_R => {},
            OP_XORI | OP_XORI_R => {},
            OP_CSWAP | OP_CSWAP_R => {},
            OP_CSWAPI | OP_CSWAPI_R => {},
            OP_RR => {},
            OP_RL => {},
            OP_RRI => {},
            OP_RLI => {},
            OP_CALL => {},
            OP_UNCALL => {},
            OP_FOLW => {},
            OP_RET => {},
            OP_START => {},
            OP_END => {},
            OP_EVAL => {},
            OP_DEVAL => {},
            OP_SYSC => {},
            OP_RSYSC => {},
            OP_EXPF => {},
            OP_COLF => {},
            OP_SUMC | OP_SUMC_R => {},
            OP_PRODC | OP_PRODC_R => {},
            _ => {},
        }

        Ok(EvalStatus::Pending)
    }
}

#[cfg(test)]
mod test {}
