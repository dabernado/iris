/*
 * machine.c - Iris VM implementation
 *
 */

#include "opcodes.h"
#include "functions.h"

#define BIT_WIDTH 32

#define STATUS_OK  0
#define STATUS_ACC 1
#define STATUS_ERR 2

/* executes IRIS bytecode on a new VM */
void *init(int *prog, int prog_size, int mb)
{
  int pc = 0; // program counter
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

  // load the program into memory
  for (int i = 0; i < prog_size, i++)
    *(memory + i) = *(prog_size + i);
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
      pc += branch
    else pc += 1;
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
      
      fn_cswap(regs,
          ra, rb, -1,
          regs[ra], regs[rb], offset);

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
      int rd = (instr & RD_MASK) >> 27;
      int rs = (instr & RTYPE_RS_MASK) >> 22;
      int offset = (instr & RTYPE_OFF_MASK) >> 6;
      
      int addr = regs[rs] + offset;
      int m_val = *(m_regs + addr);
      int r_val = regs[rd];

      regs[rd] = m_val;
      *(m_regs + addr) = r_val;
      break;

    case OP_DEL:
      int rd = (instr & RD_MASK) >> 27;
      int value = regs[rd];

      // push regs[rd] onto garbage stack
      regs[rd] = 0;
      (*garbage) = value;
      garbage++;
      break;

    case OP_MDEL:
      int rd = (instr & RD_MASK) >> 27;
      int offset = (instr & ITYPE_OFF_MASK) >> 11;
      int addr = regs[rd] + offset;
      int m_val = *(m_regs + addr);

      // push mem value onto garbage stack
      *(m_regs + addr) = 0;
      (*garbage) = m_val;
      garbage++;
      break;
    
    // control
    case OP_BLTU:
    case OP_BLT:
      int rd = (instr & RD_MASK) >> 27;
      int rs = (instr & RTYPE_RS_MASK) >> 22;
      int offset = (instr & RTYPE_OFF_MASK) >> 6;

      if (regs[rd] < regs[rs])
        (*branch) += offset;
      break;

    case OP_BGEU:
    case OP_BGE:
      int rd = (instr & RD_MASK) >> 27;
      int rs = (instr & RTYPE_RS_MASK) >> 22;
      int offset = (instr & RTYPE_OFF_MASK) >> 6;

      if (regs[rd] >= regs[rs])
        (*branch) += offset;
      break;

    case OP_BEQ:
      int rd = (instr & RD_MASK) >> 27;
      int rs = (instr & RTYPE_RS_MASK) >> 22;
      int offset = (instr & RTYPE_OFF_MASK) >> 6;

      if (regs[rd] == regs[rs])
        (*branch) += offset;
      break;

    case OP_BNE:
      int rd = (instr & RD_MASK) >> 27;
      int rs = (instr & RTYPE_RS_MASK) >> 22;
      int offset = (instr & RTYPE_OFF_MASK) >> 6;

      if (regs[rd] != regs[rs])
        (*branch) += offset;
      break;

    case OP_BEVN:
      int rd = (instr & RD_MASK) >> 27;
      int offset = (instr & ITYPE_OFF_MASK) >> 11;

      if ((regs[rd] % 2) == 0)
        (*branch) += offset;
      break;

    case OP_BODD:
      int rd = (instr & RD_MASK) >> 27;
      int offset = (instr & ITYPE_OFF_MASK) >> 11;

      if ((regs[rd] % 2) == 1)
        (*branch) += offset;
      break;

    case OP_SWB:
      int rd = (instr & RD_MASK) >> 27;
      int bval = (*branch);

      (*branch) = regs[rd];
      regs[rd] = bval;
      break;

    case OP_RSWB:
      int rd = (instr & RD_MASK) >> 27;
      int bval = (*branch);

      (*branch) = regs[rd];
      regs[rd] = bval;
      (*direction) = ~(*direction);
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
        fn_add(regs, rd, rs, regs[rs]);
      } else {
        int offset = (instr & ITYPE_OFF_MASK) >> 11;
        fn_add(regs, rd, -1, offset);
        }
      }
      break;

    case FN_SUB:
      int rd = (instr & RD_MASK) >> 27;

      if (imm = 0) {
        int rs = (instr & RTYPE_RS_MASK) >> 22;
        fn_sub(regs, rd, rs, regs[rs]);
      } else {
        int offset = (instr & ITYPE_OFF_MASK) >> 11;
        fn_sub(regs, rd, -1, offset);
      }
      break;

    case FN_XOR:
      int rd = (instr & RD_MASK) >> 27;

      if (imm = 0) {
        int rs = (instr & RTYPE_RS_MASK) >> 22;
        fn_xor(regs, rd, rs, regs[rs]);
      } else {
        int offset = (instr & ITYPE_OFF_MASK) >> 11;
        fn_xor(regs, rd, -1, offset);
      }
      break;

    case FN_NEG:
      int rd = (instr & RD_MASK) >> 27;
      fn_neg(regs, rd);
      break;

    case FN_CSWAP:
      int ra = (instr & R3TYPE_RA_MASK) >> 27;
      int rb = (instr & R3TYPE_RB_MASK) >> 22;
      int rc = (instr & RTYPE_OFF_MASK) >> 17;
      fn_cswap(regs,
          ra, rb, rc,
          regs[ra], regs[rb], regs[rc]);
      break;

    case FN_MUL:
      int rd = (instr & RD_MASK) >> 27;

      if (imm = 0) {
        int rs = (instr & RTYPE_RS_MASK) >> 22;
        fn_mul(regs, rd, rs, regs[rs]);
      } else {
        int offset = (instr & ITYPE_OFF_MASK) >> 11;
        fn_mul(regs, rd, -1, offset);
      }
      break;

    case FN_DIV:
      int rd = (instr & RD_MASK) >> 27;

      if (imm = 0) {
        int rs = (instr & RTYPE_RS_MASK) >> 22;
        fn_div(regs, rd, rs, regs[rs]);
      } else {
        int offset = (instr & ITYPE_OFF_MASK) >> 11;
        fn_div(regs, rd, -1, offset);
      }
      break;

    case FN_RR:
      int rd = (instr & RD_MASK) >> 27;

      if (imm = 0) {
        int rs = (instr & RTYPE_RS_MASK) >> 22;
        fn_rr(regs, rd, rs, regs[rs]);
      } else {
        int offset = (instr & ITYPE_OFF_MASK) >> 11;
        fn_rr(regs, rd, -1, offset);
      }
      break;

    case FN_RL:
      int rd = (instr & RD_MASK) >> 27;

      if (imm = 0) {
        int rs = (instr & RTYPE_RS_MASK) >> 22;
        fn_rl(regs, rd, rs, regs[rs]);
      } else {
        int offset = (instr & ITYPE_OFF_MASK) >> 11;
        fn_rl(regs, rd, -1, offset);
      }
      break;

    // float functions
    case FN_FADD:
      int rd = (instr & RD_MASK) >> 27;

      if (imm = 0) {
        int rs = (instr & RTYPE_RS_MASK) >> 22;
        fn_fadd(regs, rd, rs, (float)regs[rs]);
      } else {
        float offset = (float)((instr & ITYPE_OFF_MASK) >> 11);
        fn_fadd(regs, rd, -1, offset);
      }
      break;

    case FN_FSUB:
      int rd = (instr & RD_MASK) >> 27;

      if (imm = 0) {
        int rs = (instr & RTYPE_RS_MASK) >> 22;
        fn_fsub(regs, rd, rs, (float)regs[rs]);
      } else {
        float offset = (float)((instr & ITYPE_OFF_MASK) >> 11);
        fn_fsub(regs, rd, -1, offset);
      }
      break;

    case FN_FMUL:
      int rd = (instr & RD_MASK) >> 27;

      if (imm = 0) {
        int rs = (instr & RTYPE_RS_MASK) >> 22;
        fn_fmul(regs, rd, rs, (float)regs[rs]);
      } else {
        float offset = (float)((instr & ITYPE_OFF_MASK) >> 11);
        fn_fmul(regs, rd, -1, offset);
      }
      break;

    case FN_FDIV:
      int rd = (instr & RD_MASK) >> 27;

      if (imm = 0) {
        int rs = (instr & RTYPE_RS_MASK) >> 22;
        fn_fdiv(regs, rd, rs, (float)regs[rs]);
      } else {
        float offset = (float)((instr & ITYPE_OFF_MASK) >> 11);
        fn_fdiv(regs, rd, -1, offset);
      }
      break;
  }
  return 0;
}

/* evaluates a single instruction in reverse */
void r_eval(
    int regs[REGS_NUM],
    int *m_regs, int *garbage,
    int *direction, int *branch,
    int instr)
{
  int imm = 0;
  
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
      
      fn_cswap(regs,
          ra, rb, -1,
          0, 0, offset);
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
      int rd = (instr & RD_MASK) >> 27;
      int rs = (instr & RTYPE_RS_MASK) >> 22;
      int offset = (instr & RTYPE_OFF_MASK) >> 6;
      
      int addr = regs[rs] + offset;
      int m_val = *(m_regs + addr);
      int r_val = regs[rd];

      regs[rd] = m_val;
      *(m_regs + addr) = r_val;
      break;

    case OP_DEL:
      int rd = (instr & RD_MASK) >> 27;
      int value = (*garbage);

      // pop value off garbage stack to regs[rd]
      regs[rd] = value;
      (*garbage) = 0;
      garbage--;
      break;

    case OP_MDEL:
      int rd = (instr & RD_MASK) >> 27;
      int offset = (instr & ITYPE_OFF_MASK) >> 11;
      int addr = regs[rd] + offset;
      int value = (*garbage);

      // pop value off garbage stack to mem location
      *(m_regs + addr) = value;
      (*garbage) = 0;
      garbage--;
      break;
    
    // control
    case OP_BLTU:
    case OP_BLT:
      int rd = (instr & RD_MASK) >> 27;
      int rs = (instr & RTYPE_RS_MASK) >> 22;
      int offset = (instr & RTYPE_OFF_MASK) >> 6;

      if (regs[rd] < regs[rs])
        (*branch) -= offset;
      break;

    case OP_BGEU:
    case OP_BGE:
      int rd = (instr & RD_MASK) >> 27;
      int rs = (instr & RTYPE_RS_MASK) >> 22;
      int offset = (instr & RTYPE_OFF_MASK) >> 6;

      if (regs[rd] >= regs[rs])
        (*branch) -= offset;
      break;

    case OP_BEQ:
      int rd = (instr & RD_MASK) >> 27;
      int rs = (instr & RTYPE_RS_MASK) >> 22;
      int offset = (instr & RTYPE_OFF_MASK) >> 6;

      if (regs[rd] == regs[rs])
        (*branch) -= offset;
      break;

    case OP_BNE:
      int rd = (instr & RD_MASK) >> 27;
      int rs = (instr & RTYPE_RS_MASK) >> 22;
      int offset = (instr & RTYPE_OFF_MASK) >> 6;

      if (regs[rd] != regs[rs])
        (*branch) -= offset;
      break;

    case OP_BEVN:
      int rd = (instr & RD_MASK) >> 27;
      int offset = (instr & ITYPE_OFF_MASK) >> 11;

      if ((regs[rd] % 2) == 0)
        (*branch) -= offset;
      break;

    case OP_BODD:
      int rd = (instr & RD_MASK) >> 27;
      int offset = (instr & ITYPE_OFF_MASK) >> 11;

      if ((regs[rd] % 2) == 1)
        (*branch) -= offset;
      break;

    case OP_SWB:
      int rd = (instr & RD_MASK) >> 27;
      int bval = (*branch);

      (*branch) = regs[rd];
      regs[rd] = bval;
      break;

    case OP_RSWB:
      int rd = (instr & RD_MASK) >> 27;
      int bval = (*branch);

      (*branch) = regs[rd];
      regs[rd] = bval;
      (*direction) = ~(*direction);
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
        fn_sub(regs, rd, rs, regs[rs]);
      } else {
        int offset = (instr & ITYPE_OFF_MASK) >> 11;
        fn_sub(regs, rd, -1, offset);
      }
      break;

    case FN_SUB:
      int rd = (instr & RD_MASK) >> 27;

      if (imm = 0) {
        int rs = (instr & RTYPE_RS_MASK) >> 22;
        fn_add(regs, rd, rs, regs[rs]);
      } else {
        int offset = (instr & ITYPE_OFF_MASK) >> 11;
        fn_add(regs, rd, -1, offset);
      }
      break;
    
    case FN_XOR:
      int rd = (instr & RD_MASK) >> 27;

      if (imm = 0) {
        int rs = (instr & RTYPE_RS_MASK) >> 22;
        fn_xor(regs, rd, rs, regs[rs]);
      } else {
        int offset = (instr & ITYPE_OFF_MASK) >> 11;
        fn_xor(regs, rd, -1, offset);
      }
      break;

    case FN_NEG:
      int rd = (instr & RD_MASK) >> 27;
      fn_neg(regs, rd);
      break;

    case FN_CSWAP:
      int ra = (instr & R3TYPE_RA_MASK) >> 27;
      int rb = (instr & R3TYPE_RB_MASK) >> 22;
      int rc = (instr & RTYPE_OFF_MASK) >> 17;
      fn_cswap(regs,
          ra, rb, rc,
          regs[ra], regs[rb], regs[rc]);
      break;

    case FN_MUL:
      int rd = (instr & RD_MASK) >> 27;

      if (imm = 0) {
        int rs = (instr & RTYPE_RS_MASK) >> 22;
        fn_div(regs, rd, rs, regs[rs]);
      } else {
        int offset = (instr & ITYPE_OFF_MASK) >> 11;
        fn_div(regs, rd, -1, offset);
      }
      break;

    case FN_DIV:
      int rd = (instr & RD_MASK) >> 27;

      if (imm = 0) {
        int rs = (instr & RTYPE_RS_MASK) >> 22;
        fn_mul(regs, rd, rs, regs[rs]);
      } else {
        int offset = (instr & ITYPE_OFF_MASK) >> 11;
        fn_mul(regs, rd, -1, offset);
      }
      break;

    case FN_RR:
      int rd = (instr & RD_MASK) >> 27;

      if (imm = 0) {
        int rs = (instr & RTYPE_RS_MASK) >> 22;
        fn_rl(regs, rd, rs, regs[rs]);
      } else {
        int offset = (instr & ITYPE_OFF_MASK) >> 11;
        fn_rl(regs, rd, -1, offset);
      }
      break;

    case FN_RL:
      int rd = (instr & RD_MASK) >> 27;

      if (imm = 0) {
        int rs = (instr & RTYPE_RS_MASK) >> 22;
        fn_rr(regs, rd, rs, regs[rs]);
      } else {
        int offset = (instr & ITYPE_OFF_MASK) >> 11;
        fn_rr(regs, rd, -1, offset);
      }
      break;

    /* float functions */
    case FN_FADD:
      int rd = (instr & RD_MASK) >> 27;

      if (imm = 0) {
        int rs = (instr & RTYPE_RS_MASK) >> 22;
        fn_fsub(regs, rd, rs, (float)regs[rs]);
      } else {
        float offset = (float)((instr & ITYPE_OFF_MASK) >> 11);
        fn_fsub(regs, rd, -1, offset);
      }
      break;

    case FN_FSUB:
      int rd = (instr & RD_MASK) >> 27;

      if (imm = 0) {
        int rs = (instr & RTYPE_RS_MASK) >> 22;
        fn_fadd(regs, rd, rs, (float)regs[rs]);
      } else {
        float offset = (float)((instr & ITYPE_OFF_MASK) >> 11);
        fn_fadd(regs, rd, -1, offset);
      }
      break;

    case FN_FMUL:
      int rd = (instr & RD_MASK) >> 27;

      if (imm = 0) {
        int rs = (instr & RTYPE_RS_MASK) >> 22;
        fn_fdiv(regs, rd, rs, (float)regs[rs]);
      } else {
        float offset = (float)((instr & ITYPE_OFF_MASK) >> 11);
        fn_fdiv(regs, rd, -1, offset);
      }
      break;

    case FN_FDIV:
      int rd = (instr & RD_MASK) >> 27;

      if (imm = 0) {
        int rs = (instr & RTYPE_RS_MASK) >> 22;
        fn_fmul(regs, rd, rs, (float)regs[rs]);
      } else {
        float offset = (float)((instr & ITYPE_OFF_MASK) >> 11);
        fn_fmul(regs, rd, -1, offset);
      }
      break;
  }
  return 0;
}
