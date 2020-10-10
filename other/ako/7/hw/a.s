.intel_syntax noprefix
.global main

.text
    main:
        push rbp
        mov rbp, rsp

        mov rdi, offset msg
        xor eax, eax
        call printf

        xor rax, rax # return 0

        mov rsp, rbp
        pop rbp
        ret

.data
msg:
.asciz "Hello, world!\n"
