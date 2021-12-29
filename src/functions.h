/*
 * functions.h - IRIS function implementations
 *
 */

#ifndef IRIS_FNS
#define IRIS_FNS

#include "opcodes.h"

void fn_add(
    int regs[REGS_NUM], int v_regs[REGS_NUM][VECTOR_LEN],
    int rd, int rs,
    int n);
void fn_sub(
    int regs[REGS_NUM], int v_regs[REGS_NUM][VECTOR_LEN],
    int rd, int rs,
    int n);
void fn_xor(
    int regs[REGS_NUM], int v_regs[REGS_NUM][VECTOR_LEN],
    int rd, int rs,
    int n);
void fn_neg(
    int regs[REGS_NUM], int v_regs[REGS_NUM][VECTOR_LEN],
    int rd);

void fn_cswap(
    int regs[REGS_NUM], int v_regs[REGS_NUM][VECTOR_LEN],
    int ra, int rb, int rc,
    int a, int b, int c);

void fn_mul(
    int regs[REGS_NUM], int v_regs[REGS_NUM][VECTOR_LEN],
    int rd, int rs,
    int n);
void fn_div(
    int regs[REGS_NUM], int v_regs[REGS_NUM][VECTOR_LEN],
    int rd, int rs,
    int n);

void fn_rr(
    int regs[REGS_NUM], int v_regs[REGS_NUM][VECTOR_LEN],
    int rd, int rs,
    int n);
void fn_rl(
    int regs[REGS_NUM], int v_regs[REGS_NUM][VECTOR_LEN],
    int rd, int rs,
    int n);

void fn_fadd(
    int regs[REGS_NUM], int v_regs[REGS_NUM][VECTOR_LEN],
    int rd,
    float a, float b);
void fn_fsub(
    int regs[REGS_NUM], int v_regs[REGS_NUM][VECTOR_LEN],
    int rd,
    float a, float b);
void fn_fmul(
    int regs[REGS_NUM], int v_regs[REGS_NUM][VECTOR_LEN],
    int rd,
    float a, float b);
void fn_fdiv(
    int regs[REGS_NUM], int v_regs[REGS_NUM][VECTOR_LEN],
    int rd,
    float a, float b);

#endif
