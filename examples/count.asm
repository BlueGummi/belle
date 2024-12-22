    ; This program counts to the 16 bit unsigned integer limit 
    ; It is indefinite
#include "defaults.asm"
    jmp @add_loop
add_loop:
    pop r4
    add r5, 1
    int 5
    jo @end
    jmp @add_loop
end:
    pop r4
    hlt
