.start [0x0]
    lea r0, @hello
    lea r1, @hello_end
    int 8
    mov r4, 0x7F
    add r4, r4
    add r4, r4
    add r4, r4
    add r4, r4
    add r4, r4
    add r4, r4
    int 10
    .pad 244
hello:
    .asciiz "Hello, world!"
hello_end:
