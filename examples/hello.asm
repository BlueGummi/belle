; this program prints "Hello, world!" to stdout
.start [0]
mov r0, @hello_start
mov r1, @hello_end
int 8
hlt
hello_start:
    .asciiz "Hello, world!"
    .byte 10
hello_end:
