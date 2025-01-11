    jmp @hello_end
hello_start:
    .asciiz "Hello, world!"
    .word 10
hello_end:
    lea r0, @hello_start
    lea r1, @hello_end
    int 8
    hlt
