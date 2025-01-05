    .ssp [40]  
    .sbp [40]  
    .start [41] 
    jmp @start
msg:
    .asciiz "The golden ratio is: "
msg_end:
    .asciiz "Enter how many numbers to calculate: "
start:
    mov r6, 0 
    mov r4, 1 
    push r6
    push r4 
    mov r0, @msg_end
    mov r1, @start
    int 8
    int 40
    mov r1, 0
    jmp @fib_loop 
fib_loop:
    pop $140
    mov r5, 0 
    pop r4
    pop r6 
    add r5, r4 
    add r5, r6
    mov r6, r4
    mov r4, r5
    push r6
    push r4 
    jo @finish 
    cmp r0, r2
    jz @finish
    int 5
    st &r1, r5
    add r1, 1
    add r2, 1
    jmp @fib_loop 
finish:
    add r1, -2
    mov r7, &r1 
    add r1, -1
    mov r6, &r1 
    div r7, r6 
    mov r0, @msg
    mov r1, @msg_end
    int 8
    int 7
    hlt
