#include <stdio.h>

int main()
{
    int x = 0x75;
    int y;

    asm volatile
    (
        ".intel_syntax noprefix;"
        
        "xor eax, eax;"
        "mov edx, %0;"
        "xor ecx, ecx;"

    "skok:"
        "shl edx, 1;"
        "adc eax, 0;"

        "inc ecx;"
        "cmp ecx, 32;"
        "jnz skok;"

        "mov %0, eax;"

        ".att_syntax prefix;"
        :"=r" (y)
        :"r" (x)
        :"eax", "ecx", "edx"
    );

    printf("x = %i, y=%i\n", x, y);
}
