    ; This program calculates the fibonacci sequence to the largest signed 16 bit integer value
    p = 20
    st = 100
.ssp [p]
.sbp [p]
.start [st]
    mov r6, 0         ; first value of fibonacci
    mov r7, 1         ; second value of fibonacci
    push r6           ; r6 onto stack
    push r7           ; r7 onto stack 
    jmp @fib_loop
fib_loop: ; lib foop
    pop r0            ; pop the jump address and save it
    mov r5, 0         ; final value of fibonacci
    pop r7            ; load values back into registers
    pop r6
    add r5, r7        ; z += y
    add r5, r6        ; z += x
    mov r6, r7        ; x = y
    mov r7, r5        ; y = z
    push r6           ; push values back onto stack
    push r7
    jo @finish
    int 5             ; print the value in register 5
    jmp @fib_loop     ; continue loop
finish: 	      ; done
    hlt
