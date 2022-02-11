/*
 * functions.c - IRIS function implementations
 *
 */
#include "opcodes.h"
#include "functions.h"

void fn_add(int regs[REGS_NUM], int rd, int rs)
{
  regs[rd] += regs[rs];
}

void fn_sub(int regs[REGS_NUM], int rd, int rs)
{
    regs[rd] -= regs[rs];
}

void fn_xor(int regs[REGS_NUM], int rd, int rs)
{
  regs[rd] = regs[rd] ^ regs[rs];
}

void fn_neg(int regs[REGS_NUM], int rd)
{
    regs[rd] = ~(regs[rd]);
}

void fn_cswap(int regs[REGS_NUM], int ra, int rb, int rc)
{
  regs[ra] = regs[ra] ^ ((regs[ra] ^ regs[rb]) & regs[rc]);
  regs[rb] = regs[rb] ^ ((regs[ra] ^ regs[rb]) & regs[rc]);
}

void fn_mul(int regs[REGS_NUM], int rd, int rs)
{
  regs[rd] = regs[rd] * regs[rs];
}

void fn_div(int regs[REGS_NUM], int rd, int rs)
{
  regs[rd] = regs[rd] / regs[rs];
}

void fn_rr(int regs[REGS_NUM], int rd, int rs)
{
  int x = regs[rd];
  regs[rd] = (x >> regs[rs]) | (x << (32 - regs[rs]));
}

void fn_rl(int regs[REGS_NUM], int rd, int rs)
{
    int x = regs[rd];
    regs[rd] = (x << regs[rs]) | (x >> (32 - regs[rs]));
}

void fn_fadd(int regs[REGS_NUM], int rd, int rs)
{
  // convert int to float
  float x = (float)regs[rd];
  regs[rd] = (int)(x + (float)regs[rs]);
}

void fn_fsub(int regs[REGS_NUM], int rd, int rs)
{
  // convert int to float
  float x = (float)regs[rd];
  regs[rd] = (int)(x - (float)regs[rs]);
}

void fn_fmul(int regs[REGS_NUM], int rd, int rs)
{
  // convert int to float
  float x = (float)regs[rd];
  regs[rd] = (int)(x * (float)regs[rs]);
}

void fn_fdiv(int regs[REGS_NUM], int rd, int rs)
{
  // convert int to float
  float x = (float)regs[rd];
  regs[rd] = (int)(x / (float)regs[rs]);
}
