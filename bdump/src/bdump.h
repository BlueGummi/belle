#ifndef BDUMP_H
#define BDUMP_H

#include <errno.h>
#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#ifdef _WIN32
#include <windows.h>
typedef DWORD thread_ret_t;
#else
#include <pthread.h>
typedef void *thread_ret_t;
#endif

#define BUFFER_SIZE 1024
#define THREAD_COUNT 4

#define CHUNK_SIZE 1024

typedef struct {
    FILE *input;
    uint8_t buffer[BUFFER_SIZE];
    size_t bytes_read;
} ThreadData;

typedef struct {
    int opcode;
    int destination;
    int source;
    int type; // type 0 is reg, reg
              // type 1 is reg, lit
              // type 2 is reg, mptr
              // type 3 is reg, rptr
    int full_ins;
} Instruction;

typedef struct {
    char *input_file;
    int line_num;
    int colors;
    int verbosity;
    int binary;
} CLI;
CLI args = {0};

CLI parse_arguments(int argc, char *argv[]);
Instruction parse_instruction(int instruction);
void print_binary(int num, int leading);
void print_instruction(Instruction *s);
void print_help(char *bin);
char *match_opcode(Instruction *s);
int main(int argc, char *argv[]);
#pragma once
#endif
#include "consts.h"
#include "print_helpers.c"
#include "print_utils.c"
