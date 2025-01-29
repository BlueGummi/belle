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

#define BUFFER_SIZE 65536 * 2
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

#define MAX_SUGGESTIONS 5
#endif
