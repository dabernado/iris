#include <stdio.h>
#include "machine.h"

#define PROJECT_NAME "iris"

#define BIT_WIDTH 32

int main(int argc, char **argv)
{
    if(argc != 1) {
        printf("%s takes no arguments.\n", argv[0]);
        return 1;
    }

    int prog = 0;
    int prog_size = 1;
    
    init_vm(&prog, prog_size, 4);

    return 0;
}
