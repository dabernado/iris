/*
 * machine.h - Iris VM implementation
 *
 */

#ifndef IRIS_VM
#define IRIS_VM

#include "opcodes.h"

/* eval - evaluates a single instruction
 *
 * This function takes a single IRIS instruction, decodes
 * it and then executes it on a given set of cpu and memory
 * registers
 */

void eval(
    int regs[REGS_NUM], // CPU registers
    int v_regs[REGS_NUM][VECTOR_LEN], // Vector registers
    int *m_regs, // Pointer to start of memory
    int *garbage, // Pointer to top of garbage stack within memory
    int mem_size // Memory size
    int *direction, // Direction bit - assumed to be 0
    int *branch, // Branch register
    int instr // Instruction to execute
    );

/* r_eval - evaluates a single instruction in reverse
 *
 * This function takes a single IRIS instruction, decodes
 * it and then executes its inverse operation on a given set of
 * cpu and memory registers
 */

void r_eval(
    int regs[REGS_NUM], // CPU registers
    int v_regs[REGS_NUM][VECTOR_LEN], // Vector registers
    int *m_regs, // Pointer to start of memory
    int *garbage, // Pointer to top of garbage stack within memory
    int mem_size // Memory size
    int *direction, // Direction bit - assumed to be 1
    int *branch, // Branch register
    int instr // Instruction to execute
    );

#endif
