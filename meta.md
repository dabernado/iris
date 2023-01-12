## Questions
### How to fix EXPF/COLF?
- needs some kind of type token to compare actual values when unifying
  - GC must walk through the actual values of the type and compare them
- introduce `type` primitive into type system?
  - could also be useful for defining functions
  - does it necessarily need to be a primitive type though?
- or put type info into fraction type?
  - arg in bytecode must still be polymorphic, even if it is a fraction

### What is the relationship between interaction combinators and IRIS?
- interaction combinators as IRIS implementation technique?
  - operators can't be cut; reduction must be reversible somehow

### How to optimize?
- parallelization
  - Iris could achieve HVM levels of speedup
- nat embedding
  - embedded directly into product cells
- localized sum tag
  - tag is always directly in front of wrapped value, instead of its pointer
- packed inductives
  - each inductive contains a field indicating the size of each datum
    - size 1 (nat, unit)
    - size 2 (product, sum(nat))
    - size 3 (sum(product))
    - size n (inductive); equivalent to 1 (each cell is a pointer to inductive)
- lazy evaluation?
- packed products?
  - not feasible without a tag byte
- type encoding
  - encode enough information in ops to determine size of current data type
  - could datum size be encoded in `FOLD/UNFOLD`, instead of as inductive field?

### Should access paths be implemented at IRIS layer?
- IRIS access path = inductive of (fst/snd + sum + index) tokens at each step
- useful for replacing pointer to current data type
- access path = type token?
  - can only be as such if root type is known, which is constantly changing
  - add field to IRIS machine for holding/manipulating type info?
- add ops for access paths?

### How to implement interaction (aka concurrency)?
- coinductive types?
  - implementation?
    - isomorphism?
      - unfold = straightforward, take next element in type
      - fold = requires seeding with pre-existing coinductive value
        - and iso(/arrow?) which generates/degenerates succeeding values?
    - case 0: access communication channel
      - successful creation of type dependent on thread's capabilities
      - specific types defined in machine with integer constants
        - FFI, hardware interfaces, thread creation, etc.
    - case 1: lazy evaluated data structure
      - execute async piece of code which generates next element
        - real numbers, infinite lists, etc.
  - whats the relationship between coinductive types and capabilities?
- interaction channels (standardized "IRIS Interactive System Interface"?)
  - 0: open/close new channels
  - 1: delete data (potentially irreversible)
  - 2: verify and/or execute bytecode
  - 3: storage I/O
  - 4: console I/O
  - 5: network I/O

### How to implement dynamic function dispatch?
- add coinductive capability for thread to inspect/reflect itself?

## TODO
- reimplement EXPF/COLF
- implement optimizations
