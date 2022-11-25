use iris::bytecode::*;
use iris::constants::*;

#[test]
fn test_get_opcode() {
    assert!(OP_ID == get_opcode(OP_ID as u32, false)
            && OP_ID_R == get_opcode(OP_ID as u32, true));
    assert!(OP_ZEROE == get_opcode(OP_ZEROE as u32, false)
            && OP_ZEROI == get_opcode(OP_ZEROE as u32, true));
    assert!(OP_FOLD == get_opcode(OP_FOLD as u32, false)
            && OP_UFOLD == get_opcode(OP_FOLD as u32, true));
    assert!(OP_READ == get_opcode(OP_READ as u32, false)
            && OP_WRITE == get_opcode(OP_READ as u32, true));
}

#[test]
fn test_itype() {
    let instr = encode_i(OP_EXPN, 2).unwrap();

    assert!(OP_EXPN == get_opcode(instr, false));
    assert!(2 == decode_i(instr));
}

#[test]
fn test_stype() {
    let instr = encode_s(OP_SWAPS, 6, 9).unwrap();

    assert!(OP_SWAPS == get_opcode(instr, false));
    assert!((6, 9) == decode_s(instr));
}
