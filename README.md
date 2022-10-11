# IRIS/Iris
**IRIS** (Isomorphic Reduced Instruction Set) is an ISA which describes a reversible computer that can execute computations both forwards and backwards. **Iris** is a virtual machine which implements IRIS on top of irreversible hardware.

A ["reversible computer"](https://en.wikipedia.org/wiki/Reversible_computing) is a computer in which every primitive operation is logically reversible; i.e., all information is preserved in computation, and computations can not only be executed forwards as usual, but can also be reversed to some previous state. For example, a compression algorithm written for a reversible computer comes with a decompression algorithm for free simply by running the algorithm in reverse, whereas an irreversible computer would require the programmer to implement the decompression algorithm separately. What's more, reversible computing provides a form of orthogonal persistence OOTB at no additional performance cost, allowing the user to recover any past state of the computer at will. Reversible computing has exciting implications for software design, cybersecurity, operating systems, quantum computing, and the environment. Theoretically, a physical reversible computer would run with nearly zero energy requirements, due to the conservation of entropy resulting in extremely low heat dissapation.

**IRIS** is a statically typed functional language, and describes a novel computer architecture that diverges from the usual von Neumann-style RISC architecture in many important and interesting ways, while remaining simpler and more efficient than most existing computer architectures. The core IRIS spec consists of just 25 isomorphisms, with possible extensions for floating-point, I/O, and vector instructions. Ideally, IRIS will be well suited not only for implementation in software, but also in hardware in a fully reversible IRIS-based microprocessor.

**Iris** is currently being written in Rust, and will mainly consist of a bytecode interpreter as a compilation target for higher-level reversible languages. The main reason for building a virtual machine as the reference implementation of IRIS is to capture the histories of irreversible operations such as I/O, for further reversibility and user control.

## Roadmap
As of now, Iris is getting close to becoming a functioning prototype of an IRIS interpreter. Once that goal is reached, I will begin developing a compiler for a high-level reversible language which targets IRIS, and then will continue refining the two together in tandem as a single toolchain.

Right now, the next steps are as follows:
- write integration tests for bytecode interpretation
- design and implement a new data structure for code/data images
- implement and test IRIS image serialization/deserialization
- implement foreign function interface
- design and implement vector ops extension
- ???
