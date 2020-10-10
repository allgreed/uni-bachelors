	.file	"a.c"
	.text
.Ltext0:
	.section	.rodata
.LC0:
	.string	"Abcdef"
.LC1:
	.string	"y=%d\n"
	.text
	.globl	main
	.type	main, @function
main:
.LFB0:
	.file 1 "a.c"
	.loc 1 4 0
	.cfi_startproc
	leal	4(%esp), %ecx
	.cfi_def_cfa 1, 0
	andl	$-16, %esp
	pushl	-4(%ecx)
	pushl	%ebp
	.cfi_escape 0x10,0x5,0x2,0x75,0
	movl	%esp, %ebp
	pushl	%ebx
	pushl	%ecx
	.cfi_escape 0xf,0x3,0x75,0x78,0x6
	.cfi_escape 0x10,0x3,0x2,0x75,0x7c
	subl	$16, %esp
	call	__x86.get_pc_thunk.cx
	addl	$_GLOBAL_OFFSET_TABLE_, %ecx
	.loc 1 5 0
	leal	.LC0@GOTOFF(%ecx), %eax
	movl	%eax, -12(%ebp)
	.loc 1 8 0
	movl	-12(%ebp), %ebx
#APP
# 8 "a.c" 1
	.intel_syntax noprefix;mov eax, -1;znak:inc eax;mov dl, [%ebx + eax];cmp dl, 0;jne znak;mov %ebx, eax;.att_syntax prefix;
# 0 "" 2
#NO_APP
	movl	%ebx, -16(%ebp)
	.loc 1 29 0
	subl	$8, %esp
	pushl	-16(%ebp)
	leal	.LC1@GOTOFF(%ecx), %eax
	pushl	%eax
	movl	%ecx, %ebx
	call	printf@PLT
	addl	$16, %esp
	movl	$0, %eax
	.loc 1 30 0
	leal	-8(%ebp), %esp
	popl	%ecx
	.cfi_restore 1
	.cfi_def_cfa 1, 0
	popl	%ebx
	.cfi_restore 3
	popl	%ebp
	.cfi_restore 5
	leal	-4(%ecx), %esp
	.cfi_def_cfa 4, 4
	ret
	.cfi_endproc
.LFE0:
	.size	main, .-main
	.section	.text.__x86.get_pc_thunk.cx,"axG",@progbits,__x86.get_pc_thunk.cx,comdat
	.globl	__x86.get_pc_thunk.cx
	.hidden	__x86.get_pc_thunk.cx
	.type	__x86.get_pc_thunk.cx, @function
__x86.get_pc_thunk.cx:
.LFB1:
	.cfi_startproc
	movl	(%esp), %ecx
	ret
	.cfi_endproc
.LFE1:
	.text
.Letext0:
	.file 2 "/usr/lib/gcc/x86_64-linux-gnu/6/include/stddef.h"
	.file 3 "/usr/include/bits/types.h"
	.file 4 "/usr/include/libio.h"
	.file 5 "/usr/include/stdio.h"
	.file 6 "/usr/include/bits/sys_errlist.h"
	.section	.debug_info,"",@progbits
