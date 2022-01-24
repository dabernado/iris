#include <stdio.h>
#include "vm.h"

#define PROJECT_NAME "iris"

#define BIT_WIDTH 32

int main(int argc, char **argv)
{
    if(argc != 1) {
        printf("%s takes no arguments.\n", argv[0]);
        return 1;
    }
    printf("This is project %s.\n", PROJECT_NAME);
    return 0;
}

/* runs the program bytecode */
void init(int *prog, int mb)
{
  int pc = 0; // program counter
  int direction = 0; // direction bit
  int branch = 0; // branch register

  // initialize registers
  int regs[REGS_NUM];
  int v_regs[REGS_NUM][VECTOR_LEN];

  // initialize memory
  int mem_size = (mb * 1000000) / (BIT_WIDTH / 8);
  int *memory = malloc(sizeof(int) * mem_size);

  // initialize garbage stack to last 1/8th of memory
  int *garbage = memory + (mem_size - (mem_size / 8));

  // initialize r0 and r1
  regs[0] = 0;
  regs[1] = -1;
}
