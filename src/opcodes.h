/*
 * opcodes.h - Definition of IRIS opcodes
 *
 */

#ifndef IRIS_OPS
#define IRIS_OPS

/* R-Type
 *
 * 31    27 26    22 21                    6 5        0
 * [  rd  ] [  rs  ] [     func/imm/off    ] [ opcode ]
 *
 * R-Type instructions specify a destination register
 * and a source register to perform an op on, specified
 * by a 6-bit field and an additional 16-bit field which is
 * used as a function, immediate or offset field, depending
 * on the operation
 */

#define OP_SPECIAL  0x00
#define OP_EXCH     0x02
#define OP_MEXCH    0x04
#define OP_CSWAPI   0x08

#define OP_BLT      0x11
#define OP_BLTU     0x13
#define OP_BGE      0x12
#define OP_BGEU     0x16
#define OP_BEQ      0x14
#define OP_BNE      0x15

#define OP_VSPECIAL 0x20
#define OP_VEXCH    0x22
#define OP_VMEXCH   0x24

// functions
#define FN_ADD   0x01
#define FN_SUB   0x03
#define FN_XOR   0x05
#define FN_NEG   0x06
#define FN_MUL   0x09
#define FN_DIV   0x0b
#define FN_RR    0x0d
#define FN_RL    0x0e

#define FN_FADD  0x41
#define FN_FSUB  0x43
#define FN_FMUL  0x49
#define FN_FDIV  0x4b

/* R3-Type
 *
 * 31    27 26    22 21    17 16           6 5        0
 * [  ra  ] [  rb  ] [  rc  ] [   func11   ] [ opcode ]
 *
 * R3-Type instructions specify three registers to operate
 * on, with a 11-bit function field. CSWAP is the only
 * instruction of this format
 */

#define FN_CSWAP   0x07

/* I-Type
 *
 * 31    27 26                   11 10     6 5        0
 * [  rd  ] [       imm/off       ] [ func ] [ opcode ]
 *
 * I-Type instructions specify a destination register
 * and a 16-bit immediate value as operands, for the
 * op specified by the 5-bit function and 6-bit opcode fields
 */

#define OP_IMM   0x01
#define OP_DEL   0x05

#define OP_SWB   0x06
#define OP_RSWB  0x07
#define OP_BEVN  0x18
#define OP_BODD  0x19

#define OP_VIMM  0x21
#define OP_VDEL  0x25

// functions
#define FN_ADDI  0x01
#define FN_SUBI  0x03
#define FN_XORI  0x05
#define FN_NEGI  0x06
#define FN_MULI  0x09
#define FN_DIVI  0x0b
#define FN_RRI   0x0d
#define FN_RLI   0x0e

#define FN_FADDI 0x11
#define FN_FSUBI 0x13
#define FN_FMULI 0x19
#define FN_FDIVI 0x1b

#endif
