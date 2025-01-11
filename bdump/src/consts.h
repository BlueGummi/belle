#ifndef CONSTS_H
#define CONSTS_H

#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define HLT_OP 0b0000
#define ADD_OP 0b0001
#define JO_OP 0b0010
#define POP_OP 0b0011
#define DIV_OP 0b0100
#define RET_OP 0b0101
#define LD_OP 0b0110
#define ST_OP 0b0111
#define JMP_OP 0b1000
#define JZ_OP 0b1001
#define CMP_OP 0b1010
#define MUL_OP 0b1011
#define PUSH_OP 0b1100
#define INT_OP 0b1101
#define MOV_OP 0b1110
#define LEA_OP 0b1111

#define ANSI_RESET "\033[0m"
#define ANSI_BOLD "\033[1m"
#define ANSI_UNDERLINE "\033[4m"
#define ANSI_BLACK "\033[30m"
#define ANSI_RED "\033[31m"
#define ANSI_GREEN "\033[32m"
#define ANSI_YELLOW "\033[33m"
#define ANSI_BLUE "\033[34m"
#define ANSI_MAGENTA "\033[35m"
#define ANSI_CYAN "\033[36m"
#define ANSI_WHITE "\033[37m"
#define ANSI_GRAY "\033[90m"
#define ANSI_LIGHT_GRAY "\033[37m"
#define ANSI_BG_BLACK "\033[40m"
#define ANSI_BG_RED "\033[41m"
#define ANSI_BG_GREEN "\033[42m"
#define ANSI_BG_YELLOW "\033[43m"
#define ANSI_BG_BLUE "\033[44m"
#define ANSI_BG_MAGENTA "\033[45m"
#define ANSI_BG_CYAN "\033[46m"
#define ANSI_BG_WHITE "\033[47m"
#endif
#pragma once
