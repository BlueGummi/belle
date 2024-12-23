    ; This program increments the number being printed to stdout, then detects an overflow, before decrementing it again
    ; It is indefinite
    ; used for benchmarking
    #include "defaults.asm"
    mov r5, #31
    mov r4, #7
    mul r5, r4 ; 31 x 7 is in r5
    mov r4, #100
    add r4, #51 ; 151
    mul r5, r4
    jmp @loop
loop:
    pop r4 ; pop off return address
    add r0, #1
    int #0
    cmp r0, r5
    jz @sub_loop
    jmp @loop
sub_loop:
    pop r4 ; pop off that return address
    add r0, #-1
    int #0
    cmp r0, r1
    jz @loop
    jmp @sub_loop
