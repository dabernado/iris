## Questions
### How to implement dynamic function dispatch?
- special `READ/WRITE` channel for type checking + executing functions
  - system may choose to disable and remove ability to execute live code
  - requires a type checker to be implemented in VM/hardware

### How to fix EXPF/COLF?
- needs some kind of type token to compare actual values when unifying
- introduce `type` primitive into type system?
  - could also be useful for defining functions
  - does it necessarily need to be a primitive type though?
- or put type info into fraction type?
  - arg in bytecode must still be polymorphic, even if it is a fraction

### How to implement packed inductives?
- each inductive contains a field indicating the size of each datum
  - size 1 (nat, unit)
  - size 2 (product, sum(nat))
  - size 3 (sum(product))
  - size n (inductive); equivalent to 1 (each cell is a pointer to inductive)
- could datum size be encoded in `FOLD/UNFOLD` ops?

### Should we implement access paths at IRIS layer?
- IRIS access path = inductive of (fst/snd + sum + index) tokens at each step
- useful for replacing pointer to current data type
- access path = type token?
  - can only be as such if root type is known, which is constantly changing
  - add field to IRIS machine for holding/manipulating type info?
- add ops for access paths?

### How to implement interaction?
- add arrows to wrap READ/WRITE?
  - arrow call context?
  - or should it be in sive runtime?
    - has potential to break pure reversible type checking in IRIS
- async/await for IO operations?
- `READ/WRITE` as delimited continuations?
  - negative types give concise definition
- static interaction vs dynamic interaction
  - static interaction = interaction with channel id as constant
  - dynamic interaction = interaction with channel id as op argument
    - capabilities?
- interaction channels (standardized "IRIS Interactive System Interface"?)
  - 0: open/close new channels
  - 1: delete data (potentially irreversible)
  - 2: verify and/or execute bytecode
  - 3: storage I/O
  - 4: console I/O
  - 5: network I/O

## TODO
- reimplement EXPF/COLF
- optimize op fetch
