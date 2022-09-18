use iris::bytecode::*;
use iris::constants::*;

#[test]
fn test_get_opcode() {
    assert!(OP_ID == get_opcode(OP_ID as u32, false)
            && OP_ID_R == get_opcode(OP_ID as u32, true));
    assert!(OP_ZEROE == get_opcode(OP_ZEROE as u32, false)
            && OP_ZEROI == get_opcode(OP_ZEROE as u32, true));
    assert!(OP_ADD == get_opcode(OP_ADD as u32, false)
            && OP_SUB == get_opcode(OP_ADD as u32, true));
    assert!(OP_SYSC == get_opcode(OP_SYSC as u32, false)
            && OP_RSYSC == get_opcode(OP_SYSC as u32, true));
}

#[test]
fn test_itype() {
    let instr = encode_i(OP_ADDI, 2).unwrap();

    assert!(OP_ADDI == get_opcode(instr, false));
    assert!(2 == decode_i(instr));
}

#[test]
fn test_stype() {
    let instr = encode_s(OP_SUMS, 4, 2, 3);

    assert!(OP_SUMS == get_opcode(instr, false));
    assert!((4, 2, 3) == decode_s(instr));
}
