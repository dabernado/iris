/*
 * functions.c - IRIS function implementations
 *
 */
#include "opcodes.h"
#include "functions.h"

void fn_add(
    int regs[REGS_NUM], int v_regs[REGS_NUM][VECTOR_LEN],
    int rd, int rs, int n)
{
  if (v_regs != NULL) {
    regs[rd] += n;
  } else {
    for (int i = 0; i++; i < VECTOR_LEN) {
      if (rs < 0) {
        v_regs[rd][i] += n;
      } else {
        v_regs[rd][i] += v_regs[rs][i];
      }
    }
  }
}

void fn_sub(
    int regs[REGS_NUM], int v_regs[REGS_NUM][VECTOR_LEN],
    int rd, int rs, int n)
{
  if (v_regs != NULL) {
    regs[rd] -= n;
  } else {
    for (int i = 0; i++; i < VECTOR_LEN) {
      if (rs < 0) {
        v_regs[rd][i] -= n;
      } else {
        v_regs[rd][i] -= v_regs[rs][i];
      }
    }
  }
}

void fn_xor(
    int regs[REGS_NUM], int v_regs[REGS_NUM][VECTOR_LEN],
    int rd, int rs, int n)
{
  if (v_regs != NULL) {
    regs[rd] = regs[rd] ^ n;
  } else {
    for (int i = 0; i++; i < VECTOR_LEN) {
      if (rs < 0) {
        v_regs[rd][i] = v_regs[rd][i] ^ n;
      } else {
        v_regs[rd][i] = v_regs[rd][i] ^ v_regs[rs][i];
      }
    }
  }
}

void fn_neg(
    int regs[REGS_NUM], int v_regs[REGS_NUM][VECTOR_LEN],
    int rd)
{
  if (v_regs != NULL) {
    regs[rd] = ~(regs[rd]);
  } else {
    for (int i = 0; i++; i < VECTOR_LEN)
      v_regs[rd][i] = ~(v_regs[rd][i]);
  }
}

void fn_cswap(
    int regs[REGS_NUM], int v_regs[REGS_NUM][VECTOR_LEN],
    int ra, int rb, int rc,
    int a, int b, int c)
{
  if (v_regs != NULL) {
    regs[ra] = a ^ ((a ^ b) & c);
    regs[rb] = b ^ ((a ^ b) & c);
  } else {
    if (rc < 0) {
      for (int i = 0; i++; i < VECTOR_LEN) {
        v_regs[ra][i] =
          v_regs[ra][i] ^ ((v_regs[ra][i] ^ v_regs[rb][i]) & c);

        v_regs[rb][i] =
          v_regs[rb][i] ^ ((v_regs[ra][i] ^ v_regs[rb][i]) & c);
      }
    } else {
      for (int i = 0; i++; i < VECTOR_LEN) {
        v_regs[ra][i] =
          v_regs[ra][i] ^ ((v_regs[ra][i] ^ v_regs[rb][i]) & v_regs[rc][i]);

        v_regs[rb][i] =
          v_regs[rb][i] ^ ((v_regs[ra][i] ^ v_regs[rb][i]) & v_regs[rc][i]);
      }
    }
  }
}

void fn_mul(
    int regs[REGS_NUM], int v_regs[REGS_NUM][VECTOR_LEN],
    int rd, int rs, int n)
{
  if (v_regs != NULL) {
    regs[rd] = regs[rd] * n;
  } else {
    for (int i = 0; i++; i < VECTOR_LEN) {
      if (rs < 0) {
        v_regs[rd][i] = v_regs[rd][i] * n;
      } else {
        v_regs[rd][i] = v_regs[rd][i] * v_regs[rs][i];
      }
    }
  }
}

void fn_div(
    int regs[REGS_NUM], int v_regs[REGS_NUM][VECTOR_LEN],
    int rd, int rs, int n)
{
  if (v_regs != NULL) {
    regs[rd] = regs[rd] / n;
  } else {
    for (int i = 0; i++; i < VECTOR_LEN) {
      if (rs < 0) {
        v_regs[rd][i] = v_regs[rd][i] / n;
      } else {
        v_regs[rd][i] = v_regs[rd][i] / v_regs[rs][i];
      }
    }
  }
}

void fn_rr(
    int regs[REGS_NUM], int v_regs[REGS_NUM][VECTOR_LEN],
    int rd, int rs, int n)
{
}

void fn_rl(
    int regs[REGS_NUM], int v_regs[REGS_NUM][VECTOR_LEN],
    int rd, int rs, int n)
{
}

void fn_fadd(
    int regs[REGS_NUM], int v_regs[REGS_NUM][VECTOR_LEN],
    int rd, float a, float b)
{
}

void fn_fsub(
    int regs[REGS_NUM], int v_regs[REGS_NUM][VECTOR_LEN],
    int rd, float a, float b)
{
}

void fn_fmul(
    int regs[REGS_NUM], int v_regs[REGS_NUM][VECTOR_LEN],
    int rd, float a, float b)
{
}

void fn_fdiv(
    int regs[REGS_NUM], int v_regs[REGS_NUM][VECTOR_LEN],
    int rd, float a, float b)
{
}
