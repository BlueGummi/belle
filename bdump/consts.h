#ifndef CONSTS_H
#define CONSTS_H

#define TABLE_SIZE 4096
#define BUFFER_SIZE 65536 * 2
#define THREAD_COUNT 4
#define CHUNK_SIZE 1024
#define MAX_SUGGESTIONS 5
#define MAX_INPUT_FILES 100
#define MAX_LINES 100
#define MAX_LENGTH 256

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


#endif
