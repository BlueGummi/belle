.start [0x0]
    jmp start
start:
    lea r0, [0xff] 
    lea r1, [0xff]
    lea r2, [0xff]
    add r1, r2
    add r1, r2
    add r1, r2
    add r1, r2
    lea r2, [0xff]
    add r1, r2
    int 8
    mov r4, 0x7F
    add r4, r4
    add r4, r4
    add r4, r4
    add r4, r4
    add r4, r4
    add r4, r4
    int 10
    .pad 235
hello:
    .asciiz "         -/oyddmdhs+:.                                                     "
    .word 10
    .asciiz "     -odNMMMMMMMMNNmhy+-`                                                  "
    .word 10
    .asciiz "   -yNMMMMMMMMMMMNNNmmdhy+-                                                "
    .word 10
    .asciiz " omMMMMMMMMMMMNhhyyyohmdddhhhdo`                                           "
    .word 10
    .asciiz ".ydMMMMMMMMMMdhs++so/smdddhhhhdm+`                                         "
    .word 10
    .asciiz " oyhdmNMMMMMMMNdyooydmddddhhhhyhNd.                                        "
    .word 10
    .asciiz "  :oyhhdNNMMMMMMMNNNmmdddhhhhhyymMh          Powered by                    "
    .word 10
    .asciiz "    .:+sydNMMMMMNNNmmmdddhhhhhhmMmy                                        "
    .word 10
    .asciiz "       /mMMMMMMNNNmmmdddhhhhhmMNhs:         Gentoo Linux!!                 "
    .word 10
    .asciiz "    `oNMMMMMMMNNNmmmddddhhdmMNhs+`                                         "
    .word 10
    .asciiz "  `sNMMMMMMMMNNNmmmdddddmNMmhs/.                                           "
    .word 10
    .asciiz " /NMMMMMMMMNNNNmmmdddmNMNdso:`                                             "
    .word 10
    .asciiz "+MMMMMMMNNNNNmmmmdmNMNdso/-                                                "
    .word 10
    .asciiz "yMMNNNNNNNmmmmmNNMmhs+/-`                                                  "
    .word 10
    .asciiz "/hMMNNNNNNNNMNdhs++/-`                                                     "
    .word 10
    .asciiz "`/ohdmmddhys+++/:.`                                                        "
    .word 10
    .asciiz " `-//////:--.                                                              "
    .word 10
hello_end:
