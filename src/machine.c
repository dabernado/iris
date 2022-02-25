/*
 * machine.c - Iris VM implementation
 *
 */

#include <stdio.h>
#include <stdlib.h>
#include "machine.h"
#include "opcodes.h"
#include "functions.h"

#define BIT_WIDTH 32

#define STATUS_OK  0
#define STATUS_ACC 1
#define STATUS_ERR 2

/* executes IRIS bytecode on a new VM */
void *init_vm(int *prog, int prog_size, int mb)
{
  int *pc = NULL; // program counter
  int direction = 0; // direction bit
  int branch = 0; // branch register
  int mem_size = (mb * 1000000) / (BIT_WIDTH / 8);

  // if prog_size > 7/8th of mem_size, exit the function
  if (prog_size > ((mem_size / 8) * 7))
    return NULL;

  // initialize registers
  int regs[REGS_NUM];
  regs[0] = 0;
  
  //int v_regs[REGS_NUM][VREG_LEN];
  //int vlen = 0;

  // initialize memory + garbage stack to last 1/8th of memory
  int *memory = malloc(sizeof(int) * mem_size);
  int *garbage = memory + (mem_size - (mem_size / 8));
  int *garbage_start = garbage;

  // load the program into memory
  for (int i = 0; i < prog_size; i++)
    *(memory + i) = *(prog + i);
  pc = memory;

  // execution loop
  int status = STATUS_OK;
  while (status == STATUS_OK)
  {
    if (direction == 0)
    {
      eval(
          regs,
          memory, garbage,
          &direction, &branch,
          (*pc));
    } else
    {
      r_eval(
          regs,
          memory, garbage,
          &direction, &branch,
          (*pc));
    }

    // increment program counter
    if (branch != 0)
    {
      pc += branch;
    }
    else pc += 1;

    // check if garbage pointer is within bounds
    if ((garbage > garbage_start) && (garbage < (memory + mem_size)))
    {
      printf("ERROR: Garbage stack is full");
      status = STATUS_ERR;
    }
  }

  return memory;
}

/* evaluates a single instruction */
void eval(
    int regs[REGS_NUM],
    int *m_regs, int *garbage,
    int *direction, int *branch,
    int instr)
{
  int imm = 0;

  // R-Type defaults
  int rd = (instr & RD_MASK) >> 27;
  int rs = (instr & RTYPE_RS_MASK) >> 22;
  int offset = (instr & RTYPE_OFF_MASK) >> 6;

  // exchange values
  int a = 0;
  int b = 0;
  
  // check opcode
  switch (instr & OP_MASK) {
    // base functions
    case OP_SPECIAL:
      goto FUNC;

    case OP_IMM:
      imm = 1;
      goto FUNC;

    case OP_CSWAPI:
      fn_cswap(regs, rs, rd, offset);
      break;

    // memops
    case OP_EXCH:
      a = regs[rd];
      b = regs[rs];

      regs[rd] = b;
      regs[rs] = a;
      break;

    case OP_MEXCH:
      a = regs[rs] + offset; // memory address
      b = regs[rd];

      regs[rd] = *(m_regs + a);
      *(m_regs + a) = b;
      break;

    case OP_DEL:
      a = regs[rd];

      // push regs[rd] onto garbage stack
      regs[rd] = 0;
      (*garbage) = a;
      garbage++;
      break;

    case OP_MDEL:
      offset = (instr & ITYPE_OFF_MASK) >> 11;
      a = regs[rd] + offset;
      b = *(m_regs + a);

      // push mem value onto garbage stack
      *(m_regs + a) = 0;
      (*garbage) = b;
      garbage++;
      break;
    
    // control
    case OP_BLTU:
    case OP_BLT:
      if (regs[rd] < regs[rs])
        (*branch) += offset;
      break;

    case OP_BGEU:
    case OP_BGE:
      if (regs[rd] >= regs[rs])
        (*branch) += offset;
      break;

    case OP_BEQ:
      if (regs[rd] == regs[rs])
        (*branch) += offset;
      break;

    case OP_BNE:
      if (regs[rd] != regs[rs])
        (*branch) += offset;
      break;

    case OP_BEVN:
      offset = (instr & ITYPE_OFF_MASK) >> 11;

      if ((regs[rd] % 2) == 0)
        (*branch) += offset;
      break;

    case OP_BODD:
      offset = (instr & ITYPE_OFF_MASK) >> 11;

      if ((regs[rd] % 2) == 1)
        (*branch) += offset;
      break;

    case OP_SWB:
      a = (*branch);

      (*branch) = regs[rd];
      regs[rd] = a;
      break;

    case OP_RSWB:
      a = (*branch);

      (*branch) = regs[rd];
      regs[rd] = a;
      (*direction) = ~(*direction);
      break;
  }

  // Check funcode
FUNC:
  switch (instr & FN_MASK) {
    // int functions
    case FN_ADD:
      if (imm == 0) {
        fn_add(regs, rd, regs[rs]);
      } else {
        offset = (instr & ITYPE_OFF_MASK) >> 11;
        fn_add(regs, rd, offset);
      }
      break;

    case FN_SUB:
      if (imm == 0) {
        fn_sub(regs, rd, regs[rs]);
      } else {
        offset = (instr & ITYPE_OFF_MASK) >> 11;
        fn_sub(regs, rd, offset);
      }
      break;

    case FN_XOR:
      if (imm == 0) {
        fn_xor(regs, rd, regs[rs]);
      } else {
        offset = (instr & ITYPE_OFF_MASK) >> 11;
        fn_xor(regs, rd, offset);
      }
      break;

    case FN_NEG:
      fn_neg(regs, rd);
      break;

    case FN_CSWAP:
      rd = (instr & R3TYPE_RA_MASK) >> 27; // ra
      rs = (instr & R3TYPE_RB_MASK) >> 22; // rb
      offset = (instr & R3TYPE_RC_MASK) >> 17; // rc
      fn_cswap(regs, rd, rs, regs[offset]);
      break;

    case FN_MUL:
      if (imm == 0) {
        fn_mul(regs, rd, regs[rs]);
      } else {
        offset = (instr & ITYPE_OFF_MASK) >> 11;
        fn_mul(regs, rd, offset);
      }
      break;

    case FN_DIV:
      if (imm == 0) {
        fn_div(regs, rd, regs[rs]);
      } else {
        offset = (instr & ITYPE_OFF_MASK) >> 11;
        fn_div(regs, rd, offset);
      }
      break;

    case FN_RR:
      if (imm == 0) {
        fn_rr(regs, rd, regs[rs]);
      } else {
        offset = (instr & ITYPE_OFF_MASK) >> 11;
        fn_rr(regs, rd, offset);
      }
      break;

    case FN_RL:
      if (imm == 0) {
        fn_rl(regs, rd, regs[rs]);
      } else {
        offset = (instr & ITYPE_OFF_MASK) >> 11;
        fn_rl(regs, rd, offset);
      }
      break;

    // float functions
    case FN_FADD:
      if (imm == 0) {
        fn_fadd(regs, rd, (float)regs[rs]);
      } else {
        offset = (instr & ITYPE_OFF_MASK) >> 11;
        fn_fadd(regs, rd, (float)offset);
      }
      break;

    case FN_FSUB:
      if (imm == 0) {
        fn_fsub(regs, rd, (float)regs[rs]);
      } else {
        offset = (instr & ITYPE_OFF_MASK) >> 11;
        fn_fsub(regs, rd, (float)offset);
      }
      break;

    case FN_FMUL:
      if (imm == 0) {
        fn_fmul(regs, rd, (float)regs[rs]);
      } else {
        offset = (instr & ITYPE_OFF_MASK) >> 11;
        fn_fmul(regs, rd, (float)offset);
      }
      break;

    case FN_FDIV:
      if (imm == 0) {
        fn_fdiv(regs, rd, (float)regs[rs]);
      } else {
        offset = (instr & ITYPE_OFF_MASK) >> 11;
        fn_fdiv(regs, rd, (float)offset);
      }
      break;
  }
}

