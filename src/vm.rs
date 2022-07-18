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

        let op = cont.get_next_opcode(mem)?;
        let opcode = get_opcode(op);

        match opcode {
            OP_ID | OP_ID_R => {},
            OP_ZEROI => {},
            OP_ZEROE => {},
            OP_UNITI => {},
            OP_UNITE => {},
            OP_SWAPP | OP_SWAPP_R => {},
            OP_ASSRP => {},
            OP_ASSLP => {},
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
        }
    }
}
