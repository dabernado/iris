use crate::alloc::api::AllocObject;
use crate::array::{Array, ArraySize, StackContainer};
use crate::bytecode::{
    Continuation,
    get_opcode,
    decode_i,
    decode_s
};
use crate::constants::*;
use crate::context::{Context, ContextStack};
use crate::data::*;
use crate::error::{RuntimeError, ErrorKind};
use crate::memory::{MutatorView, MutatorScope};
use crate::op::*;
use crate::safeptr::*;

/*
 * Iris Datatypes
 */
pub type Instruction<O: AllocObject> = Product<Nat, Sum<O>>;
pub type Function = Array<Instruction<()>>;

#[derive(PartialEq)]
pub enum EvalStatus {
    Pending,
    Ok,
    Err,
}

pub struct Thread {
    function: CellPtr<Function>,
    continuation: CellPtr<Continuation>,
    cxt_stack: CellPtr<ContextStack>,
    data: UntypedCellPtr,
}

impl AllocObject for Thread {}

impl Thread {
    pub fn alloc_with_arg<'guard>(
        mem: &'guard MutatorView,
        data: ScopedPtr<'guard, Product<Function, ()>>
    )
        -> Result<ScopedPtr<'guard, Thread>, RuntimeError>
    {
    }

    pub fn call_func<'guard>(
        &self,
        mem: &'guard dyn MutatorScope,
        start: ArraySize,
        end: ArraySize,
        not: bool,
    ) -> Result<(), RuntimeError> {
    }

    fn eval_context<'guard>(&self, mem: &'guard MutatorView)
        -> Result<(), RuntimeError>
    {
        let cxt_stack = self.cxt_stack.get(mem);
        let cont = self.continuation.get(mem);

        match cxt_stack.top(mem)? {
            Context::Nil => {},
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
                if cont.direction() && ip == fst_op_index {
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
                if !cont.direction() && ip == right_op_index {
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
                if cont.direction() && ip == left_op_index {
                    // exit combinator
                    cxt_stack.pop(mem);
                    cont.jump(jump + 1);
                    self.data.set(root_val.get(mem).as_untyped(mem));
                }
            },
            Context::Call {
                not,
                start,
                end,
                ret
            } => {},
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

        /*
         * TODO: Reimplement instruction fetching
        let op = cont.get_next_op(mem)?;
        let opcode = get_opcode(op, cont.direction());
        */

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
            OP_FOLD => {}, // TODO: implement
            OP_UFOLD => {}, // TODO: implement
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
                self.data.set(mem.alloc(Unit::new())?.as_untyped(mem));
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
            OP_READ => {}, // TODO: FFI
            OP_WRITE => {}, // TODO: FFI
            OP_SUMS => {
                let div = decode_i(op);
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
