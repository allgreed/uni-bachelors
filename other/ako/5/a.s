.intel_syntax noprefix
.global main

.text
    main:
        mov eax, offset msg

        push eax # printf-0
        call printf # call printf with one arg
        #pop eax # clean printf-1
        add esp, 4

        xor eax, eax # return 0
        ret

.data
msg:
.asciz "Hello, world\n"
