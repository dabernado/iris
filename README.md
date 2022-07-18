# IRIS/Iris
**IRIS** (Isomorphic Reduced Instruction Set) is an ISA which describes a reversible computer that can execute computations both forwards and backwards. **Iris** is a virtual machine which implements IRIS on top of irreversible hardware.

A ["reversible computer"](https://en.wikipedia.org/wiki/Reversible_computing) is a computer in which every primitive operation is logically reversible; i.e., all information is preserved in computation, and computations can not only be executed forwards as usual, but can also be reversed to some previous state. For example, a compression algorithm written for a reversible computer comes with a decompression algorithm for free simply by running the algorithm in reverse, whereas an irreversible computer would require the programmer to implement the decompression algorithm separately. What's more, reversible computing provides a form of orthogonal persistence OOTB at no additional performance cost, allowing the user to recover any past state of the computer at will. Reversible computing has exciting implications for software design, cybersecurity, operating systems, quantum computing, and the environment. Theoretically, a physical reversible computer would run with nearly zero energy requirements, due to the conservation of entropy resulting in extremely low heat dissapation.

**IRIS** is a statically typed functional language, and describes a novel computer architecture that diverges from the usual von Neumann-style RISC architecture in many important and interesting ways. It allows for many high-level language features, such as higher-order functions and polymorphism, while still remaining simple and efficient as a low-level assembly language. The core IRIS spec consists of just 31 isomorphisms, with extensions planned for floating-point, I/O, and vector instructions. Ideally, IRIS will be well suited not only for implementation in software, but also in hardware in a fully reversible IRIS-based microprocessor.

**Iris** is currently being written in Rust, and will consist of both a bytecode interpreter and a JIT compiler for IRIS. The main reason for building a virtual machine as the reference implementation of IRIS is to allow programmers to modify the underlying implementation programs at will. For example, the programmer can indicate which of the functions in their program should be compiled to native machine code, which should be interpreted, and which should link to the Rust FFI. While running the program, they may decide to swap compiled functions out for their interpreted versions on the fly, and vice versa. Or, they could forego all that and compile into a standalone binary, as if they were developing with an ahead-of-time compiled language.

## Machine
An IRIS machine consists of the following components:
- a direction bit, which stores the direction of execution
- a context stack, which stores return addresses and other information about the current execution environment
- an instruction pointer, which points to the next instruction to execute
- a data pointer, which points to the block which contains the current data structure on which the processor is computing
- a size register, which points to the last register in the current block that contains data
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

**Left @i**; the context for processing the left value of a sum
- @i = pointer to first instruction of the right part of the sum combinator, which is compared to the instruction pointer before each instruction is executed. When the instruction pointer reaches this value, the Left context is popped off the stack and the instruction pointer moves down the program without executing anything until it reaches the end of the sum combinator.

**Right @i**; the context for processing the right value of a sum
- @i = pointer to first instruction of the right part of the sum combinator, which when executing in reverse behaves exactly like the instruction pointer in a Left context. When the instruction pointer reaches the end of the sum combinator, the Right context is popped off the stack.

**Indirect @a @b**; the context for following a pointer to some object
- @a = pointer to last object from which the process came from
- @b = pointer to current object

**Call @i @j**; the context for calling a function
- @i = pointer to the next instruction after the `CALL/UNCALL` or `EVAL/DEVAL` which prompted the function call
- @j = pointer to end of function; machine returns from context when instruction pointer is equal to this value

Context values contain a 3-bit tag which indicates what context it is, with the rest of the word divided up between whatever fields the context value holds (15/31-bit instruction pointer and 14/30-bit data pointer for product contexts, 29/61-bit instruction pointer for sum contexts).

## Memory
IRIS uses a register-based memory architecture, but instead of addressing registers directly, the processor instead operates directly on data types. Since type size and structure is known at compile time, most runtime type checking can be optimized away and necessary information for certain operations can be encoded in a single instruction. Additionally, each object cannot access outside data besides holding pointers to other objects.

### Data Types
#### Primitive Types
Besides the four algebraic data types, there are four primitive types in IRIS: `int`, `uint`, `1` (the unit type) and `0` (the empty type). Since there are no possible values of type `0`, it can only be used to define sum types in which only one of the two variants can be instantiated. The only value of type `1` is "()", or the unit value.

The numerical primtypes `int` and `uint` are actually represented by 15 and 31 bits on 32-bit and 64-bit platforms respectively, in order to be able to fit into product type values. IRIS can be extended with full-precision numerical types, but since they won't fit inside of a product cell, they can't be considered as primtypes.

Some other primtypes which can be added to IRIS via extension are arrays (`int[]`, `1[]`, etc.) provided by the vector extension, and the floating-point numerical type `float` provided by the floating-point extension.

#### Sum Types
Sum types are represented by an additional integer indicating which variant of the type the value is, along with the value itself. The amount of memory allocated is equal to the size of the largest variant of the type. For example, a value of 'left (right v)' of type `((int + int) + int)` would be represented as such:
```
r0	r1
[  1  ] [  v  ]
```

The assembler keeps track of where the variant value places the data in the type by remembering both the amount of variants in the left hand of the root sum, the total number of variants in the type, and an offset value which is set to 0 for operations on the root sum.

For example, a value "left (right (right v))" of type `((int + (int + int)) + (int + int))` would be represented with a variant value of 2, with the division value being 3 and a total value of 4. Say that the processor executed an `ASSLS` on the data structure, transforming it into the type `(int + ((int + int) + (int + int)))`; the division value becomes (total-division)+offset = 1 and the variant value stays the same, but since the division value has changed its interpretation now becomes "right (left (right v))".

Similarly, if a `SWAPS` was performed on the original value, turning it into type `((int + int) + (int + (int + int)))`, the division value would become "(total-division)+1+offset" which would evaluate to 2 in this case, and the variant value would become "new division + variant" which evaluates to 4, making the value "right (right (right v))".

What if an `+{ID + SWAPS}+` is executed on this result? The assembler needs to keep track of the division and total values of the inner type, while still executing with a variant value that specifies the variant in the context of the outer type. In this case, the processor would perform the `SWAPP` calculations with the inner type's division and total values as normal, but the offset would be equal to the division value of the outer type (in this case 2), which would then be added to get the resulting variant of the whole data structure.

The only sum operation which allocates/deallocates information is `ZEROI/ZEROE`, which must introduct a variant value to the type it is operating on. If the next register after the current value is empty, the the processor simply moves it over and introduces the variant value at its former starting location. If not, then the processor must reallocate the value along with its variant at the next free registers, and update the pointer to the value in the outer type.

#### Product Types
Product types are represented by a special cell type which can be divided into two parts; a field containing the first value, and a field for the second value. Each of these pointers can be further divided into a 1-bit field indicating whether or not the value is a pointer or a primtype, and a 15/31-bit field containing the pointer or primtype. For example, a value of type `(int * int)` on a 32-bit system would be represented as such:
```
0     1		 15 16    17	     31
[ 0 ] [    fst    ] [ 0 ] [    snd    ]
```

While a value of type `((int * int) * int)` would look like:
```
0     1		 15 16    17	     31
[ 1 ] [   *fst    ] [ 0 ] [    snd    ]
	    |
	    v
	    0     1	     15 16    17	 31
	    [ 0 ] [    fst    ] [ 0 ] [    snd    ]
```

The pointers of product cells can further be divided into a 1-bit field indicating whether or not the pointer points to a new memory block, with the rest of the bits containing the pointer value.

For operations such as `SWAPP` and `ASSRP/ASSLP`, execution is a simple matter of moving around the fields contained in the product cells. Operations such as `UNITI/UNITE` may have to allocate an extra register to contain the product cell, in which case the pointer to the current object has to be modfied in both the value that wraps it and in the current context value. `DIST/FACT` simply involves swapping the product cell and the sum value registers, and rewriting the first pointer in the product value to point to the value which was previously contained in the sum.

#### Fractional Types
Fractional types are represented as a pointer to a data structure somewhere in memory which the type was initialized to. Upon unification, the value and the fraction are compared by the processor and, if they are equivalent, the value is deallocated and the fraction changes back into the unit type.

For example, take the sequence of operations `*{ UNITI; *{ ID * EXPF; COLF }*; * ID }*` which takes a starting type of `(1 * 1)` and transforms it into `((1 * (1/(int * int), (int * int))) * 1)` before collapsing the fraction and its value. The CPU executes these operations in order:
1. The root product combinator is entered, and the processor begins executing on the first value
2. `UNITI` begins executing; a new product cell containing of type `(1 * 1)` is allocated and the first value of the current product cell is updated to point to the new value. The data pointer is updated to point to the new product cell. The size register is updated and `UNITI` finishes
3. The next product combinator is entered, and `ID` is executed on the first value of our newly created product cell
4. The second part of the combinator is entered, and `EXPF` begins execution; a new product cell is allocated containing a pointer to the fractional value and a pointer to where the new value will be allocated, and the second field of the previous product cell is updated to point to the new cell
5. The CPU allocates the new value into the location, and `EXPF` finishes executing
6. `COLF` begins execution; all the primtypes contained in the fraction value and the allocated value are compared
7. If the two values are unequal, an exception is thrown; otherwise, the value is deallocated and the fraction pointer is turned back into the unit type, and `COLF` finishes execution
8. The inner product combinator is exited, and the CPU returns to the root product cell after transitioning from the contexts that were previously pushed onto the stack
9. `ID` is executed on the second value, and the root product combinator is exited

#### Negative Types
The negative type isomorphism `EXPN/COLN` is interesting because it is the only isomorphism in IRIS which is partial between the forward and backward evaluators. `EXPN` can only be performed when executing backwards, and `COLN` only when executing forwards. Both of these instructions flip the direction bit and the direction of execution when performed.

Based on the sum variant of the incoming value, `EXPN/COLN` will negate its type. For example, if a value of "right v" enters `COLN`, it will be transformed into "left -v" and the CPU will begin executing in reverse. Otherwise, if a value of "left -v" enters `COLN`, it will become "right v" and the CPU will also begin executing in reverse. The 'vice versa' holds for `EXPN`, which accepts incoming values in reverse and switches the CPU to begin executing forwards.

Negative types can be used to implement novel control structures such as recursion, delimited continuations, coroutines, and others. Looping in IRIS is implemented with the additive trace, which itself is implemented using negative types. For example, given a function `f` of type `(?a + ?b) + (?a + ?c)`, a trace over `f` can be constructed with the following function:

```
traceadd: ?b <-> ?c
    START
    ZEROI
    +{
    	ID
    	+
    	EXPN
    }+
    +{
    	CALL f
	+
	ID
    }+
    +{
    	ID
	+
	COLN
    }+
    ZEROE
    RETURN
```

#### Functions
Functions as a datatype in IRIS are represented by a pointer to the function's start in the program, which also includes an offset value which indicates the end of the function. When `EVAL` is called on a data value with a function pointer, the instruction pointer is updated with the information in the function pointer and the function is executed on the data value, returning after the function is finished executing.

#### Polymorphism
Polymorphic functions can be written IRIS, which are instantiated as overloaded versions of the functions at compile time.

## Exceptions
Despite the strong typing of IRIS allowing for the elimination of many runtime errors that are possible in other assembly languages, there are still some scenarios in which the attempted execution of certain instructions may result in the CPU throwing an exception. Some of the most common are:

`ERRFRAC` - failed unification of a fraction and a value

`ERRALLOC` - not enough space left to perform allocation

`ERRZERO` - encountered a function which expects a value of type 0

`ERRISIZE` - integer overflow

`ERRDIV0` - attempted integer division by 0

`ERRMUL0` - attempted integer multiplication by 0 (irreversible)

`ERREXT` - code contains instructions/datatypes from an extension not supported by the current environment

`ERRCXT` - attempted invalid context trainsition

`ERRSYSC` - attempted invalid syscall

`ERRTYPE` - types of attempted operation and data do not match
