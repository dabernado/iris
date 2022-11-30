use std::cell::Cell;

use crate::alloc::api::AllocObject;
use crate::array::{Array, ArraySize, IndexedContainer};
use crate::constants::*;
use crate::data::{Nat, Product, Sum};
use crate::error::{RuntimeError, ErrorKind};
use crate::memory::{MutatorScope, MutatorView};
use crate::safeptr::ScopedPtr;

/*
 * Iris Datatypes
 */
pub type Opcode = Nat;
pub type Instruction<O: AllocObject> = Product<Opcode, Sum<O>>;
pub type Function = Array<Instruction<()>>;

#[derive(Clone)]
pub struct Continuation {
    ip: Cell<ArraySize>,
    direction: Cell<bool>,
}
impl AllocObject for Continuation {}

impl Continuation {
    pub fn alloc<'guard>(
        mem: &'guard MutatorView,
    ) -> Result<ScopedPtr<'guard, Continuation>, RuntimeError> {
        mem.alloc(Continuation {
            ip: Cell::new(0),
            direction: Cell::new(false),
        })
    }

    pub fn fetch_instr<'guard>(
        &self,
        guard: &'guard dyn MutatorScope,
        func: ScopedPtr<'guard, Function>
    ) -> Result<Instruction, RuntimeError> {
        func.get(guard, self.ip)
    }

    pub fn set_ip(&self, i: ArraySize) { self.ip.set(i); }
    pub fn jump(&self, jmp: ArraySize) {
        if !self.direction() {
            self.set_ip(self.ip() + jmp);
        } else {
            self.set_ip(self.ip() - jmp);
        }
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
}

// Decoding Functions
pub fn get_opcode(instr: Opcode, dir: bool) -> u8 {
    if !dir {
        (instr & OP_MASK) as u8
    } else {
        (!instr & OP_MASK) as u8
    }
}

pub fn decode_i(instr: Opcode) -> u32 {
    (instr & I_MASK) >> 5
}

pub fn decode_s(instr: Opcode) -> (u16, u16) {
    (
        ((instr & S_LC_MASK) >> 5) as u16,
        ((instr & S_RC_MASK) >> 18) as u16
    )
}

// Encoding Functions
pub fn encode_i(op: u8, imm: u32) -> Result<Opcode, RuntimeError> {
    // check if within bounds
    if imm <= MAX_ITYPE_FIELD {
        Ok((imm << 5) ^ (op as u32))
    } else {
        Err(RuntimeError::new(ErrorKind::IntOverflow))
    }
}

pub fn encode_s(op: u8, lc: u16, rc: u16) -> Result<Opcode, RuntimeError> {
    // check if within bounds
    if lc <= MAX_CTYPE_FIELD && rc <= MAX_CTYPE_FIELD {
        let padded_rc = (rc as u32) << 18;
        let padded_lc = (lc as u32) << 5;

        Ok(((0 ^ padded_rc) ^ padded_lc) ^ (op as u32))
    } else {
        Err(RuntimeError::new(ErrorKind::IntOverflow))
    }
}
