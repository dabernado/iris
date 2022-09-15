use iris::bytecode::*;
use iris::constants::*;

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