/* evaluates a single instruction in reverse */
void r_eval(
    int regs[REGS_NUM],
    int *m_regs, int *garbage,
    int *direction, int *branch,
    int instr)
{
  int imm = 0;

  // R-Type defaults
  int rd = (instr & RD_MASK) >> 27;
  int rs = (instr & RTYPE_RS_MASK) >> 22;
  int offset = (instr & RTYPE_OFF_MASK) >> 6;

  // exchange values
  int a = 0;
  int b = 0;
  
  // check opcode
  switch (instr & OP_MASK) {
    // base functions
    case OP_SPECIAL:
      goto FUNC;

    case OP_IMM:
      imm = 1;
      goto FUNC;

    case OP_CSWAPI:
      fn_cswap(regs, rs, rd, offset);
      break;

    // memops
    case OP_EXCH:
      a = regs[rd];
      b = regs[rs];

      regs[rd] = b;
      regs[rs] = a;
      break;

    case OP_MEXCH:
      a = regs[rs] + offset; // memory address
      b = regs[rd];

      regs[rd] = *(m_regs + a);
      *(m_regs + a) = b;
      break;

    case OP_DEL:
      a = (*garbage);

      // push regs[rd] onto garbage stack
      regs[rd] = a;
      (*garbage) = 0;
      garbage--;
      break;

    case OP_MDEL:
      offset = (instr & ITYPE_OFF_MASK) >> 11;
      a = regs[rd] + offset;
      b = (*garbage);

      // push mem value onto garbage stack
      *(m_regs + a) = b;
      (*garbage) = 0;
      garbage--;
      break;
    
    // control
    case OP_BLTU:
    case OP_BLT:
      if (regs[rd] < regs[rs])
        (*branch) -= offset;
      break;

    case OP_BGEU:
    case OP_BGE:
      if (regs[rd] >= regs[rs])
        (*branch) -= offset;
      break;

    case OP_BEQ:
      if (regs[rd] == regs[rs])
        (*branch) -= offset;
      break;

    case OP_BNE:
      if (regs[rd] != regs[rs])
        (*branch) -= offset;
      break;

    case OP_BEVN:
      offset = (instr & ITYPE_OFF_MASK) >> 11;

      if ((regs[rd] % 2) == 0)
        (*branch) -= offset;
      break;

    case OP_BODD:
      offset = (instr & ITYPE_OFF_MASK) >> 11;

      if ((regs[rd] % 2) == 1)
        (*branch) -= offset;
      break;

    case OP_SWB:
      a = (*branch);

      (*branch) = regs[rd];
      regs[rd] = a;
      break;

    case OP_RSWB:
      a = (*branch);

      (*branch) = regs[rd];
      regs[rd] = a;
      (*direction) = ~(*direction);
      break;
  }

  // Check funcode
FUNC:
  switch (instr & FN_MASK) {
    // int functions
    case FN_ADD:
      if (imm == 0) {
        fn_sub(regs, rd, regs[rs]);
      } else {
        offset = (instr & ITYPE_OFF_MASK) >> 11;
        fn_sub(regs, rd, offset);
      }
      break;

    case FN_SUB:
      if (imm == 0) {
        fn_add(regs, rd, regs[rs]);
      } else {
        offset = (instr & ITYPE_OFF_MASK) >> 11;
        fn_add(regs, rd, offset);
      }
      break;

    case FN_XOR:
      if (imm == 0) {
        fn_xor(regs, rd, regs[rs]);
      } else {
        offset = (instr & ITYPE_OFF_MASK) >> 11;
        fn_xor(regs, rd, offset);
      }
      break;

    case FN_NEG:
      fn_neg(regs, rd);
      break;

    case FN_CSWAP:
      rd = (instr & R3TYPE_RA_MASK) >> 27; // ra
      rs = (instr & R3TYPE_RB_MASK) >> 22; // rb
      offset = (instr & R3TYPE_RC_MASK) >> 17; // rc
      fn_cswap(regs, rd, rs, regs[offset]);
      break;

    case FN_MUL:
      if (imm == 0) {
        fn_div(regs, rd, regs[rs]);
      } else {
        offset = (instr & ITYPE_OFF_MASK) >> 11;
        fn_div(regs, rd, offset);
      }
      break;

    case FN_DIV:
      if (imm == 0) {
        fn_mul(regs, rd, regs[rs]);
      } else {
        offset = (instr & ITYPE_OFF_MASK) >> 11;
        fn_mul(regs, rd, offset);
      }
      break;

    case FN_RR:
      if (imm == 0) {
        fn_rl(regs, rd, regs[rs]);
      } else {
        offset = (instr & ITYPE_OFF_MASK) >> 11;
        fn_rl(regs, rd, offset);
      }
      break;

    case FN_RL:
      if (imm == 0) {
        fn_rr(regs, rd, regs[rs]);
      } else {
        offset = (instr & ITYPE_OFF_MASK) >> 11;
        fn_rr(regs, rd, offset);
      }
      break;

    // float functions
    case FN_FADD:
      if (imm == 0) {
        fn_fsub(regs, rd, (float)regs[rs]);
      } else {
        offset = (instr & ITYPE_OFF_MASK) >> 11;
        fn_fsub(regs, rd, (float)offset);
      }
      break;

    case FN_FSUB:
      if (imm == 0) {
        fn_fadd(regs, rd, (float)regs[rs]);
      } else {
        offset = (instr & ITYPE_OFF_MASK) >> 11;
        fn_fadd(regs, rd, (float)offset);
      }
      break;

    case FN_FMUL:
      if (imm == 0) {
        fn_fdiv(regs, rd, (float)regs[rs]);
      } else {
        offset = (instr & ITYPE_OFF_MASK) >> 11;
        fn_fdiv(regs, rd, (float)offset);
      }
      break;

    case FN_FDIV:
      if (imm == 0) {
        fn_fmul(regs, rd, (float)regs[rs]);
      } else {
        offset = (instr & ITYPE_OFF_MASK) >> 11;
        fn_fmul(regs, rd, (float)offset);
      }
      break;
  }
}
