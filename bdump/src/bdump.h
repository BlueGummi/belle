#ifndef BDUMP_H
#define BDUMP_H
#define TABLE_SIZE 512
#include "consts.h"
typedef struct
{
    FILE *input;
    uint8_t buffer[BUFFER_SIZE];
    size_t bytes_read;
} ThreadData;

typedef struct
{
    int16_t opcode;
    int16_t destination;
    int16_t source;
    int16_t type; // type 0 is reg, reg
                  // type 1 is reg, lit
                  // type 2 is reg, mptr
                  // type 3 is reg, rptr
    int16_t full_ins;
} Instruction;
#define MAX_INPUT_FILES 100

typedef struct {
    char *input_files[MAX_INPUT_FILES];
    uint8_t num_files;
    uint8_t colors;
    uint8_t verbosity;
    uint8_t binary;
    uint8_t only_code;
    uint8_t print_hex;
    uint8_t hex_operands;
} CLI;
typedef enum {
    COLOR_RED,
    COLOR_GREEN,
    COLOR_YELLOW,
    COLOR_BLUE,
    COLOR_MAGENTA,
    COLOR_CYAN,
    COLOR_WHITE,
    COLOR_GRAY,
    COLOR_LIGHT_GRAY,
    COLOR_BG_BLACK,
    COLOR_BG_RED,
    COLOR_BG_GREEN,
    COLOR_BG_YELLOW,
    COLOR_BG_BLUE,
    COLOR_BG_MAGENTA,
    COLOR_BG_CYAN,
    COLOR_BG_WHITE,
    COLOR_COUNT
} Color;

typedef struct
{
    Color color;
    uint64_t id;
    uint64_t source;
    uint64_t destination;
    uint8_t column;
    bool reverse;
} Jump;

typedef struct Node {
    size_t key;
    Jump value;
    struct Node *next;
} Node;

typedef struct {
    Node *table[TABLE_SIZE];
} HashMap;

typedef struct {
    Jump *data;
    size_t size;
    size_t capacity;
} JumpVector;
CLI parse_arguments(int argc, char *argv[]);
Instruction parse_instruction(uint32_t instruction);
void print_binary(int16_t num);
void print_instruction(Instruction *s, JumpVector *jumpsHere);
void print_help(char *bin);
char *match_opcode(Instruction *s);
int main(int argc, char *argv[]);
void print_instruction_header(size_t line, bool colors, bool is_directive);
HashMap *jump_map_create(void);
void print_header(char *filename);
void print_footer(void);
#pragma once
#endif
#define FORMAT_STRING_MEMPTR (args.hex_operands ? "&0x%x" : "&%d")
#define FORMAT_STRING_MEMPTR_COLORED (args.hex_operands ? "&%s0x%x%s" : "&%s%d%s")
#define FORMAT_STRING_MEM_COLORED (args.hex_operands ? "[%s0x%x%s]" : "[%s%d%s]")
#define FORMAT_STRING_MEM (args.hex_operands ? "[0x%x]" : "[%d]")
#define FORMAT_STRING_COLORED (args.hex_operands ? "%s0x%x%s" : "%s%d%s")
#define FORMAT_STRING (args.hex_operands ? "0x%x" : "%d")
#define FORMAT_STRING_SIGNED (args.hex_operands ? "0x%x" : "-%d")

#define FORMAT_STRING_WORD_COLORED (args.hex_operands ? "%s.word%s %s0x%x%s" : "%s.word%s %s%d%s")
#define FORMAT_STRING_WORD (args.hex_operands ? ".word 0x%x" : ".word %d")

#define FORMAT_STRING_START (args.hex_operands ? ".start [0x%x]" : ".start [%d]")
#define FORMAT_STRING_START_COLORED (args.hex_operands ? "%s.start%s [%s0x%x%s]" : "%s.start%s [%s%d%s]")
#define FORMAT_STRING_SSP (args.hex_operands ? ".ssp [0x%x]" : ".ssp [%d]")
#define FORMAT_STRING_SBP (args.hex_operands ? ".sbp [0x%x]" : ".sbp [%d]")
#define FORMAT_STRING_SSP_COLORED (args.hex_operands ? "%s.ssp%s [%s0x%x%s]" : "%s.ssp%s [%s%d%s]")
#define FORMAT_STRING_SBP_COLORED (args.hex_operands ? "%s.sbp%s [%s0x%x%s]" : "%s.sbp%s [%s%d%s]")

#define FORMAT_STRING_ASCII_COLORED (args.hex_operands ? "%s%s%s (%s0x%x%s)" : "%s%s%s (%s%d%s)")
#define FORMAT_STRING_ASCII (args.hex_operands ? "%s (0x%x)" : "%s (%d)")

#define FORMAT_STRING_ST_COLORED (args.hex_operands ? "[%s0x%x%s], %sr%d%s" : "[%s%d%s], %sr%d%s")
#define FORMAT_STRING_ST (args.hex_operands ? "[0x%x], r%d" : "[%d], r%d")
#define ANSI_VARIED (args.hex_operands ? ANSI_CYAN : ANSI_GREEN)
#define FORMAT_STRING_COLORED_SIGNED (args.hex_operands ? "%s0x%x%s" : "-%s%d%s")
#define POSSIBLE_ANSI_BOLD (args.colors ? ANSI_BOLD : "")
#define FMTS (sign ? FORMAT_STRING_SIGNED : FORMAT_STRING)
#define FMTSC (sign ? FORMAT_STRING_COLORED_SIGNED : FORMAT_STRING_COLORED)
