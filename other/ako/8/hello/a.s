.intel_syntax noprefix
.global _start
.text
_start:
    mov eax, 4
    mov ebx, 1 # stdin
    mov ecx, offset msg
    mov edx, 14 # chars in hello world
    int 0x80

    mov eax, 1
    mov ebx, 0
    int 0x80
.data
msg: .ascii "Hello, world!\n"
