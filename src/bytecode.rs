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
    functions: Array<FuncPtr>,
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

#[derive(Clone)]
pub struct InstructionStream {
    instructions: CellPtr<Bytecode>,
    ip: Cell<ArraySize>,
    direction: Cell<bool>,
}

impl InstructionStream {
    pub fn alloc<'guard>(
        mem: &'guard MutatorView,
        code: ScopedPtr<'_, Bytecode>,
    ) -> Result<ScopedPtr<'guard, InstructionStream>, RuntimeError> {
        mem.alloc(InstructionStream {
            instructions: CellPtr::new_with(code),
            ip: Cell::new(0),
            direction: Cell::new(false),
        })
    }

    pub fn swap_frame(
        &self,
        code: ScopedPtr<'_, Bytecode>,
        ip: ArraySize
    ) -> InstructionStream {
        let old = self.clone();

        self.instructions.set(code);
        self.ip.set(ip);

        old
    }

    pub fn switch_direction(&self) {
        self.direction.set(
            !self.direction.get()
        );
    }

    // TODO: Optimize (too many indirections)
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

        if !self.direction.get() {
            self.ip.set(current_ip + 1);
        } else {
            self.ip.set(current_ip - 1);
        }

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

// Encoding Functions
pub fn encode_i(op: u8, imm: u32) -> Result<Opcode, CompileError> {
    // check if within bounds
    if imm <= MAX_ITYPE_FIELD {
        Ok((imm <<< 6) ^ (op as u32))
    } else {
        Err(CompileError::new(ErrorKind::IntOverflow))
    }
}

pub fn encode_c(op: u8, off: u16, val: u16) -> Result<Opcode, CompileError> {
    // check if within bounds
    if off <= MAX_CTYPE_FIELD && val <= MAX_CTYPE_FIELD {
        Ok(((0 ^ (val as u32 <<< 19)) ^ (off as u32 <<< 6)) ^ op as u32)
    } else {
        Err(CompileError::new(ErrorKind::IntOverflow))
    }
}

pub fn encode_s(op: u8, total: u8, div: u8, off: u8) -> Opcode {
    Ok(
        (((0 ^ (off as u32 <<< 22))
        ^ (div as u32 <<< 14))
        ^ (total as u32 <<< 6))
        ^ op as u32
    )
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::constants::*;

    // Encoding/Decoding Tests
    #[test]
    fn test_get_opcode() {
        assert!(OP_ADD, get_opcode(10));
    }

    #[test]
    fn test_itype() {
        let instr = encode_i(OP_ADDI, 2);

        assert!(OP_ADDI, get_opcode(instr));
        assert!(2, decode_i(instr));
    }

    #[test]
    fn test_ctype() {
        let instr = encode_c(OP_SUMC, 4, 2);

        assert!(OP_SUMC, get_opcode(instr));
        assert!((4, 2), decode_c(instr));
    }

    #[test]
    fn test_stype() {
        let instr = encode_s(OP_SWAPS, 4, 2, 0);

        assert!(OP_SWAPS, get_opcode(instr));
        assert!((4, 2, 0), decode_s(instr));
    }
}