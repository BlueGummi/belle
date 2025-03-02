.data "This is my text\n"
.data "Metadata\n"
.start [0x0]
    ld r1, goober
    ld r4, goobertwo
    int 71
begin:
    add r0, 1
    cmp r0, r1
    bl begin
    bg end
goober:
    .word 0xfff4
end:
    add r2, 1
    mov r0, 0
    cmp r2, r4
    bg stop
    bl begin
stop:
    hlt
goobertwo:
    .word 5087
