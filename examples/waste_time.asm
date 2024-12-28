    jmp @start
msg_begin:
    .asciiz "Wasted time."
    .word 10
msg_end:
start:
    mov r0, @msg_begin
    mov r1, @msg_end
    jmp @time_waster
time_waster:
    pop r4
    nop
    jmp @print
    jmp @time_waster
print:
    int 8
    ret

