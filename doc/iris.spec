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

// Bitrotating
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

// Control Flow
BGEZ @d [off]
/*
 * Branch by offset [off] if @d is greater
 * than or equal to 0
 */

BGZ @d [off]
/*
 * Branch by offset [off] if @d is greater
 * than 0
 */

BLEZ @d [off]
/*
 * Branch by offset [off] if @d is less
 * than or equal to 0
 */

BLZ @d [off]
/*
 * Branch by offset [off] if @d is less
 * than 0
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
