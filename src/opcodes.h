/*
 * opcodes.h - Definition of IRIS opcodes
 *
 */

#ifndef IRIS_OPS
#define IRIS_OPS

/* R-Type
 *
 * 31    27 26    22 21                 7 6          0
 * [  rd  ] [  rs  ] [     func15       ] [  opcode  ]
 *
 * R-Type instructions specify a destination register
 * and a source register to perform an op on, specified
 * by a 7-bit field and an additional 15-bit function field
 */

#define OP_SPECIAL  0x00
#define OP_EXCH     0x02

#define OP_VSPECIAL 0x40
#define OP_VEXCH    0x42

// functions
#define FN_ADD   0x01
#define FN_SUB   0x03
#define FN_XOR   0x05
#define FN_NEG   0x06
#define FN_MUL   0x09
#define FN_DIV   0x0b
#define FN_RR    0x0d
#define FN_RL    0x0e

#define FN_VADD  0x41
#define FN_VSUB  0x43
#define FN_VXOR  0x45
#define FN_VNEG  0x46
#define FN_VMUL  0x49
#define FN_VDIV  0x4b
#define FN_VRR   0x4d
#define FN_VRL   0x4e

/* R3-Type
 *
 * 31    27 26    22 21    17 16        7 6          0
 * [  ra  ] [  rb  ] [  rc  ] [ func10  ] [  opcode  ]
 *
 * R3-Type instructions specify three registers to operate
 * on, with a 10-bit function field. FEY is the only
 * instruction of this format
 */

#define FN_FEY   0x07

/* I-Type
 *
 * 31    27 26                11 10     7 6          0
 * [  rd  ] [     imm/off      ] [ func ] [  opcode  ]
 *
 * I-Type instructions specify a destination register
 * and a 16-bit immediate value as operands, for the
 * op specified by the function and opcode fields
 */

#define OP_IMM   0x01
#define OP_DEL   0x04
#define OP_SWB   0x06
#define OP_RSWB  0x07
#define OP_BLT   0x11
#define OP_BLTU  0x13
#define OP_BGE   0x12
#define OP_BGEU  0x16
#define OP_BEQ   0x14
#define OP_BNE   0x15
#define OP_BEVN  0x18
#define OP_BODD  0x19

#define OP_VIMM  0x41
#define OP_VDEL  0x44

// functions
#define FN_ADDI  0x01
#define FN_SUBI  0x03
#define FN_XORI  0x05
#define FN_NEGI  0x06
#define FN_MULI  0x09
#define FN_DIVI  0x0b
#define FN_RRI   0x0d
#define FN_RLI   0x0e

#define FN_VADDI 0x41
#define FN_VSUBI 0x43
#define FN_VXORI 0x45
#define FN_VNEGI 0x46
#define FN_VMULI 0x49
#define FN_VDIVI 0x4b
#define FN_VRRI  0x4d
#define FN_VRLI  0x4e

/* B-Type
 *
 * 31    27 26     22 21                 6 5         0
 * [  ra  ] [  rb   ] [     imm/off      ] [ opcode  ]
 *
 * B-Type instructions specify two registers and a
 * 16-bit immediate value as operands, with no function
 * field and a 6-bit opcode field. FEYI is the only
 * non-branch instruction of this format
 */

#define OP_FEYI  0x09
#define OP_VFEYI 0x49

#endif
