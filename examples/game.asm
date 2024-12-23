    #include "defaults.asm"
    mov r1, '-' ; hyphen ascii
    push 2
begin:
    pop r5 ; avoiding overflow
    st &r3, r1 ; put it into memory
    add r3, 1 ; accumulator
    cmp r3, 30
    jz @next
    jmp @begin
next:	
    mov r3, 10
    st [30], r3
    mov r0, 0
    mov r1, 50
    int 8
    int 9
    int 12
    int 0
    cmp r0, 'D'
    jz @one_more
    cmp r0, 'd'
    jz @one_more
    int 12
    cmp r0, 'A'
    jz @one_less
    cmp r0, 'a'
    jz @one_less
    jmp @done
one_more:
    pop r4
    mov r1, '-'
    st [31], r1
    jmp @next
one_less:
    pop r4
    mov r1, 8
    st [30], r1
    jmp @next
done:
    hlt
