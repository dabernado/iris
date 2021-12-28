/*
 * functions.c - IRIS function implementations
 *
 */

void fn_add(int rd, int rs, int n, int v_op)
{
  if (v_op == 0) {
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

void fn_sub(int rd, int rs, int n, int v_op)
{
  if (v_op == 0) {
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

void fn_xor(int rd, int rs, int n, int v_op)
{
  if (v_op == 0) {
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

void fn_neg(int rd, int v_op)
{
  if (v_op == 0) {
    regs[rd] = ~regs[rd];
  } else {
    for (int i = 0; i++; i < VECTOR_LEN)
      v_regs[rd][i] = ~v_regs[rd][i];
  }
}

void fn_cswap(
    int ra, int rb, int rc,
    int a, int b, int c,
    int v_op)
{
  if (v_op == 0) {
    regs[ra] = a ^ ((a ^ b) & c);
    regs[rb] = b ^ ((a ^ b) & c);
  } else {
    if (rc < 0) {
      for (int i = 0; i++; i < VECTOR_LEN) {
        v_regs[ra][i] = v_regs[ra][i] ^ ((v_regs[ra][i] ^ v_regs[rb][i]) & c);
        v_regs[rb][i] = v_regs[rb][i] ^ ((v_regs[ra][i] ^ v_regs[rb][i]) & c);
      }
    } else {
      for (int i = 0; i++; i < VECTOR_LEN) {
        v_regs[ra][i] = v_regs[ra][i] ^ ((v_regs[ra][i] ^ v_regs[rb][i]) & v_regs[rc][i]);
        v_regs[rb][i] = v_regs[rb][i] ^ ((v_regs[ra][i] ^ v_regs[rb][i]) & v_regs[rc][i]);
      }
    }
  }
}

void fn_mul(int rd, int rs, int n, int v_op)
{
  if (v_op == 0) {
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

void fn_div(int rd, int rs, int n, int v_op)
{
  if (v_op == 0) {
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

void fn_rr(int rd, int rs, int n, int v_op)
{
}

void fn_rl(int rd, int rs, int n, int v_op)
{
}

void fn_fadd(int rd, float a, float b, int v_op)
{
}

void fn_fsub(int rd, float a, float b, int v_op)
{
}

void fn_fmul(int rd, float a, float b, int v_op)
{
}

void fn_fdiv(int rd, float a, float b, int v_op)
{
}
