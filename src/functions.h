/*
 * functions.h - IRIS function implementations
 *
 */

#ifndef IRIS_FNS
#define IRIS_FNS

#include "opcodes.h"

void fn_add(
    int regs[REGS_NUM],
    int rd, int rs);
void fn_sub(
    int regs[REGS_NUM],
    int rd, int rs);
void fn_xor(
    int regs[REGS_NUM],
    int rd, int rs);
void fn_neg(
    int regs[REGS_NUM],
    int rd);

void fn_cswap(
    int regs[REGS_NUM],
    int ra, int rb, int rc);

void fn_mul(
    int regs[REGS_NUM],
    int rd, int rs);
void fn_div(
    int regs[REGS_NUM],
    int rd, int rs);

void fn_rr(
    int regs[REGS_NUM],
    int rd, int rs);
void fn_rl(
    int regs[REGS_NUM],
    int rd, int rs);

void fn_fadd(
    int regs[REGS_NUM],
    int rd, int rs);
void fn_fsub(
    int regs[REGS_NUM],
    int rd, int rs);
void fn_fmul(
    int regs[REGS_NUM],
    int rd, int rs);
void fn_fdiv(
    int regs[REGS_NUM],
    int rd, int rs);

#endif
