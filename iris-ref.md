# Language Spec
### Types
```
0	:= empty type
1	:= unit type

?a <-> ?b := isomorphism type
(?a * ?b) := product type
(?a + ?b) := sum type
x.?a      := inductive type
-?a       := negative type
1/?a      := fraction type

nat  := x.(1 + x)
int  := (nat + nat)
bool := (1 + 1)
list := x.(1 + (?a * x))
```

### Functions
```
ID <-> ID         : ?a <-> ?a
 * Identity; does nothing

ZEROI <-> ZEROE   : ?a <-> (0 + ?a)
 * Introduce/eliminate sum variant of type 0

SWAPS <-> SWAPS   : (?a + ?b) <-> (?b + ?a)
 * Swap the two variant types' sides
 *
 * lc = # of types on the left-hand side of the sum
 * rc = # of types on the right-hand side of the sum

ASSRS <-> ASSLS   : ((?a + ?b) + ?c) <-> (?a + (?b + ?c))
 * Associate inner sum with types on the right or left

UNITI <-> UNITE   : ?a <-> (1 * ?a)
 * Introduce/eliminate product with unit type

SWAPP <-> SWAPP   : (?a * ?b) <-> (?b * ?a)
 * Swap the first and second values

ASSRP <-> ASSLP   : ((?a * ?b) * ?c) <-> (?a * (?b * ?c))
 * Associate inner product with types on the right or left

DIST <-> FACT     : ((?a + ?b) * ?c) <-> ((?a * ?c) + (?b * ?c))
 * Distribute inner sum over both product values/Factor inner
 * sum into first value
 *
 * lc = # of types on the left-hand side of the sum
 * rc = # of types on the right-hand side of the sum

FOLD <-> UFOLD    : a[x.?a] <-> x.?a
 * Fold/unfold value into/out of an inductive type

EXPN <-> COLN     : 0 <-> (-?a + ?a)
 * Reverse type sign and direction of execution
 *
 * n = number of types in each side of the sum

EXPF x <-> COLF x : 1 <-> (1/?a * ?a)
 * Allocate/deallocate new variable
 * x = index of fraction array to value being introduced
```

### Combinators
```
+{
 * Left hand sum combinator delimiter
 * l = number of instructions in left hand of combinator
 * r = number of instructions in right hand of combinator
 * d = index of last variant of the left hand of the type

}+
 * Right hand sum combinator delimiter
 * l = number of instructions in left hand of combinator
 * r = number of instructions in right hand of combinator
 * d = index of last variant of the left hand of the type

*{
 * First product combinator delimiter
 * j = jump to first instruction of second half of combinator

}*
 * Second product combinator delimiter
 * j = jump to last instruction of first half of combinator
```

### Control/Memory
```
CALL f <-> UNCALL f		: ?a <-> ?b
	where f: ?a <-> ?b
 * Invoke function forwards/backwards on datatype
 * f = name of invoked function, stored in a hash table with function address

READ c <-> WRITE c		: ?a <-> (?b * ?a)
 * Read/write data to/from external communication channel with ?a as
 * an optional argument; also can open new channels
 * c = id of communication channel
```

## Instruction Encoding
```
 * I-Type
 *
 * 31                                     0
 * [            imm            ] [ opcode ]
 *	            27b		               5b
 *
 * Instructions that do not contain additional information or
 * contain a constant value are represented by the I-Type
 * encoding. Most IRIS instructions are I-Type encoded.

 * S-Type
 *
 * 31                                     0
 * [0] [    rc    ] [    lc    ] [ opcode ]
 *  1b      13b	         13b	       5b
 *
 * The S-Type encoding is for certain sum type
 * instructions, which contain the number of variants
 * on the left and right hand sides of the type.
```
