newline = 10
    .data "Copyright (c) 2025 BlueGummi"
    .dataword 10
    .data "All rights reserved"
    .dataword 10
    .dataword 10
    .data "This code is licensed under the BSD 3-Clause License."
.ssp [0x28]  ; set stack pointer to addr 0x28
.sbp [0x28]  ; set base pointer to addr 0x28
.start [0x29] ; program starts at addr 0x29
    int 71   ; don't push returns
    jmp start
msg2:
    .asciiz "The number entered is too large."
    .word newline
text:
    .asciiz "The number entered is too small."
    .word newline
msg:
    .asciiz "The golden ratio is: "

msg_end:
    .asciiz "Enter how many numbers to calculate (max 23): "

start:
    mov r6, 0 ; move 0 into register 6
    mov r4, 1 ; move 1 into register 4
    push r6
    push r4 ; push initial Fibonacci values onto call stack
    lea r0, msg_end
    lea r1, start
    int 8
    int 40
    cmp r0, 1
    bz early_exit
    cmp r0, 0
    bz early_exit
    cmp r0, 2
    bz early_exit
    cmp r0, 24
    bg early_exit2
    mov r1, 0
fib_loop:
    mov r5, 0 ; clear register 5
    pop r4
    pop r6 ; retrieve values
    add r5, r4 ; Fibonacci computation
    add r5, r6
    mov r6, r4
    mov r4, r5
    push r6
    push r4 ; push updated Fibonacci values back onto stack
    cmp r0, r2
    bz finish
    int 5
    st &r1, r5
    add r1, 1
    add r2, 1
    jmp fib_loop ; continue Fibonacci calculation

finish:
    add r1, -2
    mov r7, &r1 ; get back most recent value
    add r1, -1
    mov r6, &r1 ; get back second most recent value 
    div r7, r6 ; golden ratio
    lea r0, msg
    lea r1, msg_end
    int 8
    int 7
    hlt
early_exit:
    lea r0, text
    lea r1, msg
    int 8
    hlt
early_exit2:
    lea r0, msg2
    lea r1, text
    int 8
    hlt
