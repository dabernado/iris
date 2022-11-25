## IRIS Machine
An IRIS machine or thread consists of the following components:
- a direction bit, which stores the direction of execution
- a context stack, which stores return addresses and other information about the current execution environment
- an instruction pointer, which points to the next instruction to execute
- a data pointer, which points to the current data structure on which the processor is computing
- a program and associated data structure, which includes:
    - the program bytecode
    - the data structure which the top-level isomorphism of the program operates on

### Context Stack
The context stack tells an IRIS computer what they are doing, and stores necessary information for switching contexts. There are seven possible values which can inhabit the context stack:

**!**; the terminal symbol, simply represented by a value of 0, with the first bit of a context value indicating whether it is terminal or non-terminal

**Fst @i @a @b**; the context for processing the first value of a product
- @i = pointer to first instruction of the second part of the product combinator, which is compared to the instruction pointer before each instruction is executed. When the instruction pointer reaches this value, the Fst context is popped off the stack and a Snd context is pushed
- @a = pointer to the second value of the product, which is pushed onto the data stack once the current context is finished
- @b = pointer to root product

**Snd @i @a @b**; the context for processing the second value of a product
- @i = pointer to last instruction of the first part of the product combinator, which when executing in reverse behaves exactly like the instruction pointer in the Fst context. When the instruction pointer reaches the end of the product combinator, the Snd context is popped off the stack.
- @a = pointer to the first value of the product, which is pushed onto the data stack once the current context is finished and the current value is popped
- @b = pointer to root product

**Left @i @n**; the context for processing the left value of a sum
- @i = pointer to first instruction (or last, if executing backwards) of the right part of the sum combinator, which is compared to the instruction pointer before each instruction is executed. When the instruction pointer reaches this value, the Left context is popped off the stack and the instruction pointer moves down the program without executing anything until it reaches the end of the sum combinator.
- @n = # of instructions to jump towards the instruction after the closing SUMC of the combinator, if executing forwards

**Right @i @n**; the context for processing the right value of a sum
- @i = pointer to last instruction (or first, if executing backwards) of the left part of the sum combinator, which when executing in reverse behaves exactly like the instruction pointer in a Left context. When the instruction pointer reaches the end of the sum combinator, the Right context is popped off the stack.
- @n = # of instructions to jump backwards to the instruction before the opening SUMC of the combinator, if executing in reverse

**Call 0|1 @s @e @i**; the context for calling a function
- 0|1 = indicates if direction of execution should be inverted for this function
- @s = index to start of the function being called
- @e = index to end of the function being called
- @i = index to next instruction after the `CALL` which prompted the call

Context values contain a 3-bit tag which indicates what context it is, with the rest of the word divided up between whatever fields the context value holds (15/31-bit instruction pointer and 14/30-bit data pointer for product contexts, 29/61-bit instruction pointer for sum contexts).

### Memory
IRIS uses a register-based memory architecture, but instead of addressing registers directly, the processor instead operates directly on data types. Since type size and structure is known at compile time, most runtime type checking can be optimized away and necessary information for certain operations can be encoded in a single instruction. Additionally, each object cannot access outside data besides holding pointers to other objects.

### Data Types
#### Primitive Types

#### Sum Types

#### Product Types

#### Fractional Types

#### Negative Types
The negative type isomorphism `EXPN/COLN` is interesting because it is the only isomorphism in IRIS which is partial between the forward and backward evaluators. `EXPN` can only be performed when executing backwards, and `COLN` only when executing forwards. Both of these instructions flip the direction bit and the direction of execution when performed.

Based on the sum variant of the incoming value, `EXPN/COLN` will negate its type. For example, if a value of "right v" enters `COLN`, it will be transformed into "left -v" and the CPU will begin executing in reverse. Otherwise, if a value of "left -v" enters `COLN`, it will become "right v" and the CPU will also begin executing in reverse. The 'vice versa' holds for `EXPN`, which accepts incoming values in reverse and switches the CPU to begin executing forwards.

Negative types can be used to implement novel control structures such as recursion, delimited continuations, coroutines, and others. Looping in IRIS is implemented with the additive trace, which itself is implemented using negative types. For example, given a function `f` of type `(?a + ?b) + (?a + ?c)`, a trace over `f` can be constructed with the following function:

```
fn: (?a + ?b) <-> (?a + ?c)

trace_fn: ?b <-> ?c
trace_fn =
  zeroi                          // 0 + ?b
  (+ expn:?a, id)                // (-?a + ?a) + ?b
  assrs                          // -?a + (?a + ?b)
  (+ id, fn)                     // -?a + (?a + ?c)
  assls                          // (-?a + ?a) + ?c
  (+ coln:?a, (add5 setFlag))    // 0 + ?c
  zeroe.                         // ?c
```

#### Functions
Functions are defined as a special case of an inductive type which contains the opcode type `nat * ?a`, where depending on the opcode in the first cell, the second cell is typed according to what the op requires. Functions can also be folded/unfolded to access the individual operations within.

### Interaction

### Exceptions
Despite the strong typing of IRIS allowing for the elimination of many runtime errors that are possible in other assembly languages, there are still some scenarios in which the attempted execution of certain instructions may result in the CPU throwing an exception. Some of the most common are:

`ERRFRAC` - failed unification of a fraction and a value

`ERRALLOC` - not enough space left to perform allocation

`ERRZERO` - encountered a function which expects a value of type 0

`ERRSIZE` - nat overflow

`ERRCXT` - attempted invalid context transition

`ERRIO` - read/write error
