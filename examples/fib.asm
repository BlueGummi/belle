    randomloc = 250
.ssp [0x28]  ; set stack pointer to addr 40
.sbp [0x28]  ; set base pointer to addr 40
.start $0x29 ; program starts at addr 41
    jmp @start
text:
   .asciiz "The number entered is too small."
   .word 10
msg:
    .asciiz "The golden ratio is: "

msg_end:
    .asciiz "Enter how many numbers to calculate: "

start:
    mov r6, 0 ; move 0 into register 6
    mov r4, 1 ; move 1 into register 4
    push r6
    push r4 ; push initial Fibonacci values onto call stack
    lea r0, @msg_end
    lea r1, @start
    int 8
    int 40
    cmp r0, 1
    jz @early_exit
    cmp r0, 0
    jz @early_exit
    cmp r0, 2
    jz @early_exit
    mov r1, 0
    jmp @fib_loop ; perform a jump into the loop

fib_loop:
    pop [randomloc] ; go random
    mov r5, 0 ; clear register 5
    pop r4
    pop r6 ; retrieve values
    add r5, r4 ; Fibonacci computation
    add r5, r6
    mov r6, r4
    mov r4, r5
    push r6
    push r4 ; push updated Fibonacci values back onto stack
    jo @finish ; jump if a value overflowed
    cmp r0, r2
    jz @finish
    int 5
    st &r1, r5
    add r1, 1
    add r2, 1
    jmp @fib_loop ; continue Fibonacci calculation

finish:
    pop r4
    add r1, -2
    mov r7, &r1 ; get back most recent value
    add r1, -1
    mov r6, &r1 ; get back second most recent value 
    div r7, r6 ; golden ratio
    lea r0, @msg
    lea r1, @msg_end
    int 8
    int 7
    hlt
early_exit:
    lea r0, @text
    lea r1, @msg
    int 8
    hlt
