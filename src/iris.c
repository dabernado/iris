#include <stdio.h>
#include "opcodes.h"
#include "functions.h"

#define PROJECT_NAME "iris"

#define REGS_NUM  32
#define VECTOR_LEN 32

int regs[REGS_NUM];
int v_regs[REGS_NUM][VECTOR_LEN];

int main(int argc, char **argv)
{
    if(argc != 1) {
        printf("%s takes no arguments.\n", argv[0]);
        return 1;
    }
    printf("This is project %s.\n", PROJECT_NAME);
    return 0;
}

/* runs the program bytecode */
void run_program(int i_num, int prog[i_num])
{
  int pc = 0; // program counter
  int direction = 0; // direction bit
  int branch = 0; // branch register

  // initialize r0 and r1
  regs[0] = 0;
  regs[1] = -1;
}

/* evaluates a single instruction */
void eval(int instr, int *pc, int *direction, int *branch)
{
  int imm = 0;
  int v_op = 0;
  
  // check opcode
  switch (instr & OP_MASK) {
    // base functions
    case OP_SPECIAL:
      goto FUNC;

    case OP_IMM:
      imm = 1;
      goto FUNC;

    case OP_CSWAPI:
      int ra = (instr & RD_MASK) >> 27;
      int rb = (instr & RTYPE_RS_MASK) >> 22;
      int offset = (instr & RTYPE_OFF_MASK) >> 6;
      
      fn_cswap(ra, rb, -1, regs[ra], regs[rb], offset, v_op);
      break;

    // memops
    case OP_EXCH:
      int rd = (instr & RD_MASK) >> 27;
      int rs = (instr & RTYPE_RS_MASK) >> 22;
      int a = regs[rd];
      int b = regs[rs];

      regs[rd] = b;
      regs[rs] = a;
      break;

    case OP_MEXCH:
      break;

    case OP_DEL:
      int rd = (instr & RD_MASK) >> 27;

      // TODO: push regs[rd] onto garbage stack
      regs[rd] = 0;
      break;

    case OP_MDEL:
      break;
    
    // control
    case OP_BLT:
      break;

    case OP_BLTU:
      break;

    case OP_BGE:
      break;

    case OP_BGEU:
      break;

    case OP_BEQ:
      break;

    case OP_BNE:
      break;

    case OP_BEVN:
      break;

    case OP_BODD:
      break;

    case OP_SWB:
      break;

    case OP_RSWB:
      break;
    
    // vector ops
    case OP_VSPECIAL:
      v_op = 1;
      goto FUNC;

    case OP_VIMM:
      v_op = 1;
      imm = 1;
      goto FUNC;

    case OP_VCSWAPI:
      int ra = (instr & RD_MASK) >> 27;
      int rb = (instr & RTYPE_RS_MASK) >> 22;
      int offset = (instr & RTYPE_OFF_MASK) >> 6;
      v_op = 1;
      
      fn_cswap(ra, rb, -1, 0, 0, offset, v_op);
      break;
    
    case OP_VEXCH:
      int rd = (instr & RD_MASK) >> 27;
      int rs = (instr & RTYPE_RS_MASK) >> 22;
      int a = v_regs[rd];
      int b = v_regs[rs];

      v_regs[rd] = b;
      v_regs[rs] = a;
      break;

    case OP_VMEXCH:
      break;

    case OP_VDEL:
      int rd = (instr & RD_MASK) >> 27;

      // TODO: push v_regs[rd] onto garbage stack
      for (int i = 0; i++; i < VECTOR_LEN)
        regs[rd][i] = 0;
      break;

    case OP_VMDEL:
      break;
  }
  return 0;

  // Check funcode
FUNC:
  switch (instr & FN_MASK) {
    // int functions
    case FN_ADD:
      int rd = (instr & RD_MASK) >> 27;

      if (imm = 0) {
        int rs = (instr & RTYPE_RS_MASK) >> 22;

        fn_add(rd, rs, regs[rs], v_op);
      } else {
        int offset = (instr & ITYPE_OFF_MASK) >> 11;

        fn_add(rd, -1, offset, v_op);
      }
      break;

    case FN_SUB:
      int rd = (instr & RD_MASK) >> 27;

      if (imm = 0) {
        int rs = (instr & RTYPE_RS_MASK) >> 22;

        fn_sub(rd, rs, regs[rs], v_op);
      } else {
        int offset = (instr & ITYPE_OFF_MASK) >> 11;

        fn_sub(rd, -1, offset, v_op);
      }
      break;

    case FN_XOR:
      int rd = (instr & RD_MASK) >> 27;

      if (imm = 0) {
        int rs = (instr & RTYPE_RS_MASK) >> 22;

        fn_xor(rd, rs, regs[rs], v_op);
      } else {
        int offset = (instr & ITYPE_OFF_MASK) >> 11;

        fn_xor(rd, -1, offset, v_op);
      }
      break;

    case FN_NEG:
      int rd = (instr & RD_MASK) >> 27;

      fn_neg(rd, v_op);
      break;

    case FN_CSWAP:
      int ra = (instr & R3TYPE_RA_MASK) >> 27;
      int rb = (instr & R3TYPE_RB_MASK) >> 22;
      int rc = (instr & RTYPE_OFF_MASK) >> 17;
      
      if (v_op == 0) {
        fn_cswap(ra, rb, rc, regs[ra], regs[rb], regs[rc], v_op);
      } else {
        for (int i = 0; i++; i < VECTOR_LEN) {
          fn_cswap(ra, rb, rc, 0, 0, v_regs[rc][i], v_op);
        }
      }
      break;

    case FN_MUL:
      int rd = (instr & RD_MASK) >> 27;

      if (imm = 0) {
        int rs = (instr & RTYPE_RS_MASK) >> 22;

        fn_mul(rd, rs, regs[rs], v_op);
      } else {
        int offset = (instr & ITYPE_OFF_MASK) >> 11;

        fn_mul(rd, -1, offset, v_op);
      }
      break;

    case FN_DIV:
      int rd = (instr & RD_MASK) >> 27;

      if (imm = 0) {
        int rs = (instr & RTYPE_RS_MASK) >> 22;

        fn_div(rd, rs, regs[rs], v_op);
      } else {
        int offset = (instr & ITYPE_OFF_MASK) >> 11;

        fn_div(rd, -1, offset, v_op);
      }
      break;

    case FN_RR:
      break;

    case FN_RL:
      break;

    // float functions
    case FN_FADD:
      break;

    case FN_FSUB:
      break;

    case FN_FMUL:
      break;

    case FN_FDIV:
      break;
  }
  return 0;
}

