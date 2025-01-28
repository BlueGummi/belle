#ifndef CONSTS_H
#define CONSTS_H

#include <errno.h>
#include <inttypes.h>
#include <math.h>
#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#ifdef _WIN32
#include <windows.h>
#else
#include <sys/stat.h>
#include <unistd.h>
#endif

#define BUFFER_SIZE 65536*2
#define THREAD_COUNT 4

#define CHUNK_SIZE 1024

#define HLT_OP 0x0
#define ADD_OP 0x1
#define BO_OP 0x2
#define POP_OP 0x3
#define DIV_OP 0x4
#define RET_OP 0x5
#define LD_OP 0x6
#define ST_OP 0x7
#define JMP_OP 0x8
#define BZ_OP 0x9
#define CMP_OP 0xa
#define NAND_OP 0xb
#define PUSH_OP 0xc
#define INT_OP 0xd
#define MOV_OP 0xe
#define LEA_OP 0xf

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
#define ANSI_BRIGHT_BLACK "\033[90m"
#define ANSI_BRIGHT_RED "\033[91m"
#define ANSI_BRIGHT_GREEN "\033[92m"
#define ANSI_BRIGHT_YELLOW "\033[93m"
#define ANSI_BRIGHT_BLUE "\033[94m"
#define ANSI_BRIGHT_MAGENTA "\033[95m"
#define ANSI_BRIGHT_CYAN "\033[96m"
#define ANSI_BRIGHT_WHITE "\033[97m"
#define MAX_SUGGESTIONS 5
#endif