.Ldebug_info0:
	.long	0x33f
	.value	0x4
	.long	.Ldebug_abbrev0
	.byte	0x4
	.uleb128 0x1
	.long	.LASF57
	.byte	0xc
	.string	"a.c"
	.long	.LASF58
	.long	.Ltext0
	.long	.Letext0-.Ltext0
	.long	.Ldebug_line0
	.uleb128 0x2
	.long	.LASF8
	.byte	0x2
	.byte	0xd8
	.long	0x30
	.uleb128 0x3
	.byte	0x4
	.byte	0x7
	.long	.LASF0
	.uleb128 0x3
	.byte	0x1
	.byte	0x8
	.long	.LASF1
	.uleb128 0x3
	.byte	0x2
	.byte	0x7
	.long	.LASF2
	.uleb128 0x3
	.byte	0x4
	.byte	0x7
	.long	.LASF3
	.uleb128 0x3
	.byte	0x1
	.byte	0x6
	.long	.LASF4
	.uleb128 0x3
	.byte	0x2
	.byte	0x5
	.long	.LASF5
	.uleb128 0x4
	.byte	0x4
	.byte	0x5
	.string	"int"
	.uleb128 0x3
	.byte	0x8
	.byte	0x5
	.long	.LASF6
	.uleb128 0x3
	.byte	0x8
	.byte	0x7
	.long	.LASF7
	.uleb128 0x2
	.long	.LASF9
	.byte	0x3
	.byte	0x37
	.long	0x61
	.uleb128 0x2
	.long	.LASF10
	.byte	0x3
	.byte	0x83
	.long	0x85
	.uleb128 0x3
	.byte	0x4
	.byte	0x5
	.long	.LASF11
	.uleb128 0x2
	.long	.LASF12
	.byte	0x3
	.byte	0x84
	.long	0x6f
	.uleb128 0x3
	.byte	0x4
	.byte	0x7
	.long	.LASF13
	.uleb128 0x5
	.byte	0x4
	.uleb128 0x6
	.byte	0x4
	.long	0xa6
	.uleb128 0x3
	.byte	0x1
	.byte	0x6
	.long	.LASF14
	.uleb128 0x7
	.long	0xa6
	.uleb128 0x8
	.long	.LASF44
	.byte	0x94
	.byte	0x4
	.byte	0xf1
	.long	0x22f
	.uleb128 0x9
	.long	.LASF15
	.byte	0x4
	.byte	0xf2
	.long	0x5a
	.byte	0
	.uleb128 0x9
	.long	.LASF16
	.byte	0x4
	.byte	0xf7
	.long	0xa0
	.byte	0x4
	.uleb128 0x9
	.long	.LASF17
	.byte	0x4
	.byte	0xf8
	.long	0xa0
	.byte	0x8
	.uleb128 0x9
	.long	.LASF18
	.byte	0x4
	.byte	0xf9
	.long	0xa0
	.byte	0xc
	.uleb128 0x9
	.long	.LASF19
	.byte	0x4
	.byte	0xfa
	.long	0xa0
	.byte	0x10
	.uleb128 0x9
	.long	.LASF20
	.byte	0x4
	.byte	0xfb
	.long	0xa0
	.byte	0x14
	.uleb128 0x9
	.long	.LASF21
	.byte	0x4
	.byte	0xfc
	.long	0xa0
	.byte	0x18
	.uleb128 0x9
	.long	.LASF22
	.byte	0x4
	.byte	0xfd
	.long	0xa0
	.byte	0x1c
	.uleb128 0x9
	.long	.LASF23
	.byte	0x4
	.byte	0xfe
	.long	0xa0
	.byte	0x20
	.uleb128 0xa
	.long	.LASF24
	.byte	0x4
	.value	0x100
	.long	0xa0
	.byte	0x24
	.uleb128 0xa
	.long	.LASF25
	.byte	0x4
	.value	0x101
	.long	0xa0
	.byte	0x28
	.uleb128 0xa
	.long	.LASF26
	.byte	0x4
	.value	0x102
	.long	0xa0
	.byte	0x2c
	.uleb128 0xa
	.long	.LASF27
	.byte	0x4
	.value	0x104
	.long	0x267
	.byte	0x30
	.uleb128 0xa
	.long	.LASF28
	.byte	0x4
	.value	0x106
	.long	0x26d
	.byte	0x34
	.uleb128 0xa
	.long	.LASF29
	.byte	0x4
	.value	0x108
	.long	0x5a
	.byte	0x38
	.uleb128 0xa
	.long	.LASF30
	.byte	0x4
	.value	0x10c
	.long	0x5a
	.byte	0x3c
	.uleb128 0xa
	.long	.LASF31
	.byte	0x4
	.value	0x10e
	.long	0x7a
	.byte	0x40
	.uleb128 0xa
	.long	.LASF32
	.byte	0x4
	.value	0x112
	.long	0x3e
	.byte	0x44
	.uleb128 0xa
	.long	.LASF33
	.byte	0x4
	.value	0x113
	.long	0x4c
	.byte	0x46
	.uleb128 0xa
	.long	.LASF34
	.byte	0x4
	.value	0x114
	.long	0x273
	.byte	0x47
	.uleb128 0xa
	.long	.LASF35
	.byte	0x4
	.value	0x118
	.long	0x283
	.byte	0x48
	.uleb128 0xa
	.long	.LASF36
	.byte	0x4
	.value	0x121
	.long	0x8c
	.byte	0x4c
	.uleb128 0xa
	.long	.LASF37
	.byte	0x4
	.value	0x129
	.long	0x9e
	.byte	0x54
	.uleb128 0xa
	.long	.LASF38
	.byte	0x4
	.value	0x12a
	.long	0x9e
	.byte	0x58
	.uleb128 0xa
	.long	.LASF39
	.byte	0x4
	.value	0x12b
	.long	0x9e
	.byte	0x5c
	.uleb128 0xa
	.long	.LASF40
	.byte	0x4
	.value	0x12c
	.long	0x9e
	.byte	0x60
	.uleb128 0xa
	.long	.LASF41
	.byte	0x4
	.value	0x12e
	.long	0x25
	.byte	0x64
	.uleb128 0xa
	.long	.LASF42
	.byte	0x4
	.value	0x12f
	.long	0x5a
	.byte	0x68
	.uleb128 0xa
	.long	.LASF43
	.byte	0x4
	.value	0x131
	.long	0x289
	.byte	0x6c
	.byte	0
	.uleb128 0xb
	.long	.LASF59
	.byte	0x4
	.byte	0x96
	.uleb128 0x8
	.long	.LASF45
	.byte	0xc
	.byte	0x4
	.byte	0x9c
	.long	0x267
	.uleb128 0x9
	.long	.LASF46
	.byte	0x4
	.byte	0x9d
	.long	0x267
	.byte	0
	.uleb128 0x9
	.long	.LASF47
	.byte	0x4
	.byte	0x9e
	.long	0x26d
	.byte	0x4
	.uleb128 0x9
	.long	.LASF48
	.byte	0x4
	.byte	0xa2
	.long	0x5a
	.byte	0x8
	.byte	0
	.uleb128 0x6
	.byte	0x4
	.long	0x236
	.uleb128 0x6
	.byte	0x4
	.long	0xb2
	.uleb128 0xc
	.long	0xa6
	.long	0x283
	.uleb128 0xd
	.long	0x97
	.byte	0
	.byte	0
	.uleb128 0x6
	.byte	0x4
	.long	0x22f
	.uleb128 0xc
	.long	0xa6
	.long	0x299
	.uleb128 0xd
	.long	0x97
	.byte	0x27
	.byte	0
	.uleb128 0xe
	.long	.LASF60
	.uleb128 0xf
	.long	.LASF49
	.byte	0x4
	.value	0x13b
	.long	0x299
	.uleb128 0xf
	.long	.LASF50
	.byte	0x4
	.value	0x13c
	.long	0x299
	.uleb128 0xf
	.long	.LASF51
	.byte	0x4
	.value	0x13d
	.long	0x299
	.uleb128 0x6
	.byte	0x4
	.long	0xad
	.uleb128 0x7
	.long	0x2c2
	.uleb128 0x10
	.long	.LASF52
	.byte	0x5
	.byte	0xaa
	.long	0x26d
	.uleb128 0x10
	.long	.LASF53
	.byte	0x5
	.byte	0xab
	.long	0x26d
	.uleb128 0x10
	.long	.LASF54
	.byte	0x5
	.byte	0xac
	.long	0x26d
	.uleb128 0x10
	.long	.LASF55
	.byte	0x6
	.byte	0x1a
	.long	0x5a
	.uleb128 0xc
	.long	0x2c8
	.long	0x304
	.uleb128 0x11
	.byte	0
	.uleb128 0x7
	.long	0x2f9
	.uleb128 0x10
	.long	.LASF56
	.byte	0x6
	.byte	0x1b
	.long	0x304
	.uleb128 0x12
	.long	.LASF61
	.byte	0x1
	.byte	0x3
	.long	0x5a
	.long	.LFB0
	.long	.LFE0-.LFB0
	.uleb128 0x1
	.byte	0x9c
	.uleb128 0x13
	.string	"s"
	.byte	0x1
	.byte	0x5
	.long	0xa0
	.uleb128 0x2
	.byte	0x75
	.sleb128 -12
	.uleb128 0x13
	.string	"y"
	.byte	0x1
	.byte	0x6
	.long	0x5a
	.uleb128 0x2
	.byte	0x75
	.sleb128 -16
	.byte	0
	.byte	0
	.section	.debug_abbrev,"",@progbits
