use crate::alloc::api::AllocObject;
use crate::array::{Array, ArraySize, StackContainer};
use crate::bytecode::{
    Function,
    Continuation,
    get_opcode,
    decode_i,
    decode_s,
    decode_c
};
use crate::constants::*;
use crate::context::{Context, ContextStack};
use crate::data::*;
use crate::error::{RuntimeError, ErrorKind};
use crate::memory::{MutatorView, MutatorScope};
use crate::op::*;
use crate::safeptr::*;

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
        cxts.push(mem, Context::Nil)?;

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

    pub fn add_func<'guard>(
        &self,
        guard: &'guard MutatorView,
        func: ScopedPtr<'guard, Function>
    ) {
        self.functions.get(guard).push(guard, CellPtr::new_with(func));
    }

    pub fn call_func<'guard>(
        &self,
        mem: &'guard dyn MutatorScope,
        index: ArraySize,
        not: bool,
    ) -> Result<(), RuntimeError> {
        let cont = self.continuation.get(mem);
        let func_ptr = self.functions.get(mem)
            .read_ref(mem, index)?
            .get(mem);

        cont.set_func(func_ptr);
        if not { cont.reverse(); }

        cont.reset(func_ptr.length());
        Ok(())
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

                    cxt_stack.pop(mem);
                    cxt_stack.push(mem, new_cxt);
                    self.data.set(snd_val.get(mem));
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

                    cxt_stack.pop(mem);
                    cxt_stack.push(mem, new_cxt);
                    self.data.set(fst_val.get(mem));
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

    pub fn eval_next_instr<'guard>(&self, mem: &'guard MutatorView)
        -> Result<EvalStatus, RuntimeError>
    {
        // check the context stack for any necessary state changes
        self.eval_context(mem)?;

        let cont = self.continuation.get(mem)
            .as_ref(mem);
        let cxt_stack = self.cxt_stack.get(mem);
        let data = self.data.get(mem);

        let op = cont.get_next_op(mem)?;
        let opcode = get_opcode(op, cont.direction());

        match opcode {
            OP_ID | OP_ID_R => {}, // identity
            OP_ZEROI => {
                let new_data = mem.alloc(zeroi(data))?;
                self.data.set(new_data.as_untyped(mem));
            },
            OP_ZEROE => {
                let cast_ptr = unsafe { data.cast::<Sum<()>>(mem) };
                let inner = zeroe(cast_ptr, mem);

                self.data.set(inner);
                mem.dealloc(cast_ptr)?;
            },
            OP_UNITI => {
                let new_data = mem.alloc(uniti(data, mem)?)?;
                self.data.set(new_data.as_untyped(mem));
            },
            OP_UNITE => {
                let cast_ptr = unsafe { data.cast::<Product<Unit, ()>>(mem) };
                let inner = unite(cast_ptr, mem);

                self.data.set(inner.as_untyped(mem));
                mem.dealloc(cast_ptr.fst(mem))?;
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
                let (lc, rc) = decode_s(op);
                let cast_ptr = unsafe {
                    data.cast::<Sum<()>>(mem)
                };

                swaps(&cast_ptr, lc, rc, mem);
            },
            OP_ASSRS | OP_ASSLS => {}, // op-equivalent to ID
            OP_DIST => {
                let (lc, rc) = decode_s(op);
                let cast_ptr = unsafe {
                    data.cast::<Product<Sum<()>, ()>>(mem)
                };

                let sum = dist(cast_ptr, lc, rc, mem)?;
                self.data.set(sum.as_untyped(mem));
            },
            OP_FACT => {
                let (lc, rc) = decode_s(op);
                let cast_ptr = unsafe {
                    data.cast::<Sum<Product<(), ()>>>(mem)
                };

                let prod = fact(cast_ptr, lc, rc, mem)?;
                self.data.set(prod.as_untyped(mem));
            },
            OP_EXPN => {
                let div = decode_i(op);
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
                let div = decode_i(op);
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
                let index = decode_i(op);
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
                let operand = decode_i(op);
                let cast_ptr = unsafe { data.cast::<Nat>(mem) };
                addi(&cast_ptr, operand, mem);
            },
            OP_SUBI => {
                let operand = decode_i(op);
                let cast_ptr = unsafe { data.cast::<Nat>(mem) };
                subi(&cast_ptr, operand, mem)?;
            },
            OP_XOR | OP_XOR_R => {
                let cast_ptr = unsafe { data.cast::<Product<Nat, Nat>>(mem) };
                xor(&cast_ptr, mem);
            },
            OP_XORI | OP_XORI_R => {
                let operand = decode_i(op);
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
                let operand = decode_i(op);
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
                let operand = decode_i(op);
                let cast_ptr = unsafe { data.cast::<Nat>(mem) };
                rri(&cast_ptr, operand, mem);
            },
            OP_RLI => {
                let operand = decode_i(op);
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
                let div = decode_i(op);
                let cast_ptr = unsafe { data.cast::<Nat>(mem) };
                
                let new_data = mem.alloc(ltii(cast_ptr, div, mem))?;
                self.data.set(new_data.as_untyped(mem));
            },
            OP_LTEI => {
                let div = decode_i(op);
                let cast_ptr = unsafe { data.cast::<Sum<Nat>>(mem) };

                let inner = ltei(cast_ptr, div, mem)?;
                self.data.set(inner.as_untyped(mem));
                mem.dealloc(cast_ptr)?;
            },
            OP_CALL => {
                let index = decode_i(op);
                let dir = cont.direction();
                let new_cxt = Context::Call {
                    ret_func: CellPtr::new_with(cont.current_func(mem)),
                    ret_addr: if dir { cont.ip() - 1 } else { cont.ip() + 1 },
                    not: dir,
                };

                self.call_func(mem, index, dir);
                cxt_stack.push(mem, new_cxt);
            },
            OP_UNCALL => {
                let index = decode_i(op);
                let dir = cont.direction();
                let new_cxt = Context::Call {
                    ret_func: CellPtr::new_with(cont.current_func(mem)),
                    ret_addr: if dir { cont.ip() - 1 } else { cont.ip() + 1 },
                    not: !dir,
                };

                self.call_func(mem, index, !dir);
                cxt_stack.push(mem, new_cxt);
            },
            OP_START => {}, // op-equivalent to ID
            OP_END => {
                match cxt_stack.top(mem)? {
                    Context::Call { ret_func, ret_addr, not } => {
                        if not { cont.reverse(); }
                        cont.set_func(ret_func.get(mem));
                        cont.set_ip(ret_addr);
                    },
                    Context::Nil => return Ok(EvalStatus::Ok),
                    _ => return Err(RuntimeError::new(ErrorKind::BadContext)),
                }
            },
            OP_SYSC => {}, // TODO: FFI
            OP_RSYSC => {}, // TODO: FFI
            OP_SUMS => {
                let (rc, lc, div) = decode_c(op);
                let cast_ptr = unsafe { data.cast::<Sum<()>>(mem) };

                if cast_ptr.tag() < div as u32 {
                    if !cont.direction() {
                        let new_cxt = Context::Left {
                            right_op_index: cont.ip() + (lc + 1) as u32,
                            jump: rc as u32,
                            root_val: CellPtr::new_with(cast_ptr),
                        };

                        cxt_stack.push(mem, new_cxt);
                        self.data.set(cast_ptr.data(mem));
                    } else {
                        let new_cxt = Context::Left {
                            right_op_index: cont.ip() - rc as u32,
                            jump: rc as u32,
                            root_val: CellPtr::new_with(cast_ptr),
                        };

                        cont.jump((rc + 1) as u32); // ip - rc+1
                        cxt_stack.push(mem, new_cxt);
                        self.data.set(cast_ptr.data(mem));
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
                        self.data.set(cast_ptr.data(mem));
                    } else {
                        let new_cxt = Context::Right {
                            left_op_index: cont.ip() - (rc + 1) as u32,
                            jump: lc as u32,
                            root_val: CellPtr::new_with(cast_ptr),
                        };

                        cxt_stack.push(mem, new_cxt);
                        self.data.set(cast_ptr.data(mem));
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
            OP_PRODS => {
                let index = decode_i(op) + cont.ip();
                let cast_ptr = unsafe { data.cast::<Product<(), ()>>(mem) };

                if !cont.direction() {
                    let new_cxt = Context::First {
                        snd_op_index: index,
                        snd_val: CellPtr::new_with(cast_ptr.snd(mem)),
                        root_val: CellPtr::new_with(cast_ptr),
                    };

                    cxt_stack.push(mem, new_cxt);
                    self.data.set(cast_ptr.fst(mem));
                } else {
                    let new_cxt = Context::Second {
                        fst_op_index: index,
                        fst_val: CellPtr::new_with(cast_ptr.fst(mem)),
                        root_val: CellPtr::new_with(cast_ptr),
                    };

                    cxt_stack.push(mem, new_cxt);
                    self.data.set(cast_ptr.snd(mem));
                }
            }
            OP_PRODE => {
                let sum_cxt = cxt_stack.pop(mem)?;

                match sum_cxt {
                    Context::First { root_val, .. } => {
                        self.data.set(root_val.get(mem).as_untyped(mem));
                    },
                    Context::Second { root_val, .. } => {
                        self.data.set(root_val.get(mem).as_untyped(mem));
                    },
                    _ => return Err(RuntimeError::new(ErrorKind::BadContext)),
                }
            },
            _ => {},
        }

        Ok(EvalStatus::Pending)
    }

    pub fn data(&self) -> &UntypedCellPtr { &self.data }
}
