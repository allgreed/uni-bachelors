.intel_syntax noprefix
.global _start
.text
_start:
    mov eax, 1
    mov ebx, 99
    int 0x80
