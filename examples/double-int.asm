.start [0]
    jmp main
msg1:
.asciiz "Enter a number. This will return double of it: "
msg2:
.asciiz "Double of your number is: "
main:
    lea r0, msg1
    lea r1, msg2
    int 8
    int 40
    add r0, r0
    mov r2, r0
    lea r0, msg2
    lea r1, main
    int 8
    int 2
    hlt
