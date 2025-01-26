    p = 20
    st = 100
.ssp [p]
.sbp [p]
.start [st]
    jmp @add_loop
add_loop:
    pop r4
    add r5, 1
    bo @end
    jmp @add_loop
end:
    pop r4
    hlt
