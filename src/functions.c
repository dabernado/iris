/*
 * functions.c - IRIS function implementations
 *
 */

#include "functions.h"
#include "opcodes.h"

void fn_add(int regs[REGS_NUM], int rd, int n)
{
  regs[rd] += n;
}

void fn_sub(int regs[REGS_NUM], int rd, int n)
{
    regs[rd] -= n;
}

void fn_xor(int regs[REGS_NUM], int rd, int n)
{
  regs[rd] = regs[rd] ^ n;
}

void fn_neg(int regs[REGS_NUM], int rd)
{
    regs[rd] = ~(regs[rd]);
}

void fn_cswap(int regs[REGS_NUM], int ra, int rb, int c)
{
  regs[ra] = regs[ra] ^ ((regs[ra] ^ regs[rb]) & c);
  regs[rb] = regs[rb] ^ ((regs[ra] ^ regs[rb]) & c);
}

void fn_mul(int regs[REGS_NUM], int rd, int n)
{
  regs[rd] = regs[rd] * n;
}

void fn_div(int regs[REGS_NUM], int rd, int n)
{
  regs[rd] = regs[rd] / n;
}

void fn_rr(int regs[REGS_NUM], int rd, int n)
{
  int x = regs[rd];
  regs[rd] = (x >> n) | (x << (32 - n));
}

void fn_rl(int regs[REGS_NUM], int rd, int n)
{
    int x = regs[rd];
    regs[rd] = (x << n) | (x >> (32 - n));
}

void fn_fadd(int regs[REGS_NUM], int rd, float n)
{
  // convert int to float
  float x = (float)regs[rd];
  regs[rd] = (int)(x + n);
}

void fn_fsub(int regs[REGS_NUM], int rd, float n)
{
  // convert int to float
  float x = (float)regs[rd];
  regs[rd] = (int)(x - n);
}

void fn_fmul(int regs[REGS_NUM], int rd, float n)
{
  // convert int to float
  float x = (float)regs[rd];
  regs[rd] = (int)(x * n);
}

void fn_fdiv(int regs[REGS_NUM], int rd, float n)
{
  // convert int to float
  float x = (float)regs[rd];
  regs[rd] = (int)(x / n);
}
