    ; This program prints "Hello, world!" to stdout
    #include "defaults.asm"
    mov r2, 'h'
    st $1, r2
    mov r2, 'e'
    st $2, r2
    mov r2, 'l'
    st $3, r2
    mov r2, 'l'
    st $4, r2
    mov r2, 'o'
    st $5, r2
    mov r2, ','
    st $6, r2
    mov r2, ' '
    st $7, r2
    mov r2, 'w'
    st $8, r2
    mov r2, 'o'
    st $9, r2
    mov r2, 'r'
    st $10, r2
    mov r2, 'l'
    st $11, r2
    mov r2, 'd'
    st $12, r2
    mov r2, '!'
    st $13, r2
    mov r2, #10
    st $14, r2
    mov r0, #1
    mov r1, #14
    int #8
    hlt
