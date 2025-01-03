; this program prints "Hello, world!" to stdout
	jmp @hello_end
hello_start:
    .asciiz "Hello, world!"
    .word 10
hello_end:
	mov r0, @hello_start
	mov r1, @hello_end
	int 8
	hlt
