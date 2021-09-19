// Arithmetic
ADD @d @s|*imm
/*
 * Add two registers or one register and immediate,
 * result placed in @d
 */

SUB @d @s|*imm
/*
 * Subtract @s|*imm from @d, result placed in @d
 */

// Logic
NEG @d
/*
 * Negate @d
 */

XOR @d @s|*imm
/*
 * Exclusive-or @d and @s|*imm, result placed in @d
 */

CSWAP @a @b @c|*imm
/*
 * Fredkin gate on @a and @b, with @c|*imm as the control,
 * results placed in @a and @b
 */

// Control Flow
BLT @d @s *off
/*
 * Branch by offset *off if @d is less
 * than @s
 */

BLTU @d @s *off
/*
 * BLT on unsigned integers
 */

BGE @d @s *off
/*
 * Branch by offset *off if @d is greater
 * than or equal to @s
 */

BGEU @d @s *off
/*
 * BGE on unsigned integers
 */

BEQ @d @s *off
/*
 * Branch by offset *off if @d is equal
 * to @s
 */

BNE @d @s *off
/*
 * Branch by offset *off if @d is not
 * equal to @s
 */

BEVN @d *off
/*
 * Branch by offset *off if @d is even
 */

BODD @d *off
/*
 * Branch by offset *off if @d is odd
 */

SWB @d
/*
 * Swaps @d with the branch register
 */

RSWB @d
/*
 * Swaps @d with the branch register and
 * flips the direction bit
 */

// Memory
EXCH @d @s
/*
 * Swap the values in @d and @s
 */

// IRIS Extensions
/// I: Irreversibility
DEL @d
/*
 * Deletes the value in @d
 *
 * This is the only irreversible operation in
 * IRIS, and thus can only be used inside arrows
 */

/// M: Multiplication and Division
MUL @d @s|*imm
/* 
 * Multiply @d with @s|*imm, result placed in @d
 */

DIV @d @s|*imm
/* 
 * Divide @d by @s|*imm, result placed in @d
 */

/// F: Floating Point
FADD @d @s|*imm
/*
 * Add two registers, result placed in @d
 */

FSUB @d @s|*imm 
/*
 * Subtract @s|*imm from @d, result placed in @d
 */

FMUL @d @s|*imm
/* 
 * Multiply @d with @s|*imm, result placed in @d
 *
 * This instruction is included only if the
 * M extension is enabled
 */

FDIV @d @s|*imm
/* 
 * Divide @d by @s|*imm, result placed in @d
 *
 * This instruction is included only if the
 * M extension is enabled
 */

/// B: Bitrotating
RL @d @s|*imm
/*
 * Rotate bits in @d left by value in @s|*imm
 */

RR @d @s|*imm
/*
 * Rotate bits in @d right by value in @s|*imm
 */

/// V: Vector Instructions
VADD @vd @vs|*imm
/*
 * Add two registers or immediate, result placed in @vd
 */

VSUB @vd @vs|*imm 
/*
 * Subtract @vd from @vs|*imm, result placed in @vd
 */

VNEG @v
/*
 * Negate @v
 */

VXOR @vd @vs|*imm
/*
 * Exclusive-or @vd and @vs|*imm, result placed in @vd
 */

VCSWAP @va @vb @vc|*imm
/*
 * Fredkin gate with @va, @vb and @vc|*imm
 */

VEXCH @vd @vs
/*
 * Swap the vectors in @vd and @vs
 */

VDEL @v
/*
 * Deletes the vector @v
 *
 * This instruction is included only if the
 * I extension is enabled
 */

VMUL @vd @vs|*imm
/* 
 * Multiply @vd with @vs|*imm, result placed in @vd
 *
 * This instruction is included only if the
 * M extension is enabled
 */

VDIV @vd @vs|*imm
/* 
 * Divide @vd by @vs|*imm, result placed in @vd
 *
 * This instruction is included only if the
 * M extension is enabled
 */

VRL @vd @vs|*imm
/*
 * Rotate bits in @vd left by value in @vs|*imm
 *
 * This instruction is included only if the
 * B extension is enabled
 */

VRR @vd @vs|*imm
/*
 * Rotate bits in @vd right by value in @vs|*imm
 *
 * This instruction is included only if the
 * B extension is enabled
 */

VFADD @vd @vs|*imm
/*
 * Add two registers or immediate, result placed in @vd
 *
 * This instruction is included only if the
 * F extension is enabled
 */

VFSUB @vd @vs|*imm
/*
 * Subtract @vs|*imm from @vd, result placed in @vd
 *
 * This instruction is included only if the
 * F extension is enabled
 */

VFMUL @vd @vs|*imm
/* 
 * Multiply @vd with @vs|*imm, result placed in @vd
 *
 * This instruction is included only if the
 * F and M extensions are enabled
 */

VFDIV @vd @vs|*imm
/* 
 * Divide @vd by @vs|*imm, result placed in @d
 *
 * This instruction is included only if the
 * F and M extensions are enabled
 */