.Ldebug_abbrev0:
	.uleb128 0x1
	.uleb128 0x11
	.byte	0x1
	.uleb128 0x25
	.uleb128 0xe
	.uleb128 0x13
	.uleb128 0xb
	.uleb128 0x3
	.uleb128 0x8
	.uleb128 0x1b
	.uleb128 0xe
	.uleb128 0x11
	.uleb128 0x1
	.uleb128 0x12
	.uleb128 0x6
	.uleb128 0x10
	.uleb128 0x17
	.byte	0
	.byte	0
	.uleb128 0x2
	.uleb128 0x16
	.byte	0
	.uleb128 0x3
	.uleb128 0xe
	.uleb128 0x3a
	.uleb128 0xb
	.uleb128 0x3b
	.uleb128 0xb
	.uleb128 0x49
	.uleb128 0x13
	.byte	0
	.byte	0
	.uleb128 0x3
	.uleb128 0x24
	.byte	0
	.uleb128 0xb
	.uleb128 0xb
	.uleb128 0x3e
	.uleb128 0xb
	.uleb128 0x3
	.uleb128 0xe
	.byte	0
	.byte	0
	.uleb128 0x4
	.uleb128 0x24
	.byte	0
	.uleb128 0xb
	.uleb128 0xb
	.uleb128 0x3e
	.uleb128 0xb
	.uleb128 0x3
	.uleb128 0x8
	.byte	0
	.byte	0
	.uleb128 0x5
	.uleb128 0xf
	.byte	0
	.uleb128 0xb
	.uleb128 0xb
	.byte	0
	.byte	0
	.uleb128 0x6
	.uleb128 0xf
	.byte	0
	.uleb128 0xb
	.uleb128 0xb
	.uleb128 0x49
	.uleb128 0x13
	.byte	0
	.byte	0
	.uleb128 0x7
	.uleb128 0x26
	.byte	0
	.uleb128 0x49
	.uleb128 0x13
	.byte	0
	.byte	0
	.uleb128 0x8
	.uleb128 0x13
	.byte	0x1
	.uleb128 0x3
	.uleb128 0xe
	.uleb128 0xb
	.uleb128 0xb
	.uleb128 0x3a
	.uleb128 0xb
	.uleb128 0x3b
	.uleb128 0xb
	.uleb128 0x1
	.uleb128 0x13
	.byte	0
	.byte	0
	.uleb128 0x9
	.uleb128 0xd
	.byte	0
	.uleb128 0x3
	.uleb128 0xe
	.uleb128 0x3a
	.uleb128 0xb
	.uleb128 0x3b
	.uleb128 0xb
	.uleb128 0x49
	.uleb128 0x13
	.uleb128 0x38
	.uleb128 0xb
	.byte	0
	.byte	0
	.uleb128 0xa
	.uleb128 0xd
	.byte	0
	.uleb128 0x3
	.uleb128 0xe
	.uleb128 0x3a
	.uleb128 0xb
	.uleb128 0x3b
	.uleb128 0x5
	.uleb128 0x49
	.uleb128 0x13
	.uleb128 0x38
	.uleb128 0xb
	.byte	0
	.byte	0
	.uleb128 0xb
	.uleb128 0x16
	.byte	0
	.uleb128 0x3
	.uleb128 0xe
	.uleb128 0x3a
	.uleb128 0xb
	.uleb128 0x3b
	.uleb128 0xb
	.byte	0
	.byte	0
	.uleb128 0xc
	.uleb128 0x1
	.byte	0x1
	.uleb128 0x49
	.uleb128 0x13
	.uleb128 0x1
	.uleb128 0x13
	.byte	0
	.byte	0
	.uleb128 0xd
	.uleb128 0x21
	.byte	0
	.uleb128 0x49
	.uleb128 0x13
	.uleb128 0x2f
	.uleb128 0xb
	.byte	0
	.byte	0
	.uleb128 0xe
	.uleb128 0x13
	.byte	0
	.uleb128 0x3
	.uleb128 0xe
	.uleb128 0x3c
	.uleb128 0x19
	.byte	0
	.byte	0
	.uleb128 0xf
	.uleb128 0x34
	.byte	0
	.uleb128 0x3
	.uleb128 0xe
	.uleb128 0x3a
	.uleb128 0xb
	.uleb128 0x3b
	.uleb128 0x5
	.uleb128 0x49
	.uleb128 0x13
	.uleb128 0x3f
	.uleb128 0x19
	.uleb128 0x3c
	.uleb128 0x19
	.byte	0
	.byte	0
	.uleb128 0x10
	.uleb128 0x34
	.byte	0
	.uleb128 0x3
	.uleb128 0xe
	.uleb128 0x3a
	.uleb128 0xb
	.uleb128 0x3b
	.uleb128 0xb
	.uleb128 0x49
	.uleb128 0x13
	.uleb128 0x3f
	.uleb128 0x19
	.uleb128 0x3c
	.uleb128 0x19
	.byte	0
	.byte	0
	.uleb128 0x11
	.uleb128 0x21
	.byte	0
	.byte	0
	.byte	0
	.uleb128 0x12
	.uleb128 0x2e
	.byte	0x1
	.uleb128 0x3f
	.uleb128 0x19
	.uleb128 0x3
	.uleb128 0xe
	.uleb128 0x3a
	.uleb128 0xb
	.uleb128 0x3b
	.uleb128 0xb
	.uleb128 0x49
	.uleb128 0x13
	.uleb128 0x11
	.uleb128 0x1
	.uleb128 0x12
	.uleb128 0x6
	.uleb128 0x40
	.uleb128 0x18
	.uleb128 0x2116
	.uleb128 0x19
	.byte	0
	.byte	0
	.uleb128 0x13
	.uleb128 0x34
	.byte	0
	.uleb128 0x3
	.uleb128 0x8
	.uleb128 0x3a
	.uleb128 0xb
	.uleb128 0x3b
	.uleb128 0xb
	.uleb128 0x49
	.uleb128 0x13
	.uleb128 0x2
	.uleb128 0x18
	.byte	0
	.byte	0
	.byte	0
	.section	.debug_aranges,"",@progbits
	.long	0x1c
	.value	0x2
	.long	.Ldebug_info0
	.byte	0x4
	.byte	0
	.value	0
	.value	0
	.long	.Ltext0
	.long	.Letext0-.Ltext0
	.long	0
	.long	0
	.section	.debug_line,"",@progbits