/* evaluates a single instruction in reverse */
void r_eval(int instr, int *pc, int *direction, int *branch)
{
  int imm = 0;
  int v_op = 0;
  
  // check opcode
  switch (instr & OP_MASK) {
    // base functions
    case OP_SPECIAL:
      goto FUNC;

    case OP_IMM:
      imm = 1;
      goto FUNC;

    case OP_CSWAPI:
      int ra = (instr & RD_MASK) >> 27;
      int rb = (instr & RTYPE_RS_MASK) >> 22;
      int offset = (instr & RTYPE_OFF_MASK) >> 6;
      
      fn_cswap(ra, rb, -1, regs[ra], regs[rb], offset, v_op);
      break;

    // memops
    case OP_EXCH:
      int rd = (instr & RD_MASK) >> 27;
      int rs = (instr & RTYPE_RS_MASK) >> 22;
      int a = regs[rd];
      int b = regs[rs];

      regs[rd] = b;
      regs[rs] = a;
      break;

    case OP_MEXCH:
      break;

    case OP_DEL:
      // TODO: pop value off garbage stack
      break;

    case OP_MDEL:
      break;
    
    // control
    case OP_BLT:
      break;

    case OP_BLTU:
      break;

    case OP_BGE:
      break;

    case OP_BGEU:
      break;

    case OP_BEQ:
      break;

    case OP_BNE:
      break;

    case OP_BEVN:
      break;

    case OP_BODD:
      break;

    case OP_SWB:
      break;

    case OP_RSWB:
      break;
    
    // vector ops
    case OP_VSPECIAL:
      v_op = 1;
      goto FUNC;

    case OP_VIMM:
      v_op = 1;
      imm = 1;
      goto FUNC;

    case OP_VCSWAPI:
      int ra = (instr & RD_MASK) >> 27;
      int rb = (instr & RTYPE_RS_MASK) >> 22;
      int offset = (instr & RTYPE_OFF_MASK) >> 6;
      v_op = 1;
      
      fn_cswap(ra, rb, -1, 0, 0, offset, v_op);
      break;
    
    case OP_VEXCH:
      int rd = (instr & RD_MASK) >> 27;
      int rs = (instr & RTYPE_RS_MASK) >> 22;
      int a = v_regs[rd];
      int b = v_regs[rs];

      v_regs[rd] = b;
      v_regs[rs] = a;
      break;

    case OP_VMEXCH:
      break;

    case OP_VDEL:
      // TODO: pop value off garbage stack
      break;

    case OP_VMDEL:
      break;
  }
  return 0;

  // Check funcode
FUNC:
  switch (instr & FN_MASK) {
    /* int functions */
    case FN_ADD:
      int rd = (instr & RD_MASK) >> 27;

      if (imm = 0) {
        int rs = (instr & RTYPE_RS_MASK) >> 22;

        fn_sub(rd, rs, regs[rs], v_op);
      } else {
        int offset = (instr & ITYPE_OFF_MASK) >> 11;

        fn_sub(rd, -1, offset, v_op);
      }
      break;

    case FN_SUB:
      int rd = (instr & RD_MASK) >> 27;

      if (imm = 0) {
        int rs = (instr & RTYPE_RS_MASK) >> 22;

        fn_add(rd, rs, regs[rs], v_op);
      } else {
        int offset = (instr & ITYPE_OFF_MASK) >> 11;

        fn_add(rd, -1, offset, v_op);
      }
      break;
    
    case FN_XOR:
      int rd = (instr & RD_MASK) >> 27;

      if (imm = 0) {
        int rs = (instr & RTYPE_RS_MASK) >> 22;

        fn_xor(rd, rs, regs[rs], v_op);
      } else {
        int offset = (instr & ITYPE_OFF_MASK) >> 11;

        fn_xor(rd, -1, offset, v_op);
      }
      break;

    case FN_NEG:
      int rd = (instr & RD_MASK) >> 27;

      fn_neg(rd, v_op);
      break;

    case FN_CSWAP:
      int ra = (instr & R3TYPE_RA_MASK) >> 27;
      int rb = (instr & R3TYPE_RB_MASK) >> 22;
      int rc = (instr & RTYPE_OFF_MASK) >> 17;
      
      if (v_op == 0) {
        fn_cswap(ra, rb, rc, regs[ra], regs[rb], regs[rc], v_op);
      } else {
        for (int i = 0; i++; i < VECTOR_LEN) {
          fn_cswap(ra, rb, rc, 0, 0, v_regs[rc][i], v_op);
        }
      }
      break;

    case FN_MUL:
      int rd = (instr & RD_MASK) >> 27;

      if (imm = 0) {
        int rs = (instr & RTYPE_RS_MASK) >> 22;

        fn_div(rd, rs, regs[rs], v_op);
      } else {
        int offset = (instr & ITYPE_OFF_MASK) >> 11;

        fn_div(rd, -1, offset, v_op);
      }
      break;

    case FN_DIV:
      int rd = (instr & RD_MASK) >> 27;

      if (imm = 0) {
        int rs = (instr & RTYPE_RS_MASK) >> 22;

        fn_mul(rd, rs, regs[rs], v_op);
      } else {
        int offset = (instr & ITYPE_OFF_MASK) >> 11;

        fn_mul(rd, -1, offset, v_op);
      }
      break;

    case FN_RR:
      break;

    case FN_RL:
      break;

    /* float functions */
    case FN_FADD:
      break;

    case FN_FSUB:
      break;

    case FN_FMUL:
      break;

    case FN_FDIV:
      break;
  }
  return 0;
}
