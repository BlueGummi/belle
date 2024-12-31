    ; used for benchmarking
.ssp [3]
.sbp [3]
.start [4]
    jmp @start
string1:
    .asciiz "Adding..."
    .word 10
string2:
    .asciiz "Subtracting..."
    .word 10
start:
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
    mov r0, @string1
    mov r1, @string2
    int 8
    jmp @add_loop
sub_print:
    pop r6
    mov r0, @string2
    mov r1, @start
    int 8
    jmp @sub_loop
