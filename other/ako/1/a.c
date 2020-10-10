#include <stdio.h>

int main()
{
    int x = 10;
    int y = 2019;

    asm volatile
    (
        ".intel_syntax noprefix;"
        
        "imul %0, %1, 10;"

        ".att_syntax prefix;"
        :"=r" (y)
        :"r" (x)
    );

    printf("y = %d x = %d\n", y, x);
}
