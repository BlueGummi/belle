.start [0x0]
    mov r4, 0x7F
    add r4, r4
    add r4, r4
    add r4, r4
    add r4, r4
    add r4, r4
    add r4, r4
    int 10
    .pad 247
    .asciiz "Hello, world!"
    .word 10
    .asciiz "Testt"
