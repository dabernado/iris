// Arithmetic
ADD @d @s
/*
 * Add two registers, result placed in @d
 */

ADDI @d [imm]
/* 
 * Addition with immediate value
 */

SUB @d @s 
/*
 * Subtract @s from @d, result placed in @d
 */

SUBI @d [imm]
/*
 * Subtraction with immediate value
 */

// Logic
NEG @d
/*
 * Negate @d
 */

XOR @d @s
/*
 * Exclusive-or @d and @s, result placed in @d
 */

XORI @d [imm]
/*
 * Exclusive-or with immediate value
 */

FEY @a @b @c
/*
 * Feynman gate on @a, @b and @c
 */

FEYI @a [imm] @c
/*
 * Feynman gate with immediate value
 */

// Control Flow
BLT @d @s [off]
/*
 * Branch by offset [off] if @d is less
 * than @s
 */

BLTU @d @s [off]
/*
 * BLT on unsigned integers
 */

BGE @d @s [off]
/*
 * Branch by offset [off] if @d is greater
 * than or equal to @s
 */

BGEU @d @s [off]
/*
 * BGE on unsigned integers
 */

BEQ @d @s [off]
/*
 * Branch by offset [off] if @d is equal
 * to @s
 */

BNE @d @s [off]
/*
 * Branch by offset [off] if @d is not
 * equal to @s
 */

BEVN @d [off]
/*
 * Branch by offset [off] if @d is even
 */

BODD @d [off]
/*
 * Branch by offset [off] if @d is odd
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
MUL @d @s
/* 
 * Multiply @d with @s, result placed in @d
 */

MULI @d [imm]
/* 
 * Multiplication with immediate value
 */

DIV @d @s
/* 
 * Divide @d by @s, result placed in @d
 */

DIVI @d [imm]
/*
 * Division with immediate value
 */

/// B: Bitrotating
RL @d @s
/*
 * Rotate bits in @d left by value in @s
 */

RLI @d [imm]
/*
 * Rotate bits left with immediate value
 */

RR @d @s
/*
 * Rotate bits in @d right by value in @s
 */

RRI @d [imm]
/*
 * Rotate bits right with immediate value
 */

/// V: Vector Instructions
VADD @v0 @v1
/*
 * Add two registers, result placed in @v0
 */

VADDI @v [imm]
/* 
 * Addition with immediate value
 */

VSUB @v0 @v1 
/*
 * Subtract @v1 from @v0, result placed in @v0
 */

VSUBI @v [imm]
/*
 * Subtraction with immediate value
 */

VNEG @v
/*
 * Negate @v
 */

VXOR @v0 @v1
/*
 * Exclusive-or @v0 and @v1, result placed in @v0
 */

VXORI @v [imm]
/*
 * Exclusive-or with immediate value
 */

VFEY @v0 @v1 @v2
/*
 * Feynman gate on @v0, @v1 and @v2
 */

VFEYI @v0 [imm] @v1
/*
 * Feynman gate with immediate value
 */

VEXCH @v0 @v1
/*
 * Swap the vectors in @v0 and @v1
 */

VDEL @v
/*
 * Deletes the vector @v
 *
 * This instruction is included only if the
 * I extension is enabled
 */

VMUL @v0 @v1
/* 
 * Multiply @v0 with @v1, result placed in @v0
 *
 * This instruction is included only if the
 * M extension is enabled
 */

VMULI @v [imm]
/* 
 * Multiplication with immediate value
 *
 * This instruction is included only if the
 * M extension is enabled
 */

VDIV @v0 @v1
/* 
 * Divide @v0 by @v1, result placed in @v0
 *
 * This instruction is included only if the
 * M extension is enabled
 */

VDIVI @v [imm]
/*
 * Division with immediate value
 *
 * This instruction is included only if the
 * M extension is enabled
 */

VRL @v0 @v1
/*
 * Rotate bits in @v0 left by value in @v1
 *
 * This instruction is included only if the
 * B extension is enabled
 */

VRLI @v [imm]
/*
 * Rotate bits left with immediate value
 *
 * This instruction is included only if the
 * B extension is enabled
 */

VRR @v0 @v1
/*
 * Rotate bits in @v0 right by value in @v1
 *
 * This instruction is included only if the
 * B extension is enabled
 */

VRRI @v [imm]
/*
 * Rotate bits right with immediate value
 *
 * This instruction is included only if the
 * B extension is enabled
 */
