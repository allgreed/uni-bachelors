.intel_syntax noprefix
.global main

.text
    main:
        push rbp
        mov rbp, rsp
        mov rsi, [rsi]

        mov rdi, offset msg
        xor al, al
        call printf

        xor rax, rax # return 0

        mov rsp, rbp
        pop rbp
        ret

.data
msg:
.asciz "Hello %s\n"
