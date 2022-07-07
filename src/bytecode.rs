use crate::alloc::api::{AllocObject, ITypeId};
use crate::safeptr::{UntypedPtr, CellPtr, ScopedPtr};
use crate::array::{Array, ArraySize};
use crate::constants::*;

pub type Opcode = u32;
impl AllocObject<ITypeId> for Opcode {
    const TYPE_ID: ITypeId = ITypeId::Opcode;
}

#[derive(Clone)]
pub struct Bytecode {
    fractions: Array<UntypedPtr>,
    code: Array<Opcode>,
}

impl Bytecode {
    pub fn alloc<'guard>(
        mem: &'guard MutatorView,
    ) -> Result<ScopedPtr<'guard, Bytecode>, RuntimeError> {
        mem.alloc(Bytecode {
            fractions: Array<UntypedPtr>::new(),
            code: Array<Opcode>::new(),
        })
    }

    pub fn push<'guard>(
        &self,
        mem: &'guard MutatorView,
        op: Opcode
    ) -> Result<(), RuntimeError> { self.code.push(mem, op) }

    pub fn push_frac<'guard>(
        &self,
        mem: &'guard MutatorView,
        ptr: UntypedPtr
    ) -> Result<(), RuntimeError> { self.fractions.push(mem, ptr) }

    pub fn last_instruction(&self) -> ArraySize { self.code.length() - 1 }
    pub fn next_instruction(&self) -> ArraySize { self.code.length() }
}

pub struct InstructionStream {
    instructions: CellPtr<Bytecode>,
    ip: Cell<ArraySize>,
}

impl InstructionStream {
    pub fn alloc<'guard>(
        mem: &'guard MutatorView,
        code: ScopedPtr<'_, Bytecode>,
    ) -> Result<ScopedPtr<'guard, InstructionStream>, RuntimeError> {
        mem.alloc(InstructionStream {
            instructions: CellPtr::new_with(code),
            ip: Cell::new(0),
        })
    }

    pub fn switch_frame(&self, code: ScopedPtr<'_, Bytecode>, ip: ArraySize) {
        self.instructions.set(code);
        self.ip.set(ip);
    }

    pub fn get_next_op<'guard>(
        &self,
        guard: &'guard dyn MutatorScope,
    ) -> Result<Opcode, RuntimeError> {
        let current_ip = self.ip.get();
        let instr = self
            .instructions
            .get(guard)
            .code
            .get(guard, current_ip)?;
        self.ip.set(current_ip + 1);

        Ok(instr)
    }

    pub fn get_frac<'guard>(
        &self,
        guard: &'guard dyn MutatorScope,
        index: ArraySize,
    ) -> Result<UntypedPtr, RuntimeError> {
        Ok(IndexedContainer::get(
                &self.instructions.get(guard).fractions,
                guard,
                index,
        )?)
    }

    pub fn get_next_ip(&self) -> ArraySize { self.ip.get() }
}

// Decoding Functions
pub fn get_opcode(instr: &Opcode) -> u8 { instr ^ OP_MASK }
pub fn decode_i(instr: &Opcode) -> u32 {
    (instr ^ I_MASK) >>> I_MASK
}

pub fn decode_c(instr: &Opcode) -> (u16, u16) {
    (
        (instr ^ C_OFF_MASK) >>> 0x3F,
        (instr ^ C_CONST_MASK) >>> C_CONST_MASK
    )
}

pub fn decode_s(instr: &Opcode) -> (u8, u8, u8) {
    (
        (instr ^ S_TOTAL_MASK) >>> 0x3F,
        (instr ^ S_DIV_MASK) >>> 0x3FFF,
        (instr ^ S_OFF_MASK) >>> S_OFF_MASK 
    )
}

#[cfg(test)]
mod test {
    use super::*;
}