.Ldebug_line0:
	.section	.debug_str,"MS",@progbits,1
.LASF23:
	.string	"_IO_buf_end"
.LASF9:
	.string	"__quad_t"
.LASF31:
	.string	"_old_offset"
.LASF55:
	.string	"sys_nerr"
.LASF26:
	.string	"_IO_save_end"
.LASF5:
	.string	"short int"
.LASF8:
	.string	"size_t"
.LASF13:
	.string	"sizetype"
.LASF36:
	.string	"_offset"
.LASF20:
	.string	"_IO_write_ptr"
.LASF15:
	.string	"_flags"
.LASF22:
	.string	"_IO_buf_base"
.LASF27:
	.string	"_markers"
.LASF17:
	.string	"_IO_read_end"
.LASF54:
	.string	"stderr"
.LASF6:
	.string	"long long int"
.LASF35:
	.string	"_lock"
.LASF11:
	.string	"long int"
.LASF32:
	.string	"_cur_column"
.LASF51:
	.string	"_IO_2_1_stderr_"
.LASF60:
	.string	"_IO_FILE_plus"
.LASF48:
	.string	"_pos"
.LASF47:
	.string	"_sbuf"
.LASF44:
	.string	"_IO_FILE"
.LASF1:
	.string	"unsigned char"
