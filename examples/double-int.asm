.start [0]
.ssp [128]
.sbp [128]
    jmp @main
msg1:
.asciiz "Enter a number. This will return double of it: "
msg2:
.asciiz "Double of your number is: "
main:
    mov r0, @msg1
    mov r1, @msg2
    int 8
    int 40
    mul r0, 2
    mov r2, r0
    mov r0, @msg2
    mov r1, @main
    int 8
    int 2
    hlt
