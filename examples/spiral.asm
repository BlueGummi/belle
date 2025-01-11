.ssp [3]
.sbp [3]
.start [4]
    mov r5, 31
    mov r4, 7
    mul r5, r4 ; 31 x 7 is in r5
    mov r4, 100
    add r4, 51 ; 151
    mul r5, r4
    mul r5, 2
    mov r4, 0
    jmp @add_print
add_loop:
    pop r6 ; pop off return address
    add r4, 1
    cmp r4, r5
    je @sub_print
    jmp @add_loop
sub_loop:
    pop r6 ; pop off that return address
    add r4, -1
    cmp r4, r3
    je @add_print
    jmp @sub_loop
add_print:
    pop r6
    add r7, 1
    cmp r7, 48
    jz @print
    jmp @add_loop
sub_print:
    pop r6
    add r2, 1
    cmp r2, 48
    jz @print
    jmp @sub_loop
print:
    add r3, 1
    cmp r3, 1 ; change this number to determine how many seconds the program takes to execute
    jz @end
    mov r2, 0
    mov r7, 0
    ret
end:
    hlt
