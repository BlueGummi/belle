.start [0x0]
    lea r1, @hello_end
    int 9
    mov r2, r0
    lea r0, @hello
    lea r0, @hello
loop:
    st &r0, r2
    add r0, 1
    cmp r0, r1
    bg @end
    jmp @loop
end:
    hlt
    .pad 244
hello:
    .asciiz "Hello, world!"
hello_end:
