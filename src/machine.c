/*
 * machine.c - Iris VM implementation
 *
 */

#include "opcodes.h"
#include "functions.h"

#define BIT_WIDTH 32

/* evaluates a single instruction */
void eval(
    int regs[REGS_NUM], int v_regs[REGS_NUM][VECTOR_LEN],
    int *m_regs, int *garbage, int mem_size
    int *direction, int *branch,
    int instr)
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
      
      fn_cswap(regs, NULL,
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
      
      fn_cswap(NULL, v_regs,
          ra, rb, -1,
          0, 0, offset);

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
      int rd = (instr & RD_MASK) >> 27;
      int rs = (instr & RTYPE_RS_MASK) >> 22;
      int offset = (instr & RTYPE_OFF_MASK) >> 6;
      
      int addr = regs[rs] + offset;
      for (int i = 0; i < VECTOR_LEN; i++)
      {
        int m_val = *(m_regs + addr + i);
        int r_val = v_regs[rd][i];

        v_regs[rd][i] = m_val;
        *(m_regs + addr + i) = r_val;
      }
      break;

    case OP_VDEL:
      int rd = (instr & RD_MASK) >> 27;

      // push v_regs[rd] onto garbage stack
      for (int i = 0; i++; i < VECTOR_LEN)
        int value = v_regs[rd][i];
        v_regs[rd][i] = 0;
        (*garbage) = value;
        garbage++;
      break;

    case OP_VMDEL:
      int rd = (instr & RD_MASK) >> 27;
      int offset = (instr & ITYPE_OFF_MASK) >> 11;
      int addr = regs[rd] + offset;

      // push mem value onto garbage stack
      for (int i = 0; i < VECTOR_LEN; i++)
      {
        int m_val = *(m_regs + addr + i);
        *(m_regs + addr + i) = 0;
        (*garbage) = m_val;
        garbage++;
      }
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

        if (v_op == 0) {
          fn_add(regs, NULL, rd, rs, regs[rs]);
        } else {
          fn_add(NULL, v_regs, rd, rs, 0);
        }
      } else {
        int offset = (instr & ITYPE_OFF_MASK) >> 11;

        if (v_op == 0) {
          fn_add(regs, NULL, rd, -1, offset);
        } else {
          fn_add(NULL, v_regs, rd, -1, offset);
        }
      }
      break;

    case FN_SUB:
      int rd = (instr & RD_MASK) >> 27;

      if (imm = 0) {
        int rs = (instr & RTYPE_RS_MASK) >> 22;

        if (v_op == 0) {
          fn_sub(regs, NULL, rd, rs, regs[rs]);
        } else {
          fn_sub(NULL, v_regs, rd, rs, 0);
        }
      } else {
        int offset = (instr & ITYPE_OFF_MASK) >> 11;

        if (v_op == 0) {
          fn_sub(regs, NULL, rd, -1, offset);
        } else {
          fn_sub(NULL, v_regs, rd, -1, offset);
        }
      }
      break;

    case FN_XOR:
      int rd = (instr & RD_MASK) >> 27;

      if (imm = 0) {
        int rs = (instr & RTYPE_RS_MASK) >> 22;

        if (v_op == 0) {
          fn_xor(regs, NULL, rd, rs, regs[rs]);
        } else {
          fn_xor(NULL, v_regs, rd, rs, 0);
        }
      } else {
        int offset = (instr & ITYPE_OFF_MASK) >> 11;

        if (v_op == 0) {
          fn_xor(regs, NULL, rd, -1, offset);
        } else {
          fn_xor(NULL, v_regs, rd, -1, offset);
        }
      }
      break;

    case FN_NEG:
      int rd = (instr & RD_MASK) >> 27;

      if (v_op == 0) {
        fn_neg(regs, NULL, rd);
      } else {
        fn_neg(NULL, v_regs, rd);
      }
      break;

    case FN_CSWAP:
      int ra = (instr & R3TYPE_RA_MASK) >> 27;
      int rb = (instr & R3TYPE_RB_MASK) >> 22;
      int rc = (instr & RTYPE_OFF_MASK) >> 17;
      
      if (v_op == 0) {
        fn_cswap(regs, NULL
            ra, rb, rc,
            regs[ra], regs[rb], regs[rc]);
      } else {
        fn_cswap(NULL, v_regs
            ra, rb, rc,
            0, 0, 0);
      }
      break;

    case FN_MUL:
      int rd = (instr & RD_MASK) >> 27;

      if (imm = 0) {
        int rs = (instr & RTYPE_RS_MASK) >> 22;

        if (v_op == 0) {
          fn_mul(regs, NULL, rd, rs, regs[rs]);
        } else {
          fn_mul(NULL, v_regs, rd, rs, 0);
        }
      } else {
        int offset = (instr & ITYPE_OFF_MASK) >> 11;

        if (v_op == 0) {
          fn_mul(regs, NULL, rd, -1, offset);
        } else {
          fn_mul(NULL, v_regs, rd, -1, offset);
        }
      }
      break;

    case FN_DIV:
      int rd = (instr & RD_MASK) >> 27;

      if (imm = 0) {
        int rs = (instr & RTYPE_RS_MASK) >> 22;

        if (v_op == 0) {
          fn_div(regs, NULL, rd, rs, regs[rs]);
        } else {
          fn_div(NULL, v_regs, rd, rs, 0);
        }
      } else {
        int offset = (instr & ITYPE_OFF_MASK) >> 11;

        if (v_op == 0) {
          fn_div(regs, NULL, rd, -1, offset);
        } else {
          fn_div(NULL, v_regs, rd, -1, offset);
        }
      }
      break;

    case FN_RR:
      int rd = (instr & RD_MASK) >> 27;

      if (imm = 0) {
        int rs = (instr & RTYPE_RS_MASK) >> 22;

        if (v_op == 0) {
          fn_rr(regs, NULL, rd, rs, regs[rs]);
        } else {
          fn_rr(NULL, v_regs, rd, rs, 0);
        }
      } else {
        int offset = (instr & ITYPE_OFF_MASK) >> 11;

        if (v_op == 0) {
          fn_rr(regs, NULL, rd, -1, offset);
        } else {
          fn_rr(NULL, v_regs, rd, -1, offset);
        }
      }
      break;

    case FN_RL:
      int rd = (instr & RD_MASK) >> 27;

      if (imm = 0) {
        int rs = (instr & RTYPE_RS_MASK) >> 22;

        if (v_op == 0) {
          fn_rl(regs, NULL, rd, rs, regs[rs]);
        } else {
          fn_rl(NULL, v_regs, rd, rs, 0);
        }
      } else {
        int offset = (instr & ITYPE_OFF_MASK) >> 11;

        if (v_op == 0) {
          fn_rl(regs, NULL, rd, -1, offset);
        } else {
          fn_rl(NULL, v_regs, rd, -1, offset);
        }
      }
      break;

    // float functions
    case FN_FADD:
      int rd = (instr & RD_MASK) >> 27;

      if (imm = 0) {
        int rs = (instr & RTYPE_RS_MASK) >> 22;

        if (v_op == 0) {
          fn_fadd(regs, NULL, rd, rs, (float)regs[rs]);
        } else {
          fn_fadd(NULL, v_regs, rd, rs, 0.0);
        }
      } else {
        float offset = (float)((instr & ITYPE_OFF_MASK) >> 11);

        if (v_op == 0) {
          fn_fadd(regs, NULL, rd, -1, offset);
        } else {
          fn_fadd(NULL, v_regs, rd, -1, offset);
        }
      }
      break;

    case FN_FSUB:
      int rd = (instr & RD_MASK) >> 27;

      if (imm = 0) {
        int rs = (instr & RTYPE_RS_MASK) >> 22;

        if (v_op == 0) {
          fn_fsub(regs, NULL, rd, rs, (float)regs[rs]);
        } else {
          fn_fsub(NULL, v_regs, rd, rs, 0.0);
        }
      } else {
        float offset = (float)((instr & ITYPE_OFF_MASK) >> 11);

        if (v_op == 0) {
          fn_fsub(regs, NULL, rd, -1, offset);
        } else {
          fn_fsub(NULL, v_regs, rd, -1, offset);
        }
      }
      break;

    case FN_FMUL:
      int rd = (instr & RD_MASK) >> 27;

      if (imm = 0) {
        int rs = (instr & RTYPE_RS_MASK) >> 22;

        if (v_op == 0) {
          fn_fmul(regs, NULL, rd, rs, (float)regs[rs]);
        } else {
          fn_fmul(NULL, v_regs, rd, rs, 0.0);
        }
      } else {
        float offset = (float)((instr & ITYPE_OFF_MASK) >> 11);

        if (v_op == 0) {
          fn_fmul(regs, NULL, rd, -1, offset);
        } else {
          fn_fmul(NULL, v_regs, rd, -1, offset);
        }
      }
      break;

    case FN_FDIV:
      int rd = (instr & RD_MASK) >> 27;

      if (imm = 0) {
        int rs = (instr & RTYPE_RS_MASK) >> 22;

        if (v_op == 0) {
          fn_fdiv(regs, NULL, rd, rs, (float)regs[rs]);
        } else {
          fn_fdiv(NULL, v_regs, rd, rs, 0.0);
        }
      } else {
        float offset = (float)((instr & ITYPE_OFF_MASK) >> 11);

        if (v_op == 0) {
          fn_fdiv(regs, NULL, rd, -1, offset);
        } else {
          fn_fdiv(NULL, v_regs, rd, -1, offset);
        }
      }
      break;
  }
  return 0;
}

/* evaluates a single instruction in reverse */
void r_eval(
    int regs[REGS_NUM], int v_regs[REGS_NUM][VECTOR_LEN],
    int *m_regs, int *garbage, int mem_size
    int *direction, int *branch,
    int instr)
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
      
      fn_cswap(regs, NULL,
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
      
      fn_cswap(NULL, v_regs,
          ra, rb, -1,
          0, 0, offset);

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
      int rd = (instr & RD_MASK) >> 27;
      int rs = (instr & RTYPE_RS_MASK) >> 22;
      int offset = (instr & RTYPE_OFF_MASK) >> 6;
      
      int addr = regs[rs] + offset;
      for (int i = 0; i < VECTOR_LEN; i++)
      {
        int m_val = *(m_regs + addr + i);
        int r_val = v_regs[rd][i];

        v_regs[rd][i] = m_val;
        *(m_regs + addr + i) = r_val;
      }
      break;

    case OP_VDEL:
      int rd = (instr & RD_MASK) >> 27;

      // pop value off garbage stack to v_regs[rd]
      for (int i = 0; i < VECTOR_LEN; i++)
      {
        int value = (*garbage);
        v_regs[rd][i] = value;
        (*garbage) = 0;
        garbage--;
      }
      break;

    case OP_VMDEL:
      int rd = (instr & RD_MASK) >> 27;
      int offset = (instr & ITYPE_OFF_MASK) >> 11;
      int addr = regs[rd] + offset;

      // pop value off garbage stack to mem location
      for (int i = 0; i < VECTOR_LEN; i++)
      {
        int value = (*garbage);
        *(m_regs + addr + i) = value;
        (*garbage) = 0;
        garbage--;
      }
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

        if (v_op == 0) {
          fn_sub(regs, NULL, rd, rs, regs[rs]);
        } else {
          fn_sub(NULL, v_regs, rd, rs, 0);
        }
      } else {
        int offset = (instr & ITYPE_OFF_MASK) >> 11;

        if (v_op == 0) {
          fn_sub(regs, NULL, rd, -1, offset);
        } else {
          fn_sub(NULL, v_regs, rd, -1, offset);
        }
      }
      break;

    case FN_SUB:
      int rd = (instr & RD_MASK) >> 27;

      if (imm = 0) {
        int rs = (instr & RTYPE_RS_MASK) >> 22;

        if (v_op == 0) {
          fn_add(regs, NULL, rd, rs, regs[rs]);
        } else {
          fn_add(NULL, v_regs, rd, rs, 0);
        }
      } else {
        int offset = (instr & ITYPE_OFF_MASK) >> 11;

        if (v_op == 0) {
          fn_add(regs, NULL, rd, -1, offset);
        } else {
          fn_add(NULL, v_regs, rd, -1, offset);
        }
      }
      break;
    
    case FN_XOR:
      int rd = (instr & RD_MASK) >> 27;

      if (imm = 0) {
        int rs = (instr & RTYPE_RS_MASK) >> 22;

        if (v_op == 0) {
          fn_xor(regs, NULL, rd, rs, regs[rs]);
        } else {
          fn_xor(NULL, v_regs, rd, rs, 0);
        }
      } else {
        int offset = (instr & ITYPE_OFF_MASK) >> 11;

        if (v_op == 0) {
          fn_xor(regs, NULL, rd, -1, offset);
        } else {
          fn_xor(NULL, v_regs, rd, -1, offset);
        }
      }
      break;

    case FN_NEG:
      int rd = (instr & RD_MASK) >> 27;

      if (v_op == 0) {
        fn_neg(regs, NULL, rd);
      } else {
        fn_neg(NULL, v_regs, rd);
      }
      break;

    case FN_CSWAP:
      int ra = (instr & R3TYPE_RA_MASK) >> 27;
      int rb = (instr & R3TYPE_RB_MASK) >> 22;
      int rc = (instr & RTYPE_OFF_MASK) >> 17;
      
      if (v_op == 0) {
        fn_cswap(regs, NULL
            ra, rb, rc,
            regs[ra], regs[rb], regs[rc]);
      } else {
        fn_cswap(NULL, v_regs
            ra, rb, rc,
            0, 0, 0);
      }
      break;

    case FN_MUL:
      int rd = (instr & RD_MASK) >> 27;

      if (imm = 0) {
        int rs = (instr & RTYPE_RS_MASK) >> 22;

        if (v_op == 0) {
          fn_div(regs, NULL, rd, rs, regs[rs]);
        } else {
          fn_div(NULL, v_regs, rd, rs, 0);
        }
      } else {
        int offset = (instr & ITYPE_OFF_MASK) >> 11;

        if (v_op == 0) {
          fn_div(regs, NULL, rd, -1, offset);
        } else {
          fn_div(NULL, v_regs, rd, -1, offset);
        }
      }
      break;

    case FN_DIV:
      int rd = (instr & RD_MASK) >> 27;

      if (imm = 0) {
        int rs = (instr & RTYPE_RS_MASK) >> 22;

        if (v_op == 0) {
          fn_mul(regs, NULL, rd, rs, regs[rs]);
        } else {
          fn_mul(NULL, v_regs, rd, rs, 0);
        }
      } else {
        int offset = (instr & ITYPE_OFF_MASK) >> 11;

        if (v_op == 0) {
          fn_mul(regs, NULL, rd, -1, offset);
        } else {
          fn_mul(NULL, v_regs, rd, -1, offset);
        }
      }
      break;

    case FN_RR:
      int rd = (instr & RD_MASK) >> 27;

      if (imm = 0) {
        int rs = (instr & RTYPE_RS_MASK) >> 22;

        if (v_op == 0) {
          fn_rl(regs, NULL, rd, rs, regs[rs]);
        } else {
          fn_rl(NULL, v_regs, rd, rs, 0);
        }
      } else {
        int offset = (instr & ITYPE_OFF_MASK) >> 11;

        if (v_op == 0) {
          fn_rl(regs, NULL, rd, -1, offset);
        } else {
          fn_rl(NULL, v_regs, rd, -1, offset);
        }
      }
      break;

    case FN_RL:
      int rd = (instr & RD_MASK) >> 27;

      if (imm = 0) {
        int rs = (instr & RTYPE_RS_MASK) >> 22;

        if (v_op == 0) {
          fn_rr(regs, NULL, rd, rs, regs[rs]);
        } else {
          fn_rr(NULL, v_regs, rd, rs, 0);
        }
      } else {
        int offset = (instr & ITYPE_OFF_MASK) >> 11;

        if (v_op == 0) {
          fn_rr(regs, NULL, rd, -1, offset);
        } else {
          fn_rr(NULL, v_regs, rd, -1, offset);
        }
      }
      break;

    /* float functions */
    case FN_FADD:
      int rd = (instr & RD_MASK) >> 27;

      if (imm = 0) {
        int rs = (instr & RTYPE_RS_MASK) >> 22;

        if (v_op == 0) {
          fn_fsub(regs, NULL, rd, rs, (float)regs[rs]);
        } else {
          fn_fsub(NULL, v_regs, rd, rs, 0.0);
        }
      } else {
        float offset = (float)((instr & ITYPE_OFF_MASK) >> 11);

        if (v_op == 0) {
          fn_fsub(regs, NULL, rd, -1, offset);
        } else {
          fn_fsub(NULL, v_regs, rd, -1, offset);
        }
      }
      break;

    case FN_FSUB:
      int rd = (instr & RD_MASK) >> 27;

      if (imm = 0) {
        int rs = (instr & RTYPE_RS_MASK) >> 22;

        if (v_op == 0) {
          fn_fadd(regs, NULL, rd, rs, (float)regs[rs]);
        } else {
          fn_fadd(NULL, v_regs, rd, rs, 0.0);
        }
      } else {
        float offset = (float)((instr & ITYPE_OFF_MASK) >> 11);

        if (v_op == 0) {
          fn_fadd(regs, NULL, rd, -1, offset);
        } else {
          fn_fadd(NULL, v_regs, rd, -1, offset);
        }
      }
      break;

    case FN_FMUL:
      int rd = (instr & RD_MASK) >> 27;

      if (imm = 0) {
        int rs = (instr & RTYPE_RS_MASK) >> 22;

        if (v_op == 0) {
          fn_fdiv(regs, NULL, rd, rs, (float)regs[rs]);
        } else {
          fn_fdiv(NULL, v_regs, rd, rs, 0.0);
        }
      } else {
        float offset = (float)((instr & ITYPE_OFF_MASK) >> 11);

        if (v_op == 0) {
          fn_fdiv(regs, NULL, rd, -1, offset);
        } else {
          fn_fdiv(NULL, v_regs, rd, -1, offset);
        }
      }
      break;

    case FN_FDIV:
      int rd = (instr & RD_MASK) >> 27;

      if (imm = 0) {
        int rs = (instr & RTYPE_RS_MASK) >> 22;

        if (v_op == 0) {
          fn_fdiv(regs, NULL, rd, rs, (float)regs[rs]);
        } else {
          fn_fmul(NULL, v_regs, rd, rs, 0.0);
        }
      } else {
        float offset = (float)((instr & ITYPE_OFF_MASK) >> 11);

        if (v_op == 0) {
          fn_fdiv(regs, NULL, rd, -1, offset);
        } else {
          fn_fmul(NULL, v_regs, rd, -1, offset);
        }
      }
      break;
  }
  return 0;
}