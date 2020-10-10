.intel_syntax noprefix
.global main

.text
    main:
        # input 
        mov eax, 6 # args to the function

        # process - input in eax
        push eax # fib-0
        call fib
        add esp, 4 # clean fib-0

        # output - processed data in eax
        mov ebx, offset msg
        push eax # printf-1
        push ebx # printf-0
        call printf
        add esp, 8 # clean printf-0, printf-1

        xor eax, eax # return 0
        ret

    fib:
        # input
        mov edx, [esp + 4] 

        # process - input in edx
        cmp edx, 1
        jg complex 
    simple:
        mov eax, 1
        ret
    complex:
        dec edx
        dec edx

        sub esp, 4 # tmp
        push edx # sub-fib-1
        inc edx
        push edx # sub-fib-0

        call fib 
        mov [esp + 8], eax # save intermediate to tmp
        add esp, 4 # clean sub-fib-0

        call fib 
        add esp, 4 # clean sub-fib-1

        pop ebx # clean tmp
        add eax, ebx
        
        ret

.data
msg:
.asciz "%d\n"
