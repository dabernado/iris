# IRIS/Iris
**IRIS** (Isomorphic Reduced Instruction Set) is an ISA which describes a reversible computer that can execute computations both forwards and backwards. **Iris** is a virtual machine which implements IRIS on top of irreversible hardware.

A ["reversible computer"](https://en.wikipedia.org/wiki/Reversible_computing) is a computer in which every primitive operation is logically reversible; i.e., all information is preserved in computation, and all computer programs implement their inverse. For example, a compression algorithm written for a reversible computer comes with a decompression algorithm for free simply by running the algorithm in reverse, whereas an irreversible computer would require the programmer to implement the decompression algorithm separately. What's more, reversible computing provides a form of orthogonal persistence OOTB at no performance cost, allowing the user to "time-travel" to any past state of the computer at will. Reversible computing has exciting implications for software design, cybersecurity, quantum computing, biocomputing, and the environment. Theoretically, reversible silicon would run with nearly zero energy requirements, due to the conservation of entropy resulting in extremely low heat dissapation.

**IRIS** is a statically typed functional-concatenative language, and describes a novel computer architecture that diverges from the usual von Neumann-style RISC architecture in many important and interesting ways, while remaining simpler than most existing computer architectures. The core IRIS spec consists of just 16 isomorphisms, with optional extensions for arithmetic and I/O operations. Ideally, IRIS will be well suited not only for implementation in software, but also in hardware as a fully reversible microprocessor.

**Iris** is currently being written in Rust, and will mainly consist of a bytecode interpreter and Cranelift-based JIT compiler as a host for higher-level reversible languages. The main reason for building a virtual machine as the reference implementation of IRIS is to provide orthogonal persistence for irreversible operations such as I/O, in order to increase user control and conserve reversibility when possible.

## Roadmap
As of now, Iris is getting close to becoming a functioning prototype of an IRIS interpreter. Once that goal is reached, I will begin developing a compiler for a high-level reversible language which targets IRIS, and then will continue refining the two together in tandem as a single toolchain.

Right now, the next steps are as follows:
- write integration tests for bytecode interpretation
- design and implement a new data structure for code/data images
- implement and test IRIS image serialization/deserialization
- implement dynamic function lookup/dispatch
- JIT compilation via Cranelift for x86/arm64/risc-v
- implement foreign function interface
- ???