.LASF4:
	.string	"signed char"
.LASF7:
	.string	"long long unsigned int"
.LASF49:
	.string	"_IO_2_1_stdin_"
.LASF0:
	.string	"unsigned int"
.LASF45:
	.string	"_IO_marker"
.LASF34:
	.string	"_shortbuf"
.LASF19:
	.string	"_IO_write_base"
.LASF43:
	.string	"_unused2"
.LASF16:
	.string	"_IO_read_ptr"
.LASF2:
	.string	"short unsigned int"
.LASF14:
	.string	"char"
.LASF61:
	.string	"main"
.LASF46:
	.string	"_next"
.LASF37:
	.string	"__pad1"
.LASF38:
	.string	"__pad2"
.LASF39:
	.string	"__pad3"
.LASF40:
	.string	"__pad4"
.LASF41:
	.string	"__pad5"
.LASF3:
	.string	"long unsigned int"
.LASF21:
	.string	"_IO_write_end"
.LASF12:
	.string	"__off64_t"
.LASF50:
	.string	"_IO_2_1_stdout_"
.LASF10:
	.string	"__off_t"
.LASF28:
	.string	"_chain"
.LASF25:
	.string	"_IO_backup_base"
.LASF52:
	.string	"stdin"
.LASF30:
	.string	"_flags2"
.LASF42:
	.string	"_mode"
.LASF18:
	.string	"_IO_read_base"
.LASF33:
	.string	"_vtable_offset"
.LASF24:
	.string	"_IO_save_base"
.LASF56:
	.string	"sys_errlist"
.LASF29:
	.string	"_fileno"
.LASF58:
	.string	"/home/allgreed/Desktop/studia/ako/4"
.LASF53:
	.string	"stdout"
.LASF57:
	.string	"GNU C11 6.3.0 20170516 -m32 -mtune=generic -march=i686 -g"
.LASF59:
	.string	"_IO_lock_t"
	.ident	"GCC: (Debian 6.3.0-18+deb9u1) 6.3.0 20170516"
	.section	.note.GNU-stack,"",@progbits
