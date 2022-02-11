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
