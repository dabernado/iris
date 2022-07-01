use crate::alloc::api::{AllocObject, ITypeId};

pub enum Opcode {}

impl AllocObject<ITypeId> for Opcode {
    const TYPE_ID: ITypeId = ITypeId::Opcode;
}
