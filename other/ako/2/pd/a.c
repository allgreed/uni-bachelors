#include <stdio.h>

int main()
{
    int x = 0b100101110101001101;
    int y;

    asm volatile
    (
        ".intel_syntax noprefix;"
        
        "xor eax, eax;"
        "mov edx, %0;"
        "xor ecx, ecx;"

    "loop:"
        "mov ebx, edx;"
        "shr edx, 1;"

        "and bl, 0b00000111;"
        "cmp bl, 0b00000101;"

        "jne noinc;"
        "inc eax;"

    "noinc:"
        "inc ecx;"
        "cmp ecx, 32;"
        "jnz loop;"

        "mov %0, eax;"

        ".att_syntax prefix;"
        :"=r" (y)
        :"r" (x)
        :"eax","ebx", "ecx", "edx"
    );

    printf("x = %i, y=%i\n", x, y);
}
