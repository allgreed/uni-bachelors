.intel_syntax noprefix
.global main

.text
    main:
        push ebp
        mov ebp, esp

        mov eax, 5
        push eax # printf-1
        mov eax, offset msg
        push eax # printf-0
        call printf

        xor eax, eax # return 0

        mov esp, ebp
        pop ebp
        ret

.data
msg:
.asciz "%d\n"
