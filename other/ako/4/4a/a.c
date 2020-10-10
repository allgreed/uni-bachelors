#include <stdio.h>

int main()
{
    char * s = "AbcDEFGJJJJG:.!";
    int y;

    asm volatile
    (
        ".intel_syntax noprefix;"
        
        "xor eax, eax;"
        "xor ebx, ebx;"
        "mov ecx, -1;"

        "znak:"
            "inc ecx;"
            "mov dl, [%1 + ecx];"

            "sub dl, 'A';"
            "cmp dl, 'Z' - 'A';"
            "setle bl;"

            "add eax, ebx;"

            "cmp dl, -'A';"  
            "jne znak;"

        "lea %0, [eax - 1];"

        ".att_syntax prefix;"
        :"=r" (y)
        :"r" (s)
        :"eax", "ebx", "ecx", "edx"
    );

    printf("y=%d\n", y);
}
