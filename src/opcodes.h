/*
 * opcodes.h - Definition of IRIS opcodes
 *
 */

#ifndef IRIS_OPS
#define IRIS_OPS

#define REGS_NUM  32
#define VREG_LEN  32

// opcode and funcode bitmasks
#define OP_MASK 0x0000003f
#define FN_MASK 0x000007c0
#define RD_MASK 0xf8000000

/* R-Type
 *
 * 31    27 26    22 21                    6 5        0
 * [  rd  ] [  rs  ] [     func/imm/off    ] [ opcode ]
 *
 * R-Type instructions specify a destination register
 * and a source register to perform an op on, specified
 * by a 6-bit field and an additional 16-bit field which is
 * used as a function, immediate or offset field, depending
 * on the opcode
 */

#define RTYPE_OFF_MASK 0x003fffc0
#define RTYPE_RS_MASK  0x07c00000

#define OP_SPECIAL  0x00000000
#define OP_EXCH     0x00000002
#define OP_MEXCH    0x00000004
#define OP_CSWAPI   0x00000008

#define OP_BLT      0x00000011
#define OP_BLTU     0x00000013
#define OP_BGE      0x00000012
#define OP_BGEU     0x00000016
#define OP_BEQ      0x00000014
#define OP_BNE      0x00000015

#define OP_VSPECIAL 0x00000020
#define OP_VEXCH    0x00000022
#define OP_VMEXCH   0x00000024
#define OP_VCSWAPI  0x00000028

/* I-Type
 *
 * 31    27 26                   11 10     6 5        0
 * [  rd  ] [       imm/off       ] [ func ] [ opcode ]
 *
 * I-Type instructions specify a destination register
 * and a 16-bit immediate value as operands, for the
 * op specified by the 5-bit function and 6-bit opcode fields
 */

#define ITYPE_OFF_MASK 0x07fff800

#define OP_IMM   0x00000001
#define OP_DEL   0x00000031
#define OP_MDEL  0x00000032

#define OP_SWB   0x00000006
#define OP_RSWB  0x00000007
#define OP_BEVN  0x00000018
#define OP_BODD  0x00000019

#define OP_VIMM  0x00000021
#define OP_VDEL  0x00000025
#define OP_VCFG  0x00000029
#define OP_VSWL  0x0000002a

/* R3-Type
 *
 * 31    27 26    22 21    17 16           6 5        0
 * [  ra  ] [  rb  ] [  rc  ] [   func11   ] [ opcode ]
 *
 * R3-Type instructions specify three registers to operate
 * on, with a 11-bit function field. CSWAP is the only
 * instruction of this format
 */

#define R3TYPE_RA_MASK  0xf8000000
#define R3TYPE_RB_MASK  0x07c00000
#define R3TYPE_RC_MASK  0x003e0000

/* Functions */
#define FN_ADD   0x00000040
#define FN_SUB   0x000000c0
#define FN_XOR   0x00000140
#define FN_NEG   0x00000180
#define FN_CSWAP 0x000001c0
#define FN_MUL   0x00000240
#define FN_DIV   0x000002c0
#define FN_RR    0x00000340
#define FN_RL    0x00000380

#define FN_FADD  0x00000440
#define FN_FSUB  0x000004c0
#define FN_FMUL  0x00000640
#define FN_FDIV  0x000006c0

#endif
