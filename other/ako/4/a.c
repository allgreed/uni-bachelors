#include <stdio.h>

int main()
{
    char * s = "Abcdef";
    int y;

    asm volatile
    (
        ".intel_syntax noprefix;"
        
        "mov eax, -1;"

        "znak:"
            "inc eax;" 

            "mov dl, [%1 + eax];"
            "cmp dl, 0;"  
            "jne znak;"

        "mov %0, eax;"

        ".att_syntax prefix;"
        :"=r" (y)
        :"r" (s)
        :"eax", "edx"
    );

    printf("y=%d\n", y);
}
