.intel_syntax noprefix
.global _start
.text
_start:
    sub esp, 20 # make room on stack for writing contents

    mov eax, 3  
    mov ebx, 0
    mov ecx, esp
    mov edx, 20
    int 0x80

    mov ebp, -1
counter:
    inc ebp
    mov dl, [esp + ebp]
    cmp dl, '\n'
        #TODO: Check for 255
    jne counter
    inc ebp

    mov eax, 4
    mov ebx, 1 # stdout
    mov ecx, esp # input
    mov edx, ebp # chars in input
    int 0x80

    add esp, 20 # clean bytes from stack  

    mov eax, 1
    mov ebx, 0
    int 0x80
