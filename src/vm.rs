use crate::alloc::api::AllocObject;
use crate::array::{Array, ArraySize, StackContainer};
use crate::bytecode::{Function, Continuation, get_opcode, decode_i, decode_s};
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
    ) -> Result<&'guard Fraction, RuntimeError>
    {
        self.continuation.get(guard)
            .as_ref(guard)
            .get_frac(guard, index)
    }

    fn eval_context<'guard>(&self, mem: &'guard MutatorView)
        -> Result<(), RuntimeError>
    {
        let cxt_stack = self.cxt_stack.get(mem);
        let cont = self.continuation.get(mem);

        match cxt_stack.top(mem)? {
            Context::First {
                snd_op_index,
                snd_val,
                root_val
            } => {
                let ip = cont.ip();

                // if executing in reverse, will exit combinator
                // once PRODE is encountered
                // else, check if moving into second part
                if ip == snd_op_index && !cont.direction() {
                    // push Second onto context stack
                    let new_cxt = Context::Second {
                        fst_op_index: ip - 1,
                        fst_val: CellPtr::new_with(self.data.get(mem)),
                        root_val,
                    };
                }
            },
            Context::Second {
                fst_op_index,
                fst_val,
                root_val
            } => {
                let ip = cont.ip();

                // if executing forwards, will exit combinator
                // once PRODE is encountered
                // else, check if moving into first part
                if ip == fst_op_index && cont.direction() {
                    // push First onto context stack
                    let new_cxt = Context::First {
                        snd_op_index: ip + 1,
                        snd_val: CellPtr::new_with(self.data.get(mem)),
                        root_val,
                    };
                }
            },
            Context::Left {
                right_op_index,
                jump,
                root_val
            } => {
                let ip = cont.ip();

                // if executing backwards, will exit combinator
                // once SUME is encountered
                // else, check if moving out of left part
                if ip == right_op_index && !cont.direction() {
                    // exit combinator
                    cxt_stack.pop(mem);
                    cont.jump(jump + 1);
                    self.data.set(root_val.get(mem).as_untyped(mem));
                }
            },
            Context::Right {
                left_op_index,
                jump,
                root_val
            } => {
                let ip = cont.ip();

                // if executing forwards, will exit combinator
                // once SUME is encountered
                // else, check if moving out of right part
                if ip == left_op_index && cont.direction() {
                    // exit combinator
                    cxt_stack.pop(mem);
                    cont.jump(jump + 1);
                    self.data.set(root_val.get(mem).as_untyped(mem));
                }
            },
            _ => {},
        }

        Ok(())
    }

    fn eval_next_instr<'guard>(&self, mem: &'guard MutatorView)
        -> Result<EvalStatus, RuntimeError>
    {
        // check the context stack for any necessary state changes
        self.eval_context(mem)?;

        let cont = self.continuation.get(mem)
            .as_ref(mem);
        let cxt_stack = self.cxt_stack.get(mem);
        let data = self.data.get(mem);

        let op = cont.get_next_op(mem)?;
        let opcode = if !cont.direction() {
            get_opcode(&op)
        } else {
            !get_opcode(&op)
        };

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
            },
            OP_ASSRP => {
                let cast_ptr = unsafe {
                    data.cast::<Product<(), ()>>(mem)
                };

                assrp(&cast_ptr, mem);
            },
            OP_ASSLP => {
                let cast_ptr = unsafe {
                    data.cast::<Product<(), ()>>(mem)
                };

                asslp(&cast_ptr, mem);
            },
            OP_SWAPS | OP_SWAPS_R => {
                let div = decode_i(&op);
                let cast_ptr = unsafe {
                    data.cast::<Sum<()>>(mem)
                };

                swaps(&cast_ptr, div, mem);
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
            OP_EXPN => {
                let div = decode_i(&op);
                if cont.direction() {
                    let cast_ptr = unsafe {
                        data.cast::<Sum<()>>(mem)
                    };

                    let new = expn(cast_ptr, div, mem)?;
                    self.data.set(new.as_untyped(mem));
                    cont.reverse();
                } else {
                    return Err(RuntimeError::new(ErrorKind::ExpectedZero));
                }
            },
            OP_COLN => {
                let div = decode_i(&op);
                if !cont.direction() {
                    let cast_ptr = unsafe {
                        data.cast::<Sum<()>>(mem)
                    };

                    // expn and coln are basically the same function, only
                    // one runs forwards and the other backwards
                    let new = expn(cast_ptr, div, mem)?;
                    self.data.set(new.as_untyped(mem));
                    cont.reverse();
                } else {
                    return Err(RuntimeError::new(ErrorKind::ExpectedZero));
                }
            },
            OP_EXPF => {
                let index = decode_i(&op);
                let frac_ptr = self.lookup_frac(mem, index)?;

                let new_data = mem.alloc(expf(frac_ptr, mem)?)?;
                mem.dealloc(data)?;
                self.data.set(new_data.as_untyped(mem));
            },
            OP_COLF => {
                let cast_ptr = unsafe {
                    data.cast::<Product<Fraction, ()>>(mem)
                };

                colf(cast_ptr, mem)?;
                self.data.set(mem.alloc(())?);
            },
            OP_ADD => {
                let cast_ptr = unsafe { data.cast::<Product<Nat, Nat>>(mem) };
                add(&cast_ptr, mem);
            },
            OP_SUB => {
                let cast_ptr = unsafe { data.cast::<Product<Nat, Nat>>(mem) };
                sub(&cast_ptr, mem)?;
            },
            OP_ADDI => {
                let operand = decode_i(&op);
                let cast_ptr = unsafe { data.cast::<Nat>(mem) };
                addi(&cast_ptr, operand, mem);
            },
            OP_SUBI => {
                let operand = decode_i(&op);
                let cast_ptr = unsafe { data.cast::<Nat>(mem) };
                subi(&cast_ptr, operand, mem)?;
            },
            OP_MUL => {
                let cast_ptr = unsafe { data.cast::<Product<Nat, Nat>>(mem) };
                mul(&cast_ptr, mem)?;
            },
            OP_DIV => {
                let cast_ptr = unsafe { data.cast::<Product<Nat, Nat>>(mem) };
                div(&cast_ptr, mem)?;
            },
            OP_MULI => {
                let operand = decode_i(&op);
                let cast_ptr = unsafe { data.cast::<Nat>(mem) };
                muli(&cast_ptr, operand, mem);
            },
            OP_DIVI => {
                let operand = decode_i(&op);
                let cast_ptr = unsafe { data.cast::<Nat>(mem) };
                divi(&cast_ptr, operand, mem);
            },
            OP_XOR | OP_XOR_R => {
                let cast_ptr = unsafe { data.cast::<Product<Nat, Nat>>(mem) };
                xor(&cast_ptr, mem);
            },
            OP_XORI | OP_XORI_R => {
                let operand = decode_i(&op);
                let cast_ptr = unsafe { data.cast::<Nat>(mem) };
                xori(&cast_ptr, operand, mem);
            },
            OP_CSWAP | OP_CSWAP_R => {
                let cast_ptr = unsafe {
                    data.cast::<Product<Product<Nat, Nat>, Nat>>(mem)
                };
                cswap(&cast_ptr, mem);
            },
            OP_CSWAPI | OP_CSWAPI_R => {
                let operand = decode_i(&op);
                let cast_ptr = unsafe { data.cast::<Product<Nat, Nat>>(mem) };
                cswapi(&cast_ptr, operand, mem);
            },
            OP_RR => {
                let cast_ptr = unsafe { data.cast::<Product<Nat, Nat>>(mem) };
                rr(&cast_ptr, mem);
            },
            OP_RL => {
                let cast_ptr = unsafe { data.cast::<Product<Nat, Nat>>(mem) };
                rl(&cast_ptr, mem);
            },
            OP_RRI => {
                let operand = decode_i(&op);
                let cast_ptr = unsafe { data.cast::<Nat>(mem) };
                rri(&cast_ptr, operand, mem);
            },
            OP_RLI => {
                let operand = decode_i(&op);
                let cast_ptr = unsafe { data.cast::<Nat>(mem) };
                rli(&cast_ptr, operand, mem);
            },
            OP_LTI => {
                let cast_ptr = unsafe { data.cast::<Product<Nat, Nat>>(mem) };
                let new_data = mem.alloc(lti(cast_ptr, mem))?;
                self.data.set(new_data.as_untyped(mem));
            },
            OP_LTE => {
                let cast_ptr = unsafe {
                    data.cast::<Sum<Product<Nat, Nat>>>(mem)
                };
                let inner = lte(cast_ptr, mem)?;

                self.data.set(inner.as_untyped(mem));
                mem.dealloc(cast_ptr)?;
            },
            OP_LTII => {
                let div = decode_i(&op);
                let cast_ptr = unsafe { data.cast::<Nat>(mem) };
                
                let new_data = mem.alloc(ltii(cast_ptr, div, mem))?;
                self.data.set(new_data.as_untyped(mem));
            },
            OP_LTEI => {
                let div = decode_i(&op);
                let cast_ptr = unsafe { data.cast::<Sum<Nat>>(mem) };

                let inner = ltei(cast_ptr, div, mem)?;
                self.data.set(inner.as_untyped(mem));
                mem.dealloc(cast_ptr)?;
            },
            OP_CALL => {},
            OP_UNCALL => {},
            OP_START => {},
            OP_END => {},
            OP_SYSC => {},
            OP_RSYSC => {},
            OP_SUMS => {
                let (rc, lc, div) = decode_s(&op);
                let cast_ptr = unsafe { data.cast::<Sum<()>>(mem) };

                if cast_ptr.tag() < div as u32 {
                    if !cont.direction() {
                        let new_cxt = Context::Left {
                            right_op_index: cont.ip() + (lc + 1) as u32,
                            jump: rc as u32,
                            root_val: CellPtr::new_with(cast_ptr),
                        };

                        cxt_stack.push(mem, new_cxt);
                        self.data.set(cast_ptr.data().get(mem));
                    } else {
                        let new_cxt = Context::Left {
                            right_op_index: cont.ip() - rc as u32,
                            jump: rc as u32,
                            root_val: CellPtr::new_with(cast_ptr),
                        };

                        cont.jump((rc + 1) as u32); // ip - rc+1
                        cxt_stack.push(mem, new_cxt);
                        self.data.set(cast_ptr.data().get(mem));
                    }
                } else {
                    if !cont.direction() {
                        let new_cxt = Context::Right {
                            left_op_index: cont.ip() + lc as u32,
                            jump: lc as u32,
                            root_val: CellPtr::new_with(cast_ptr),
                        };

                        cont.jump((lc + 1) as u32); // ip + lc+1
                        cxt_stack.push(mem, new_cxt);
                        self.data.set(cast_ptr.data().get(mem));
                    } else {
                        let new_cxt = Context::Right {
                            left_op_index: cont.ip() - (rc + 1) as u32,
                            jump: lc as u32,
                            root_val: CellPtr::new_with(cast_ptr),
                        };

                        cxt_stack.push(mem, new_cxt);
                        self.data.set(cast_ptr.data().get(mem));
                    }
                }
            }
            OP_SUME => {
                let sum_cxt = cxt_stack.pop(mem)?;

                match sum_cxt {
                    Context::Left { root_val, .. } => {
                        self.data.set(root_val.get(mem).as_untyped(mem));
                    },
                    Context::Right { root_val, .. } => {
                        self.data.set(root_val.get(mem).as_untyped(mem));
                    },
                    _ => return Err(RuntimeError::new(ErrorKind::BadContext)),
                }
            },
            OP_PRODS => {}
            OP_PRODE => {},
            _ => {},
        }

        Ok(EvalStatus::Pending)
    }
}

#[cfg(test)]
mod test {}
