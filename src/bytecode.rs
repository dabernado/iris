use std::cell::Cell;

use crate::alloc::api::AllocObject;
use crate::array::{
    Array,
    ArraySize,
    Container, IndexedContainer, StackContainer
};
use crate::constants::*;
use crate::data::Fraction;
use crate::error::{RuntimeError, ErrorKind};
use crate::memory::{MutatorView, MutatorScope};
use crate::safeptr::{CellPtr, ScopedPtr, FuncPtr};

pub type Opcode = u32;

#[derive(Clone)]
pub struct Function {
    fractions: Array<Fraction>,
    code: Array<Opcode>,
}
impl AllocObject for Function {}

impl Function {
    pub fn alloc<'guard>(
        mem: &'guard MutatorView,
    ) -> Result<ScopedPtr<'guard, Function>, RuntimeError> {
        mem.alloc(Function {
            fractions: Array::<Fraction>::new(),
            code: Array::<Opcode>::new(),
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
        ptr: Fraction
    ) -> Result<(), RuntimeError> { self.fractions.push(mem, ptr) }

    pub fn length(&self) -> ArraySize { self.code.length() }
}

#[derive(Clone)]
pub struct Continuation {
    function: FuncPtr,
    ip: Cell<ArraySize>,
    direction: Cell<bool>,
}
impl AllocObject for Continuation {}

impl Continuation {
    pub fn alloc<'guard>(
        mem: &'guard MutatorView,
        func: ScopedPtr<'_, Function>,
    ) -> Result<ScopedPtr<'guard, Continuation>, RuntimeError> {
        mem.alloc(Continuation {
            function: CellPtr::new_with(func),
            ip: Cell::new(0),
            direction: Cell::new(false),
        })
    }

    pub fn switch_frame(
        &self,
        code: ScopedPtr<'_, Function>,
        ip: ArraySize,
        dir: bool,
    ) {
        self.function.set(code);
        self.ip.set(ip);
        self.direction.set(dir);
    }

    // TODO: Optimize (too many indirections)
    pub fn get_next_op<'guard>(
        &self,
        guard: &'guard dyn MutatorScope,
    ) -> Result<Opcode, RuntimeError> {
        let current_ip = self.ip.get();
        let instr = self
            .function
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
    ) -> Result<&'guard Fraction, RuntimeError> {
        Ok(self.function.get(guard).fractions.read_ref(guard, index)?)
    }

    pub fn set_ip(&self, i: ArraySize) { self.ip.set(i); }
    pub fn jump(&self, jmp: ArraySize) {
        if !self.direction() {
            self.set_ip(self.ip() + jmp);
        } else {
            self.set_ip(self.ip() - jmp);
        }
    }

    pub fn set_func<'guard>(&self, func: ScopedPtr<'guard, Function>) {
        self.function.set(func);
    }

    pub fn reset(&self, jmp: ArraySize) {
        if !self.direction() {
            self.ip.set(0);
        } else {
            self.ip.set(jmp);
        }
    }

    pub fn ip(&self) -> ArraySize { self.ip.get() }
    pub fn direction(&self) -> bool { self.direction.get() }
    pub fn reverse(&self) { self.direction.set(!self.direction()) }
    pub fn current_func<'guard>(
        &self,
        guard: &'guard dyn MutatorScope,
    ) -> ScopedPtr<'guard, Function> { self.function.get(guard) }
}

// Decoding Functions
pub fn get_opcode(instr: Opcode) -> u8 {
    (instr & OP_MASK) as u8
}
pub fn decode_i(instr: Opcode) -> u32 {
    (instr & I_MASK) >> 6
}

pub fn decode_s(instr: Opcode) -> (u8, u8, u8) {
    (
        ((instr & S_DIV_MASK) >> 6) as u8,
        ((instr & S_LC_MASK) >> 14) as u8,
        ((instr & S_RC_MASK) >> 22) as u8
    )
}

// Encoding Functions
pub fn encode_i(op: u8, imm: u32) -> Result<Opcode, RuntimeError> {
    // check if within bounds
    if imm <= MAX_ITYPE_FIELD {
        Ok((imm << 6) ^ (op as u32))
    } else {
        Err(RuntimeError::new(ErrorKind::IntOverflow))
    }
}

pub fn encode_s(op: u8, div: u8, lc: u8, rc: u8) -> Opcode {
    let padded_div = (div as u32) << 6;
    let padded_lc = (lc as u32) << 14;
    let padded_rc = (rc as u32) << 22;

    (((0 ^ padded_rc)
      ^ padded_lc)
     ^ padded_div)
    ^ (op as u32)
}

#[cfg(test)]
mod test {
    use super::*;

    // Encoding/Decoding Tests
    #[test]
    fn test_get_opcode() {
        assert!(OP_ADD == get_opcode(10));
    }

    #[test]
    fn test_itype() {
        let instr = encode_i(OP_ADDI, 2).unwrap();

        assert!(OP_ADDI == get_opcode(instr));
        assert!(2 == decode_i(instr));
    }

    #[test]
    fn test_stype() {
        let instr = encode_s(OP_SUMS, 4, 2, 3);

        assert!(OP_SUMS == get_opcode(instr));
        assert!((4, 2, 3) == decode_s(instr));
    }
}
