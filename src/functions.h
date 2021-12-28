/*
 * functions.h - IRIS function implementations
 *
 */

#ifndef IRIS_FNS
#define IRIS_FNS

void fn_add(int rd, int rs, int n, int v_op);
void fn_sub(int rd, int rs, int n, int v_op);
void fn_xor(int rd, int rs, int n, int v_op);
void fn_neg(int rd, int v_op);

void fn_cswap(
    int ra, int rb, int rc,
    int a, int b, int c,
    int v_op);

void fn_mul(int rd, int rs, int n, int v_op);
void fn_div(int rd, int rs, int n, int v_op);

void fn_rr(int rd, int rs, int n, int v_op);
void fn_rl(int rd, int rs, int n, int v_op);

void fn_add(int rd, float a, float b, int v_op);
void fn_sub(int rd, float a, float b, int v_op);
void fn_mul(int rd, float a, float b, int v_op);
void fn_div(int rd, float a, float b, int v_op);

#endif
