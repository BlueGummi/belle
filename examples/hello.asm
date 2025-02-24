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
    .asciiz "         -/oyddmdhs+:.                                                     \n"
    .asciiz "     -odNMMMMMMMMNNmhy+-`                                                  \n"
    .asciiz "   -yNMMMMMMMMMMMNNNmmdhy+-                                                \n"
    .asciiz " omMMMMMMMMMMMNhhyyyohmdddhhhdo`                                           \n"
    .asciiz ".ydMMMMMMMMMMdhs++so/smdddhhhhdm+`                                         \n"
    .asciiz " oyhdmNMMMMMMMNdyooydmddddhhhhyhNd.                                        \n"
    .asciiz "  :oyhhdNNMMMMMMMNNNmmdddhhhhhyymMh          Powered by                    \n"
    .asciiz "    .:+sydNMMMMMNNNmmmdddhhhhhhmMmy                                        \n"
    .asciiz "       /mMMMMMMNNNmmmdddhhhhhmMNhs:         Gentoo Linux!!                 \n"
    .asciiz "    `oNMMMMMMMNNNmmmddddhhdmMNhs+`                                         \n"
    .asciiz "  `sNMMMMMMMMNNNmmmdddddmNMmhs/.                                           \n"
    .asciiz " /NMMMMMMMMNNNNmmmdddmNMNdso:`                                             \n"
    .asciiz "+MMMMMMMNNNNNmmmmdmNMNdso/-                                                \n"
    .asciiz "yMMNNNNNNNmmmmmNNMmhs+/-`                                                  \n"
    .asciiz "/hMMNNNNNNNNMNdhs++/-`                                                     \n"
    .asciiz "`/ohdmmddhys+++/:.`                                                        \n"
    .asciiz " `-//////:--.                                                              \n"
hello_end:
