#ifndef BDUMP_H
#define BDUMP_H
#include "consts.h"
typedef struct
{
    FILE   *input;
    uint8_t buffer[BUFFER_SIZE];
    size_t  bytes_read;
} ThreadData;

typedef struct
{
    int opcode;
    int destination;
    int source;
    int type; // type 0 is reg, reg
              // type 1 is reg, lit
              // type 2 is reg, mptr
              // type 3 is reg, rptr
    int full_ins;
} Instruction;

typedef struct
{
    char *input_file;
    int   line_num;
    int   colors;
    int   verbosity;
    int   binary;
} CLI;

CLI         parse_arguments(int argc, char *argv[]);
Instruction parse_instruction(int instruction);
void        print_binary(int num, int leading);
void        print_instruction(Instruction *s);
void        print_help(char *bin);
char       *match_opcode(Instruction *s);
int         main(int argc, char *argv[]);
void print_instruction_header(int line, bool colors);
#pragma once
#endif
