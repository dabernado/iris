# IRIS/Iris
**IRIS** (Isomorphic Reduced Instruction Set) is an ISA which describes a reversible computer that can execute computations both forwards and backwards. **Iris** is a virtual machine which implements IRIS on top of irreversible hardware.

A ["reversible computer"](https://en.wikipedia.org/wiki/Reversible_computing) is a computer that is logically reversible; i.e., all information in data is preserved, and all computer programs implement their inverse. For example, a compression algorithm written for a reversible computer would come with its decompression algorithm for free, simply by running the algorithm in reverse. Also, it inherently persists data at no cost to performance, allowing the user to "time-travel" to any past state of the computer at will. Due to Landauer's principle, a reversible microprocessor would require much less energy than a modern one to run (in the order of magnitudes).

**IRIS** is a strongly typed concatenative language, and describes a novel computer architecture that diverges from typical computer architectures in many important and interesting ways, while remaining simpler than most existing ones. The core IRIS spec consists of just 15 reversible functions, and is Interaction Machine-complete - meaning it is even more expressive than a Turing Machine, due to its notion of _interaction_. Ideally, IRIS will be well suited not only for implementation in software, but also in hardware as a fully reversible microprocessor.

**Iris** is currently being written in Rust, and will mainly consist of a bytecode interpreter and Cranelift-based JIT compiler. The main reason for building a virtual machine as the reference implementation of IRIS is to provide orthogonal persistence for irreversible I/O operations, in order to increase user control and conserve reversibility when possible.

## Roadmap
As of now, Iris is getting close to becoming a functioning prototype of an IRIS interpreter. Once that goal is reached, I will begin developing a compiler for a higher-level reversible language which targets IRIS, and then will continue refining the two together in tandem as a single toolchain.

Right now, the next steps are as follows:
- write integration tests for bytecode interpretation
- switch out data types for capnproto types?
  - ideally, we want to load images without converting to internal data types
- design and implement a new data structure for code/data images
  - implement and test IRIS image serialization/deserialization
- implement dynamic function lookup/dispatch
- JIT compilation via Cranelift for x86/arm64/risc-v
- implement foreign function interface
- ???
