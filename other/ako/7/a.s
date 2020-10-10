.intel_syntax noprefix
.global main

.text
    main:
        push rbp
        mov rbp, rsp
        mov r12, rdi

        xor rbx, rbx
    loop:
        lea rdi, [rsi + r12 * 8 - 16]
        call atoi

        add rbx, rax
        dec r12
        cmp r12, 1
        jnz loop

        mov rdi, offset msg
        mov rsi, rbx
        xor al, al
        call printf

        xor rax, rax # return 0

        mov rsp, rbp
        pop rbp
        ret

.data
msg:
.asciz "Suma to: %d\n"
