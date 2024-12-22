#include "defaults.asm"
    mov r2, 'w'
    st [51], r2
    mov r2, 'a'
    st [52], r2
    mov r2, 's' ; s
    st [53], r2
    mov r2, 't' ; t
    st [54], r2
    mov r2, 'e' ; e
    st [55], r2
    mov r2, 'd' ; d
    st [56], r2
    mov r2, ' '; 
    st [57], r2
    mov r2, 't' ; t
    st [58], r2
    mov r2, 'i' ; i
    st [59], r2
    mov r2, 'm' ; m
    st [60], r2
    mov r2, 'e' ; e
    st [61], r2
    mov r2, '.' ; .
    st [62], r2
    mov r2, 10
    st [63], r2
    mov r0, 51
    mov r1, 63    
    jmp @time_waster
time_waster:
    pop r4
    nop
    jmp @print
    jmp @time_waster
print:
    int 8
    ret
