.intel_syntax noprefix
.global main

.text
main:
    mov eax, 42
    push eax
    call f
    add esp, 4

    push eax
    mov eax, offset msg
    push eax
    call printf
    add esp, 8

    mov eax, 0
    ret

f:
    push ebp
    mov ebp, esp

    mov eax, [ebp+8]
    cmp eax, 2
    jae skok
    mov eax, 1
    jmp koniec
skok:
    dec eax

    push eax
    call f
    add esp, 4

    push eax

    mov eax, [ebp+8]
    sub eax, 2

    push eax
    call f
    add esp, 4
    add eax, eax

    pop ebx
    add eax, ebx

koniec:
    mov esp, ebp
    pop ebp
    ret

.data
msg: .asciz "Wynik=%i\n"
