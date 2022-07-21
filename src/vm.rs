use std::cell::Cell;

use crate::array::{Array, ArraySize};
use crate::bytecode::{Function, Continuation};
use crate::constants::*;
use crate::error::{RuntimeError, ErrorKind};
use crate::memory::{MutatorView, MutatorScope};
use crate::safeptr::{UntypedPtr, ScopedPtr, FuncPtr};

#[derive(PartialEq)]
pub enum EvalStatus {
    Pending,
    Ok,
    Err,
}

pub struct Thread {
    functions: Array<FuncPtr>,
    continuation: Cell<Continuation>,
    cxt_stack: CellPtr<ContextStack>,
    data: Cell<UntypedPtr>,
}

impl Thread {
    pub fn alloc_with_arg<'guard>(mem: &'guard MutatorView, data: UntypedPtr)
        -> Result<ScopedPtr<'guard, Thread>, RuntimeError>
    {
        let funcs = Array::<FuncPtr>::alloc_with_capacity(mem, 128)?;
        let cxts = Array::<Context>::alloc_with_capacity(mem, 256)?;

        let empty_fn = Function::alloc(mem)?;
        let cont = Continuation::alloc(mem, empty_fn)?;

        mem.alloc(Thread {
            functions: func,
            continuation: cont,
            cxt_stack: cxts,
            data: Cell::new(data),
        })
    }

    fn frac_lookup<'guard>(
        &self,
        guard: &'guard dyn MutatorScope,
        index: ArraySize
    ) -> Result<UntypedPtr, RuntimeError>
    {
        self.continuation.get().get_frac(guard, index)
    }

    fn eval_next_instr<'guard>(&self, mem: &'guard MutatorView)
        -> Result<EvalStatus<'guard>, RuntimeError>
    {
        let funcs = self.functions.get(mem);
        let cxt_stack = self.cxt_stack.get(mem);
        let cont = self.continuation.get(mem);
        let data = self.data.get(mem);

        let data_header = mem.get_header(data);
        let data_type = data_header.type_id();

        let op = cont.get_next_opcode(mem)?;
        let opcode = get_opcode(op);

        match opcode {
            OP_ID | OP_ID_R => {},
            OP_ZEROI => {
                let data_ref = RawPtr::new(data.as_ptr());

                let new_val = mem.alloc(
                    zeroi(data_ref)
                )?;

                self.data.set(new_val.as_untyped());
            },
            OP_ZEROE => {
                if data_type == ITypeId::Sum {
                    let data_ref = RawPtr::new(data.as_ptr());

                    let new_data = zeroe(mem, data_ref);
                    self.data.set(new_data.as_untyped());
                } else {
                    Err(RuntimeError::new(ErrorKind::TypeError))
                }
            },
            OP_UNITI => {
                let data_ref = RawPtr::new(data.as_ptr());

                let new_val = mem.alloc(
                    uniti(data_ref)
                )?;

                self.data.set(new_val.as_untyped());
            },
            OP_UNITE => {
                if data_type == ITypeId::Prod {
                    let data_ref = RawPtr::new(data.as_ptr());

                    let new_data = unite(mem, data_ref);
                    self.data.set(new_data.as_untyped());
                } else {
                    Err(RuntimeError::new(ErrorKind::TypeError))
                }
            },
            OP_SWAPP | OP_SWAPP_R => {
                if data_type == ITypeId::Prod {
                    let data_ref = RawPtr::new(data.as_ptr());

                    swapp(&data_ref);
                } else {
                    Err(RuntimeError::new(ErrorKind::TypeError))
                }
            },
            OP_ASSRP => {
                if data_type == ITypeId::Prod {
                    let data_ref = RawPtr::new(data.as_ptr());

                    assrp(&data_ref);
                } else {
                    Err(RuntimeError::new(ErrorKind::TypeError))
                }
            },
            OP_ASSLP => {
                if data_type == ITypeId::Prod {
                    let data_ref = RawPtr::new(data.as_ptr());

                    asslp(&data_ref);
                } else {
                    Err(RuntimeError::new(ErrorKind::TypeError))
                }
            },
            OP_DIST => {},
            OP_FACT => {},
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
            OP_UADD => {},
            OP_USUB => {},
            OP_UADDI => {},
            OP_USUBI => {},
            OP_UMUL => {},
            OP_UDIV => {},
            OP_UMULI => {},
            OP_UDIVI => {},
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
            OP_SWAPS | OP_SWAPS_R => {
                if data_type == ITypeId::Sum {
                    let data_ref = RawPtr::new(data.as_ptr());

                    swaps(&data_ref);
                } else {
                    Err(RuntimeError::new(ErrorKind::TypeError))
                }
            },
            OP_ASSRS => {},
            OP_ASSLS => {},
            _ => {},
        }
    }
}

#[cfg(test)]
mod test {}
