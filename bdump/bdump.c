/*
 * Copyright (c) 2024 BlueGummi
 * All rights reserved.
 *
 * This code is licensed under the BSD 3-Clause License.
 */

#define TABLE_SIZE 4096
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

#ifdef _WIN32
#define PRINTF(msg, ...)                                                \
    {                                                                   \
        HANDLE hConsole = GetStdHandle(STD_OUTPUT_HANDLE);              \
        snprintf(buffer, sizeof(buffer), msg, ##__VA_ARGS__);           \
        DWORD written;                                                  \
        WriteConsole(hConsole, buffer, strlen(buffer), &written, NULL); \
    }
#else
#define PRINTF(msg, ...) printf(msg, ##__VA_ARGS__)
#endif

typedef struct
{
    FILE *input;
    uint8_t buffer[BUFFER_SIZE];
    size_t bytes_read;
} ThreadData;

typedef struct
{
    uint32_t opcode;
    int32_t destination;
    int32_t source;
    int32_t type; // type 0 is reg, reg
                  // type 1 is reg, lit
                  // type 2 is reg, mptr
                  // type 3 is reg, rptr
    int16_t full_ins;
} Instruction;
#define MAX_INPUT_FILES 100

typedef struct {
    char *input_files[MAX_INPUT_FILES];
    uint8_t num_files;
    bool colors;
    bool binary;
    bool only_code;
    bool hex_operands;
    bool no_jump;
    bool concat_chars;
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
} Color;

typedef struct
{
    Color color;
    uint64_t source;
    uint64_t destination;
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
void print_instruction(Instruction *s, Instruction *d, JumpVector *jumpsHere);
void print_help(char *bin);
void *process_instructions(void *arg, char *filename);
void print_jump_instruction(Instruction *ins);
void print_hlt_instruction(Instruction *ins);
bool is_directive(Instruction *ins);
void print_two_reg_args(Instruction *ins);
void jump_map_insert(HashMap *map, size_t key, Jump value);
Jump *jump_map_get(HashMap *map, size_t key);
void free_jump_vector(JumpVector *vector);
void print_operation(Instruction *ins, char *op);
void init_jump_vector(JumpVector *vector);
JumpVector *find_jumps_at_address(HashMap *jump_map, uint64_t address);
void add_jump(JumpVector *vector, Jump jump);
void free_map(HashMap *map);
unsigned int hash(size_t key);
char *match_opcode(Instruction *s);
int main(int argc, char *argv[]);
void print_instruction_header(size_t line, bool is_directive);
HashMap *jump_map_create(void);
void print_header(const char *metadata, char *filename);
void print_footer(void);
void suggest_option(const char *invalid_option, int valid_count);
int levenshtein_distance(const char *s1, const char *s2);
Color get_color(int index);
char *color_to_ansi(Color color);
void get_file_size(const char *filename, char *size_str, size_t size_str_len);

#define STRINGIFY(x) #x
#define PRINT_LINE_AND_FILE printf(" on line %d in file %s\n", __LINE__, __FILE__)
#define FORMAT_STRING_MEMPTR "&0x%X"
#define FORMAT_STRING_MEMPTR_COLORED "&%s0x%X%s"
#define FORMAT_STRING_MEM_COLORED "[%s0x%X%s]"
#define FORMAT_STRING_MEM "[0x%X]"
#define FORMAT_STRING_COLORED (args.hex_operands ? "%s0x%X%s" : "%s%d%s")
#define FORMAT_STRING (args.hex_operands ? "0x%X" : "%d")
#define FORMAT_STRING_SIGNED (args.hex_operands ? "0x%X" : "-%d")

#define FORMAT_STRING_WORD_COLORED (args.hex_operands ? "%s.word%s %s0x%X%s" : "%s.word%s %s%d%s")
#define FORMAT_STRING_WORD (args.hex_operands ? ".word 0x%X" : ".word %d")

#define FORMAT_STRING_START ".start [0x%X]"
#define FORMAT_STRING_START_COLORED "%s.start%s [%s0x%X%s]"
#define FORMAT_STRING_SSP ".ssp [0x%X]"
#define FORMAT_STRING_SBP ".sbp [0x%X]"
#define FORMAT_STRING_SSP_COLORED "%s.ssp%s [%s0x%X%s]"
#define FORMAT_STRING_SBP_COLORED "%s.sbp%s [%s0x%X%s]"

#define FORMAT_STRING_ASCII_COLORED (args.hex_operands ? "%s%s%s (%s0x%X%s)" : "%s%s%s (%s%d%s)")
#define FORMAT_STRING_ASCII (args.hex_operands ? "%s (0x%X)" : "%s (%d)")

#define FORMAT_STRING_ST_COLORED "[%s0x%X%s], %sr%d%s"
#define FORMAT_STRING_ST "[0x%X], r%d"
#define ANSI_VARIED (args.hex_operands ? ANSI_CYAN : ANSI_GREEN)
#define FORMAT_STRING_COLORED_SIGNED (args.hex_operands ? "%s0x%X%s" : "-%s%d%s")
#define POSSIBLE_ANSI_BOLD (args.colors ? ANSI_BOLD : "")
#define FMTS (sign ? FORMAT_STRING_SIGNED : FORMAT_STRING)
#define FMTSC (sign ? FORMAT_STRING_COLORED_SIGNED : FORMAT_STRING_COLORED)
CLI args = {0};

#ifdef _WIN32
int is_terminal(void) {
    HANDLE hConsole = GetStdHandle(STD_OUTPUT_HANDLE);
    DWORD mode;
    return GetConsoleMode(hConsole, &mode);
}
#else
int is_terminal(void) {
    return isatty(fileno(stdout));
}
#endif
#define ANSI_RESET (is_term ? "\033[0m" : "")
#define ANSI_BOLD (args.colors && is_term ? "\033[1m" : "")
#define ANSI_UNDERLINE (args.colors && is_term ? "\033[4m" : "")
#define ANSI_BLACK (args.colors && is_term ? "\033[30m" : "")
#define ANSI_RED (args.colors && is_term ? "\033[31m" : "")
#define ANSI_GREEN (args.colors && is_term ? "\033[32m" : "")
#define ANSI_YELLOW (args.colors && is_term ? "\033[33m" : "")
#define ANSI_BLUE (args.colors && is_term ? "\033[34m" : "")
#define ANSI_MAGENTA (args.colors && is_term ? "\033[35m" : "")
#define ANSI_CYAN (args.colors && is_term ? "\033[36m" : "")
#define ANSI_WHITE (args.colors && is_term ? "\033[37m" : "")
#define ANSI_GRAY (args.colors && is_term ? "\033[90m" : "")
#define ANSI_LIGHT_GRAY (args.colors && is_term ? "\033[37m" : "")
#define ANSI_BG_BLACK (args.colors && is_term ? "\033[40m" : "")
#define ANSI_BG_RED (args.colors && is_term ? "\033[41m" : "")
#define ANSI_BG_GREEN (args.colors && is_term ? "\033[42m" : "")
#define ANSI_BG_YELLOW (args.colors && is_term ? "\033[43m" : "")
#define ANSI_BG_BLUE (args.colors && is_term ? "\033[44m" : "")
#define ANSI_BG_MAGENTA (args.colors && is_term ? "\033[45m" : "")
#define ANSI_BG_CYAN (args.colors && is_term ? "\033[46m" : "")
#define ANSI_BG_WHITE (args.colors && is_term ? "\033[47m" : "")
#define ANSI_BRIGHT_BLACK (args.colors && is_term ? "\033[90m" : "")
#define ANSI_BRIGHT_RED (args.colors && is_term ? "\033[91m" : "")
#define ANSI_BRIGHT_GREEN (args.colors && is_term ? "\033[92m" : "")
#define ANSI_BRIGHT_YELLOW (args.colors && is_term ? "\033[93m" : "")
#define ANSI_BRIGHT_BLUE (args.colors && is_term ? "\033[94m" : "")
#define ANSI_BRIGHT_MAGENTA (args.colors && is_term ? "\033[95m" : "")
#define ANSI_BRIGHT_CYAN (args.colors && is_term ? "\033[96m" : "")
#define ANSI_BRIGHT_WHITE (args.colors && is_term ? "\033[97m" : "")
#define ANSI_RED_CONST "\033[31m"
#define ANSI_BOLD_CONST "\033[1m"
bool is_term = true;
int bin_version = 0;
bool in_char = false;
bool next_in_char = false;
bool likely_label = false;
bool printed_addr = false;
char global_str[512] = "";

#ifdef _WIN32
char buffer[1024];
#endif

Color get_color(int index) {
    Color color_codes[] = {
        COLOR_RED, COLOR_GREEN, COLOR_YELLOW,
        COLOR_BLUE, COLOR_MAGENTA, COLOR_CYAN, COLOR_WHITE,
        COLOR_GRAY};

    int num_codes = sizeof(color_codes) / sizeof(color_codes[0]);

    index = index % num_codes;

    return color_codes[index];
}

char *color_to_ansi(Color color) {
    switch (color) {
    case COLOR_RED:
        return ANSI_RED;
    case COLOR_GREEN:
        return ANSI_GREEN;
    case COLOR_YELLOW:
        return ANSI_YELLOW;
    case COLOR_BLUE:
        return ANSI_BLUE;
    case COLOR_MAGENTA:
        return ANSI_MAGENTA;
    case COLOR_CYAN:
        return ANSI_CYAN;
    case COLOR_WHITE:
        return ANSI_WHITE;
    case COLOR_GRAY:
        return ANSI_GRAY;
    case COLOR_LIGHT_GRAY:
        return ANSI_LIGHT_GRAY;
    case COLOR_BG_BLACK:
        return ANSI_BG_BLACK;
    case COLOR_BG_RED:
        return ANSI_BG_RED;
    case COLOR_BG_GREEN:
        return ANSI_BG_GREEN;
    case COLOR_BG_YELLOW:
        return ANSI_BG_YELLOW;
    case COLOR_BG_BLUE:
        return ANSI_BG_BLUE;
    case COLOR_BG_MAGENTA:
        return ANSI_BG_MAGENTA;
    case COLOR_BG_CYAN:
        return ANSI_BG_CYAN;
    case COLOR_BG_WHITE:
        return ANSI_BG_WHITE;
    default:
        return ANSI_CYAN;
    }
}
void get_file_size(const char *filename, char *size_str, size_t size_str_len) {
    long file_size = 0;

#ifdef _WIN32
    HANDLE hFile = CreateFileA(filename, GENERIC_READ, FILE_SHARE_READ, NULL, OPEN_EXISTING, FILE_ATTRIBUTE_NORMAL, NULL);
    if (hFile == INVALID_HANDLE_VALUE) {
        snprintf(size_str, size_str_len, "Error getting file size");
        return;
    }
    file_size = GetFileSize(hFile, NULL);
    CloseHandle(hFile);
#else
    struct stat file_stat;
    if (stat(filename, &file_stat) != 0) {
        snprintf(size_str, size_str_len, "Error getting file size");
        return;
    }
    file_size = file_stat.st_size;
#endif

    if (file_size < 1024) {
        snprintf(size_str, size_str_len, "%ld B", file_size);
    } else if (file_size < 1048576) {
        snprintf(size_str, size_str_len, "%.2f KB", file_size / 1024.0);
    } else {
        snprintf(size_str, size_str_len, "%.2f MB", file_size / 1048576.0);
    }
}

void get_last_modified_date(const char *filename, char *date_str, size_t date_str_len) {
#ifdef _WIN32
    HANDLE hFile = CreateFileA(filename, GENERIC_READ, FILE_SHARE_READ, NULL, OPEN_EXISTING, FILE_ATTRIBUTE_NORMAL, NULL);
    if (hFile == INVALID_HANDLE_VALUE) {
        snprintf(date_str, date_str_len, "Error getting date");
        return;
    }

    FILETIME ftLastWrite;
    GetFileTime(hFile, NULL, NULL, &ftLastWrite);
    CloseHandle(hFile);

    SYSTEMTIME st;
    FileTimeToSystemTime(&ftLastWrite, &st);
    snprintf(date_str, date_str_len, "%02d/%02d/%04d %02d:%02d:%02d",
             st.wMonth, st.wDay, st.wYear, st.wHour, st.wMinute, st.wSecond);
#else
    struct stat file_stat;
    if (stat(filename, &file_stat) != 0) {
        snprintf(date_str, date_str_len, "Error getting date");
        return;
    }

    struct tm *tm_info = localtime(&file_stat.st_mtime);
    strftime(date_str, date_str_len, "%Y-%m-%d %H:%M:%S", tm_info);
#endif
}
unsigned int hash(size_t key) {
    return key % TABLE_SIZE;
}

size_t max_columns = 0;
HashMap *jump_map_create(void) {
    HashMap *map = malloc(sizeof(HashMap));
    if (map == NULL) {
        perror("Hashmap creation memory allocation failed");
        PRINT_LINE_AND_FILE;
        exit(EXIT_FAILURE);
    }
    for (int i = 0; i < TABLE_SIZE; i++) {
        map->table[i] = NULL;
    }
    return map;
}

void jump_map_insert(HashMap *map, size_t key, Jump value) {
    unsigned int index = hash(key);
    Node *newNode = malloc(sizeof(Node));
    if (newNode == NULL) {
        perror("Node creation during hashmap insertion memory allocation failed");
        PRINT_LINE_AND_FILE;
        exit(EXIT_FAILURE);
    }
    newNode->key = key;
    newNode->value = value;
    newNode->next = map->table[index];
    map->table[index] = newNode;
}
void free_node(Node *node) {
    if (node == NULL) {
        return;
    }

    free(node);
}
Jump *jump_map_get(HashMap *map, size_t key) {
    unsigned int index = hash(key);
    Node *current = map->table[index];
    while (current) {
        if (current->key == key) {
            return &current->value;
        }
        current = current->next;
    }
    return NULL;
}

void free_map(HashMap *map) {
    for (int i = 0; i < TABLE_SIZE; i++) {
        Node *current = map->table[i];
        while (current) {
            Node *temp = current;
            current = current->next;
            free(temp);
        }
    }
    free(map);
}

void init_jump_vector(JumpVector *vector) {
    vector->size = 0;
    vector->capacity = 4;
    vector->data = malloc(vector->capacity * sizeof(Jump));
    if (vector->data == NULL) {
        perror("Vector initialization data memory allocation failed");
        PRINT_LINE_AND_FILE;
        exit(EXIT_FAILURE);
    }
}

void add_jump(JumpVector *vector, Jump jump) {
    if (vector->size >= vector->capacity) {
        vector->capacity *= 2;
        vector->data = realloc(vector->data, vector->capacity * sizeof(Jump));
        if (vector->data == NULL) {
            perror("Vector insertion memory allocation failed");
            PRINT_LINE_AND_FILE;
            exit(EXIT_FAILURE);
        }
    }
    vector->data[vector->size++] = jump;
}
void free_jump_vector(JumpVector *vector) {
    if (vector == NULL) {
        return;
    }

    free(vector->data);
    free(vector);
}
JumpVector *find_jumps_at_address(HashMap *jump_map, uint64_t address) {
    JumpVector *jump_vector = malloc(sizeof(JumpVector));
    if (jump_vector == NULL) {
        perror("Vector address search memory allocation failed");
        PRINT_LINE_AND_FILE;
        exit(EXIT_FAILURE);
    }
    init_jump_vector(jump_vector);

    for (size_t i = 0; i < TABLE_SIZE; i++) {
        Node *current = jump_map->table[i];
        while (current) {
            Jump *jump = &current->value;
            if ((address >= jump->source && address <= jump->destination) || (address <= jump->source && address >= jump->destination && jump->reverse == 1)) {
                add_jump(jump_vector, *jump);
            }
            current = current->next;
        }
        free_node(current);
    }
    return jump_vector;
}

HashMap *jump_map_global;
size_t current_addr = 100;
uint64_t len = 0;
void print_operation(Instruction *ins, char *op) {
    bool is_jump = strcmp(op, "bz") == 0 || strcmp(op, "bo") == 0 || strcmp(op, "jmp") == 0;
    bool invert = ins->destination >> 2 == 1;
    if (is_jump && invert) {
        if (strcmp(op, "bz") == 0)
            op = "bnz";
        else if (strcmp(op, "bo") == 0)
            op = "bno";
    }

    PRINTF("%s%s%s ", ANSI_BLUE, op, ANSI_RESET);
    char tempstr[40];
    snprintf(tempstr, sizeof(tempstr), "%s ", op);
    len += strlen(tempstr);
}

void print_two_reg_args(Instruction *ins) {
    PRINTF("%sr%d%s, ", ANSI_YELLOW, ins->destination, ANSI_RESET);
    char str[20];
    snprintf(str, sizeof(str), "r%d, ", ins->destination);
    len += strlen(str);
    switch (ins->type) {
    case 0: // register
        PRINTF("%sr%d%s", ANSI_YELLOW, ins->source, ANSI_RESET);
        snprintf(str, sizeof(str), "r%d", ins->source);
        break;
    case 1: // literal
    {
        bool sign = (ins->source >> 7) == 1;

        int8_t val = (int8_t) ins->source & 0x7f;

        PRINTF(FMTSC, ANSI_VARIED, args.hex_operands ? (sign ? ins->source : val) : val, ANSI_RESET);
        snprintf(str, sizeof(str), FMTS, args.hex_operands ? (sign ? ins->source : val) : val);
    } break;

    case 2: // memory address indirect
    {
        int memaddr = ins->full_ins & 0x7f;

        PRINTF(FORMAT_STRING_MEMPTR_COLORED, ANSI_VARIED, memaddr, ANSI_RESET);
        snprintf(str, sizeof(str), FORMAT_STRING_MEMPTR, memaddr);
    } break;

    case 3: // register indirect
    {
        PRINTF("%s&r%d%s", ANSI_YELLOW, ins->source & 0xF, ANSI_RESET);
        snprintf(str, sizeof(str), "&r%d", ins->source & 0xF);
    } break;

    default:
        perror("Unknown instruction type\n");
        exit(1);
    }
    len += strlen(str);
}

void print_jump_instruction(Instruction *ins) {
    char str[20];
    if (((ins->destination >> 1) & 1) == 1) {
        PRINTF("%s&r%d%s", ANSI_YELLOW, ins->source & 0xF, ANSI_RESET);
        snprintf(str, sizeof(str), "&r%d", ins->source & 0xF);
        len += strlen(str);
        return;
    }

    PRINTF(FORMAT_STRING_MEM_COLORED, ANSI_VARIED, ins->full_ins & 1023, ANSI_RESET);
    snprintf(str, sizeof(str), FORMAT_STRING_MEM, ins->full_ins & 1023);
    len += strlen(str);
}

void print_hlt_instruction(Instruction *ins) {
    char str[40];
    switch (ins->destination) {
    case 1:
        PRINTF(FORMAT_STRING_START_COLORED, ANSI_BLUE, ANSI_RESET, ANSI_VARIED, ins->full_ins & 0x1ff, ANSI_RESET);
        snprintf(str, sizeof(str), FORMAT_STRING_START, ins->full_ins & 0x1ff);
        len += strlen(str);
        break;
    case 2:
        PRINTF(FORMAT_STRING_SSP_COLORED, ANSI_BLUE, ANSI_RESET, ANSI_VARIED, ins->full_ins & 0x1ff, ANSI_RESET);
        snprintf(str, sizeof(str), FORMAT_STRING_SSP, ins->full_ins & 0x1ff);
        len += strlen(str);
        break;
    case 3:
        PRINTF(FORMAT_STRING_SBP_COLORED, ANSI_BLUE, ANSI_RESET, ANSI_VARIED, ins->full_ins & 0x1ff, ANSI_RESET);
        snprintf(str, sizeof(str), FORMAT_STRING_SBP, ins->full_ins & 0x1ff);
        len += strlen(str);
        break;
    default:
        if (ins->full_ins == 0) {
            PRINTF("%shlt%s", ANSI_BLUE, ANSI_RESET);
            len += 3;
            return;
        } else if ((ins->full_ins >> 8) == 0 && args.concat_chars) {
            return;
        }
        if (!args.only_code) {
            PRINTF(FORMAT_STRING_ASCII_COLORED, ANSI_BLUE,
                   (ins->full_ins == '\n'                          ? "\\n"
                    : ins->full_ins == '\t'                        ? "\\t"
                    : ins->full_ins == '\\'                        ? "\\\\"
                    : (ins->full_ins >= 32 && ins->full_ins < 127) ? (char[]) {(char) ins->full_ins, '\0'}
                                                                   : "???"),
                   ANSI_RESET, ANSI_VARIED, ins->full_ins, ANSI_RESET);
            snprintf(str, sizeof(str), FORMAT_STRING_ASCII,
                     (ins->full_ins == '\n'                          ? "\\n"
                      : ins->full_ins == '\t'                        ? "\\t"
                      : ins->full_ins == '\\'                        ? "\\\\"
                      : (ins->full_ins >= 32 && ins->full_ins < 127) ? (char[]) {(char) ins->full_ins, '\0'}
                                                                     : "???"),
                     ins->full_ins);
            len += strlen(str);
        } else {
            PRINTF(FORMAT_STRING_WORD_COLORED, ANSI_BLUE, ANSI_RESET, ANSI_VARIED, ins->full_ins, ANSI_RESET);
            snprintf(str, sizeof(str), FORMAT_STRING_WORD, ins->full_ins);
            len += strlen(str);
        }
        break;
    }
}

bool is_directive(Instruction *ins) {
    int part = ins->full_ins >> 9;
    return (part == 1) || (part == 2) || (part == 3) || (ins->full_ins >> 8) == 1;
}

void print_instruction(Instruction *ins, Instruction *ins2, JumpVector *jumpsHere) {
    char *op = match_opcode(ins);
    char str[50] = "";
    int counter = 0;
    if (((ins->full_ins & 0xff) != 0) && ((ins->full_ins & 0xff00) >> 8 == 0)) { // check upper and lower 8 bits
        in_char = true;
        next_in_char = ((ins2->full_ins & 0xff) != 0) && ((ins2->full_ins & 0xff00) >> 8 == 0);
    } else {
        in_char = false;
        printed_addr = false;
        next_in_char = false;
    }
    if (args.concat_chars && in_char) {
        char temp[10];
        switch (ins->full_ins) {
        case '\n':
            strcpy(temp, "\\n");
            break;
        case '\t':
            strcpy(temp, "\\t");
            break;
        case '\\':
            strcpy(temp, "\\\\");
            break;
        default:
            if (ins->full_ins >= 32 && ins->full_ins < 127) {
                temp[0] = (char) ins->full_ins;
                temp[1] = '\0';
            } else {
                strcpy(temp, "?");
            }
        }

        strcat(global_str, temp);
    }

    if (!args.only_code) {
        for (size_t i = 0; i < jumpsHere->size; i++) {
            if (jumpsHere->data[i].destination == current_addr) {
                counter++;
            }
        }
        if (counter > 1) {
            likely_label = true;
        } else {
            likely_label = false;
        }
        if ((ins->full_ins >> 8) != 1) {
            print_instruction_header(current_addr, is_directive(ins));
            print_binary(ins->full_ins);
        } else {
            return;
        }
    }

    // Instruction printing begins here
    if (ins->opcode != RET_OP && ins->opcode != HLT_OP) {
        print_operation(ins, op);
    }
    bool two_reg_args = ins->opcode == ADD_OP || ins->opcode == DIV_OP || ins->opcode == NAND_OP || ins->opcode == MOV_OP || ins->opcode == CMP_OP;

    if (two_reg_args) {
        print_two_reg_args(ins); // add, mov, div, etc.
        goto finish;
    }
    bool sign;
    int8_t val;
    switch (ins->opcode) {
    case BO_OP:
    case BZ_OP:
    case JMP_OP:
        print_jump_instruction(ins);
        break;
    case RET_OP:
        if ((ins->full_ins & 0xfff) == 0) {
            PRINTF("%sret%s", ANSI_BLUE, ANSI_RESET);
            len += 3;
        } else {
            if ((ins->destination >> 2) == 1) {
                PRINTF("%sbg %s", ANSI_BLUE, ANSI_RESET);
                len += 3;
            } else if ((ins->destination >> 2) == 0) {
                PRINTF("%sbl %s", ANSI_BLUE, ANSI_RESET);
                len += 3;
            }
            print_jump_instruction(ins);
        }
        break;
    case INT_OP:
        sign = (ins->source >> 7) == 1;

        val = (int8_t) ins->source & 0x7f;

        PRINTF(FMTSC, ANSI_VARIED, args.hex_operands ? (sign ? ins->source : val) : val, ANSI_RESET);
        snprintf(str, sizeof(str), FORMAT_STRING, args.hex_operands ? (sign ? ins->source : val) : val);
        break;
    case HLT_OP:
        print_hlt_instruction(ins);
        break;
    case LD_OP:
    case LEA_OP:
        PRINTF("%sr%d%s, ", ANSI_YELLOW, ins->destination & 0xF, ANSI_RESET);
        snprintf(str, sizeof(str), "r%d, ", ins->destination);

        PRINTF(FORMAT_STRING_MEM_COLORED, ANSI_VARIED, ins->full_ins & 511, ANSI_RESET);
        char tempstr[30];
        snprintf(tempstr, sizeof(tempstr), FORMAT_STRING_MEM, ins->source);
        len += strlen(tempstr);
        break;
    case ST_OP:
        if (ins->destination >> 2 == 1) {
            PRINTF("%s&r%d%s, %sr%d%s", ANSI_YELLOW, (ins->full_ins & 0x380) >> 7,
                   ANSI_RESET, ANSI_YELLOW, ins->source & 0xF, ANSI_RESET);
            snprintf(str, sizeof(str), "&r%d, r%d", (ins->full_ins & 0x380) >> 7, ins->source & 0x7);
        } else {
            ins->source &= 0x7;
            ins->destination = (ins->full_ins & 0xff8) >> 3;
            PRINTF(FORMAT_STRING_ST_COLORED, ANSI_VARIED, ins->destination, ANSI_RESET, ANSI_YELLOW,
                   ins->source, ANSI_RESET);
            snprintf(str, sizeof(str), FORMAT_STRING_ST, ins->destination, ins->source);
        }
        break;
    case PUSH_OP:
    case POP_OP:
        if ((ins->type == 0 && strcmp(op, "push") == 0)) {
            PRINTF("%sr%d%s", ANSI_YELLOW, ins->source & 0xF, ANSI_RESET);
            snprintf(str, sizeof(str), "r%d", ins->source & 0xF);
        } else {
            if (strcmp(op, "push") == 0) {
                PRINTF(FORMAT_STRING_COLORED, ANSI_VARIED, ins->full_ins & 2047, ANSI_RESET);
                snprintf(str, sizeof(str), FORMAT_STRING, ins->source);
            } else {
                if (ins->destination == 0x4) {
                    PRINTF(FORMAT_STRING_MEM_COLORED, ANSI_VARIED, ins->full_ins & 2047, ANSI_RESET);
                    snprintf(str, sizeof(str), FORMAT_STRING_MEM, ins->full_ins & 2047);
                } else {
                    PRINTF("%sr%d%s", ANSI_YELLOW, ins->source & 0xF, ANSI_RESET);
                    snprintf(str, sizeof(str), "r%d", ins->source & 0xF);
                }
            }
        } // push + pop
        break;
    }
    //
finish:
    if (strcmp(op, "bz") != 0 && strcmp(op, "bo") != 0 && strcmp(op, "jmp") != 0 && !two_reg_args) {
        len += strlen(str);
    }
    if (in_char && args.concat_chars) {
        if (!is_directive(ins)) {
            current_addr++;
        }
        return;
    }
    size_t spaces = 16 - len;
    if (!args.only_code) {
        for (size_t s = 0; s < spaces; s++) {
            PRINTF(" ");
        }
    }
    bool has_jump = false;
    bool has_outgoing_jump = false;
    if (!is_directive(ins)) {
        if (!args.only_code && !args.no_jump) {
            for (size_t i = 0; i < jumpsHere->size; i++) {
                char *color = color_to_ansi(jumpsHere->data[i].color);
                if (!args.colors) {
                    color = ANSI_RESET;
                }
                if (current_addr == jumpsHere->data[i].destination && !has_jump) {
                    if (has_outgoing_jump)
                        PRINTF(" ");
                    PRINTF("%s◀%s", color, ANSI_RESET);
                    if (likely_label) {
                        PRINTF("%s [ LIKELY LABEL ]%s", POSSIBLE_ANSI_BOLD, ANSI_RESET);
                    }
#if defined(_WIN32)
                    PRINTF("%s from 0x%llX%s", color, jumpsHere->data[i].source, ANSI_RESET);
#elif defined(__APPLE__)
                    PRINTF("%s from 0x%llX%s", color, jumpsHere->data[i].source, ANSI_RESET);
#else
                    PRINTF("%s from 0x%lX%s", color, jumpsHere->data[i].source, ANSI_RESET);
#endif
                    if (i < jumpsHere->size - 1) {
                        PRINTF(", ");
                    }
                    has_jump = true;
                } else if (current_addr == jumpsHere->data[i].source) {
                    PRINTF("%s▶%s", color, ANSI_RESET);
#if defined(_WIN32)
                    PRINTF("%s to 0x%llX%s ", color, jumpsHere->data[i].destination, ANSI_RESET);
#elif defined(__APPLE__)
                    PRINTF("%s to 0x%llX%s ", color, jumpsHere->data[i].destination, ANSI_RESET);
#else
                    PRINTF("%s to 0x%lX%s ", color, jumpsHere->data[i].destination, ANSI_RESET);
#endif
                    has_outgoing_jump = true;
                } else if (current_addr == jumpsHere->data[i].destination && has_jump) {
#if defined(_WIN32)
                    PRINTF("%s0x%llX%s", color, jumpsHere->data[i].source, ANSI_RESET);
#elif defined(__APPLE__)
                    PRINTF("%s0x%llX%s", color, jumpsHere->data[i].source, ANSI_RESET);
#else
                    PRINTF("%s0x%lX%s", color, jumpsHere->data[i].source, ANSI_RESET);
#endif
                    if (i < jumpsHere->size - 1) {
                        PRINTF(", ");
                    } else {
                        PRINTF(" ");
                    }
                }
            }
        }
        current_addr++;
    }
    PRINTF("\n");
}

#define MAX_LINES 100
#define MAX_LENGTH 256
void print_binary(int16_t num) {
    if (in_char && !next_in_char && args.concat_chars)
        goto print_str;
    if (printed_addr && in_char && args.concat_chars)
        return;

    char hex[5];
    hex[4] = '\0';
    int numclone = num;
    for (int i = 0; i < 4; i++) {
        hex[3 - i] = "0123456789ABCDEF"[num & 0xF];
        num >>= 4;
    }

    for (int i = 0; i < 4; i += 2) {
        if (in_char && args.concat_chars) {
            PRINTF("     ");
            break;
        }
        PRINTF("%s%c%c%s", ANSI_CYAN, hex[i], hex[i + 1], ANSI_RESET);
        if (i != 2) {
            PRINTF(" ");
        }
    }
    if (args.binary) {
        if (in_char && args.concat_chars) {
            PRINTF("                   ");
            goto end;
        }
        PRINTF(" %s0b%s", ANSI_MAGENTA, ANSI_RESET);

        for (int i = 15; i >= 0; i--) {
            PRINTF("%s%d%s", ANSI_MAGENTA, (numclone >> i) & 1, ANSI_RESET);
        }
    }
end:
    if (in_char)
        printed_addr = true;
    PRINTF(" │ ");
    return;
print_str:
    PRINTF("%s%s%s\n", ANSI_BRIGHT_GREEN, global_str, ANSI_RESET);
    strncpy(global_str, "", 512);
}

#define PRINT_COLOR_AND_VALUE(color, format, value) \
    do {                                            \
        PRINTF("%s", color);                        \
        PRINTF(format, value);                      \
        PRINTF(ANSI_RESET);                         \
    } while (0)

void print_instruction_header(size_t line, bool is_directive) {
    if (printed_addr && in_char && args.concat_chars)
        return;
    PRINTF("│ ");
    char hex[5];
    hex[4] = '\0';
    for (int i = 0; i < 4; i++) {
        hex[3 - i] = "0123456789ABCDEF"[line & 0xF];
        line >>= 4;
    }

    for (int i = 0; i < 4; i += 2) {
        if (in_char && args.concat_chars) {
            PRINT_COLOR_AND_VALUE(ANSI_RED, "%s", "     ");
            break;
        }
        if (is_directive) {
            PRINT_COLOR_AND_VALUE(ANSI_RED, "%s", "XX XX  ");
            break;
        }
        char tmpstr[5];
        snprintf(tmpstr, sizeof(tmpstr), "%c%c", hex[i], hex[i + 1]);
        PRINT_COLOR_AND_VALUE(ANSI_CYAN, "%s", tmpstr);
        if (i != 2) {
            PRINTF(" ");
        }
    }

    if (!is_directive) {
        if (likely_label) {
            PRINTF("%s ●", ANSI_RESET);
        } else {
            PRINTF("  ");
        }
        if (in_char && args.concat_chars) {
            PRINTF(" │ ");
            return;
        }
    }

    PRINTF(" │ ");
}

#define PRINT_HEADER(colors, format, ...) \
    PRINTF(format, __VA_ARGS__);

void print_header(const char *metadata, char *filename) {
    char fsize[15];
    get_file_size(filename, fsize, sizeof(fsize));
    char fdate[30];
    char *fversion = "unknown";
    switch (bin_version) {
    case 1:
        fversion = "0.1";
        break;
    case 2:
        fversion = "0.2";
        break;
    case 3:
        fversion = "0.3";
        break;
    case 4:
        fversion = "0.4";
        break;
    case 5:
        fversion = "0.5";
        break;
    }
    get_last_modified_date(filename, fdate, sizeof(fdate));
    if (!args.only_code) {
        PRINTF("╭──────────────────────────────────────────────────╮\n"
               "│ %sfile%s: %s%-42s%s │\n"
               "├───────────────────────────────┬──────────────────┤\n"
               "│ %smodified%s: %s%-19s%s │ %ssize%s: %s%-10s%s │\n"
               "│ %sbinary version%s: %s%-13s%s ╰──────────────────┤\n",
               ANSI_BOLD, ANSI_RESET, ANSI_GREEN, filename, ANSI_RESET, ANSI_BOLD, ANSI_RESET, ANSI_BRIGHT_CYAN, fdate, ANSI_RESET, ANSI_BOLD, ANSI_RESET, ANSI_RED, fsize, ANSI_RESET, ANSI_MAGENTA, ANSI_RESET, ANSI_GREEN, fversion, ANSI_RESET);
        if (*metadata != '\0') {
            char metadata_buffer[1024];
            strncpy(metadata_buffer, metadata, sizeof(metadata_buffer));
            metadata_buffer[sizeof(metadata_buffer) - 1] = '\0';
            char lines[MAX_LINES][MAX_LENGTH];
            int lineCount = 0;
            int max_length = 49;

            char *line = strtok((char *) metadata, "\n");
            while (line != NULL) {
                strncpy(lines[lineCount], line, MAX_LENGTH);
                lines[lineCount][MAX_LENGTH - 1] = '\0';
                lineCount++;

                int length = strlen(line);
                if (length > max_length) {
                    max_length = length;
                }

                line = strtok(NULL, "\n");
            }
            PRINTF("╞═══════════════════════╡ %sMETADATA%s ╞═══════════════╧", ANSI_BRIGHT_GREEN, ANSI_RESET);
            for (int i = 0; i < max_length - 49; i++) {
                PRINTF("═");
            }
            PRINTF("╕\n");
            for (char *p = metadata_buffer; *p != '\0'; p++) {
                if (*p == '\n') {
                    PRINTF("│ %*s │\n", max_length, " ");
                } else {
                    char *start = p; // Start of the line
                    while (*p != '\n' && *p != '\0') {
                        p++;
                    }
                    PRINTF("│ %.*s", (int) (p - start), start);
                    for (int i = 0; i < max_length - (int) (p - start) + 1; i++) {
                        PRINTF(" ");
                    }
                    PRINTF("│\n");
                }
            }
            PRINTF("╞════════════════════╡ %sEND METADATA%s ╞══════════════╤", ANSI_BRIGHT_RED, ANSI_RESET);
            for (int i = 0; i < max_length - 49; i++) {
                PRINTF("═");
            }
            PRINTF("╛\n");
        }
        if (!args.binary) {
            PRINT_HEADER(args.colors,
                         "├─────────┬───────┬─────────────┬──────────────────╯\n"
                         "│ %saddress%s │  %sbin%s  │ %sinstruction%s │\n"
                         "├─────────┼───────┼─────────────╯\n",
                         ANSI_CYAN, ANSI_RESET, ANSI_MAGENTA, ANSI_RESET, ANSI_BLUE, ANSI_RESET);
        } else {
            PRINT_HEADER(args.colors,
                         "├─────────┬──────────────────────────┬─────────────┤\n"
                         "│ %saddress%s │          %sbinary%s          │ %sinstruction%s │\n"
                         "├─────────┼──────────────────────────┼─────────────╯\n",
                         ANSI_CYAN, ANSI_RESET, ANSI_MAGENTA, ANSI_RESET, ANSI_BLUE, ANSI_RESET);
        }
    }
}
void print_footer(void) {
    if (!args.only_code) {
        if (!args.binary) {
            PRINTF("╰─────────┴───────╯\n");
        } else {
            PRINTF("╰─────────┴──────────────────────────╯\n");
        }
    }
}
const char *valid_options[] = {
    "-h", "--help",
    "-c", "--colorless",
    "-j", "--no-jump",
    "-o", "--only-code",
    "-C", "--concat-chars",
    "-b", "--binary",
    "-X", "--hex",
    "-V", "--version"};

const char *descriptions[] = {
    "Print help and exit",
    "Disable colors",
    "Disable jump visuals",
    "Print only disassembled code",
    "Concatenate characters",
    "Print instruction binary",
    "Print instruction operands in hexadecimal",
    "Print version"};
CLI parse_arguments(int argc, char *argv[]) {
    CLI opts = {0};
    opts.num_files = 0;
    opts.colors = 1;
    int valid_count = sizeof(valid_options) / sizeof(valid_options[0]);

    for (int i = 1; i < argc; i++) {
        if (strcmp(argv[i], "--help") == 0 || strcmp(argv[i], "-h") == 0) {
            print_help(argv[0]);
            exit(EXIT_SUCCESS);
        } else if (argv[i][0] == '-') {
            if (argv[i][1] == '-') {
                if (strcmp(argv[i], "--colorless") == 0) {
                    opts.colors = 0;
                } else if (strcmp(argv[i], "--binary") == 0) {
                    opts.binary = 1;
                } else if (strcmp(argv[i], "--only-code") == 0) {
                    opts.only_code = 1;
                } else if (strcmp(argv[i], "--hex") == 0) {
                    opts.hex_operands = 1;
                } else if (strcmp(argv[i], "--no-jump") == 0) {
                    opts.no_jump = 1;
                } else if (strcmp(argv[i], "--concat-chars") == 0) {
                    opts.concat_chars = 1;
                } else if (strcmp(argv[i], "--version") == 0) {
                    PRINTF("bdump 0.2.0\n");
                    exit(EXIT_SUCCESS);
                } else {
                    fputs("Error: Unknown option ", stderr);
                    fputs(argv[i], stderr);
                    fputc('\n', stderr);
                    suggest_option(argv[i], valid_count);
                    exit(EXIT_FAILURE);
                }
            } else {
                for (int j = 1; argv[i][j] != '\0'; j++) {
                    switch (argv[i][j]) {
                    case 'c':
                        opts.colors = 0;
                        break;
                    case 'b':
                        opts.binary = 1;
                        break;
                    case 'o':
                        opts.only_code = 1;
                        break;
                    case 'X':
                        opts.hex_operands = 1;
                        break;
                    case 'j':
                        opts.no_jump = 1;
                        break;
                    case 'h':
                        print_help(argv[0]);
                        exit(EXIT_SUCCESS);
                        break;
                    case 'C':
                        opts.concat_chars = 1;
                        break;
                    case 'V':
                        PRINTF("bdump 0.2.0\n");
                        exit(EXIT_SUCCESS);
                        break;
                    default:
                        fputs("Error: Unknown option -", stderr);
                        fputc(argv[i][j], stderr);
                        fputc('\n', stderr);
                        suggest_option(argv[i], valid_count);
                        exit(EXIT_FAILURE);
                    }
                }
            }
        } else {
            if (opts.num_files < MAX_INPUT_FILES) {
                opts.input_files[opts.num_files++] = argv[i];
            } else {
                fputs("Error: Too many input files specified\n", stderr);
                exit(EXIT_FAILURE);
            }
        }
    }
    return opts;
}

int levenshtein_distance(const char *s1, const char *s2) {
    int len1 = strlen(s1);
    int len2 = strlen(s2);
    int dp[len1 + 1][len2 + 1];

    for (int i = 0; i <= len1; i++) {
        for (int j = 0; j <= len2; j++) {
            if (i == 0) {
                dp[i][j] = j;
            } else if (j == 0) {
                dp[i][j] = i;
            } else {
                dp[i][j] = (s1[i - 1] == s2[j - 1]) ? dp[i - 1][j - 1] : 1 + fmin(fmin(dp[i - 1][j], dp[i][j - 1]), dp[i - 1][j - 1]);
            }
        }
    }
    return dp[len1][len2];
}

void print_help(char *bin) { // bin is the name of the bin program
    PRINTF("The disassembler for the BELLE-ISA\n\n");
    PRINTF("%s%sUsage:%s %s%s%s [OPTIONS] <ROMS>\n\n", ANSI_UNDERLINE, ANSI_BOLD, ANSI_RESET,
           ANSI_BOLD, bin, ANSI_RESET);
    PRINTF("%s%sArguments:%s\n", ANSI_BOLD, ANSI_UNDERLINE, ANSI_RESET);
    PRINTF("  <ROMS>  Path to ROMs\n\n");
    PRINTF("%s%sOptions:%s\n", ANSI_BOLD, ANSI_UNDERLINE, ANSI_RESET);
    PRINTF("  %s-h%s, %s--help%s          Show this help message and exit\n", ANSI_BOLD, ANSI_RESET,
           ANSI_BOLD, ANSI_RESET);
    PRINTF("  %s-c%s, %s--colorless%s     Disable colors\n", ANSI_BOLD, ANSI_RESET, ANSI_BOLD,
           ANSI_RESET);
    PRINTF("  %s-j%s, %s--no-jump%s       Disable jump visuals\n", ANSI_BOLD, ANSI_RESET, ANSI_BOLD,
           ANSI_RESET);
    PRINTF("  %s-o%s, %s--only-code%s     Print only disassembled code\n", ANSI_BOLD, ANSI_RESET, ANSI_BOLD,
           ANSI_RESET);
    PRINTF("  %s-C%s, %s--concat-chars%s  Concatenate characters\n", ANSI_BOLD, ANSI_RESET, ANSI_BOLD,
           ANSI_RESET);
    PRINTF("  %s-b%s, %s--binary%s        Print instruction binary\n", ANSI_BOLD, ANSI_RESET, ANSI_BOLD, ANSI_RESET);
    PRINTF("  %s-X%s, %s--hex%s           Print instruction operands in hexadecimal\n", ANSI_BOLD, ANSI_RESET, ANSI_BOLD,
           ANSI_RESET);
    PRINTF("  %s-V%s, %s--version%s       Print version\n", ANSI_BOLD,
           ANSI_RESET, ANSI_BOLD, ANSI_RESET);
    exit(0);
}

void suggest_option(const char *invalid_option, int valid_count) {
    int min_distance = 3;
    char *suggestions[MAX_SUGGESTIONS];
    int suggestion_indices[MAX_SUGGESTIONS] = {0};
    int suggestion_count = 0;

    for (int i = 0; i < valid_count; i++) {
        int distance = levenshtein_distance(invalid_option, valid_options[i]);
        if (distance < min_distance && suggestion_count < MAX_SUGGESTIONS) {
            suggestions[suggestion_count++] = (char *) valid_options[i];
            suggestion_indices[suggestion_count] = i;
        }
    }

    if (suggestion_count > 0) {
        fputs("Did you mean one of these options?\n", stderr);
        for (int i = 0; i < suggestion_count; i++) {
            fputs("  ", stderr);
            fputs(suggestions[i], stderr);
            fputs(": ", stderr);
            fputs(ANSI_BOLD, stderr);
            fputs(ANSI_YELLOW, stderr);
            fputs(descriptions[suggestion_indices[i + 1] / 2], stderr);
            fputs(ANSI_RESET, stderr);
            fputc('\n', stderr);
        }
    }
}

void *process_instructions(void *arg, char *filename) {
    char metadata[1024] = "\0";
#ifdef _WIN32
    SetConsoleOutputCP(CP_UTF8);
    setvbuf(stdout, NULL, _IOFBF, 1024);
#endif
    ThreadData *data = (ThreadData *) arg;
    if ((data->buffer[0]) == 1) {
        bin_version = data->buffer[1];
    }
    jump_map_global = jump_map_create();
    for (size_t i = 2; i < data->bytes_read; i += 2) {
        if (i + 1 < data->bytes_read) {
            uint16_t instruction = (data->buffer[i] << 8) | data->buffer[i + 1];
            if ((instruction >> 9) == 1) {
                current_addr = instruction & 0x1ff;
            }
            if (i > 0 && instruction >> 8 == 1) {
                char inschar = (char) instruction & 0xFF;
                strncat(metadata, &inschar, 1);
            }
        }
    }
    int counter = 0;
    size_t current_addr_tmp = current_addr;
    for (size_t i = 0; i < data->bytes_read; i += 2) { // second loop finds jumps
        if (i + 1 < data->bytes_read) {
            uint16_t instruction = (data->buffer[i] << 8) | data->buffer[i + 1];
            switch (instruction >> 12) {
            case JMP_OP:
            case BO_OP:
            case BZ_OP:
            case RET_OP:
                if (instruction >> 12 == RET_OP && (instruction & 0xfff) == 0)
                    break;
                if (((instruction >> 11) & 1) == 1)
                    break;
                Jump jump_data;
                jump_data.source = current_addr;
                jump_data.destination = instruction & 0x7ff;
                jump_data.reverse = jump_data.destination < jump_data.source;
                jump_data.color = get_color(counter++);
                jump_map_insert(jump_map_global, current_addr, jump_data);
                break;
            default:
                break;
            }
            Instruction ins = parse_instruction(instruction);
            if (!is_directive(&ins)) {
                current_addr++;
            }
        }
    }

    print_header(metadata, filename);
    current_addr = current_addr_tmp;
    for (size_t i = 2; i < data->bytes_read; i += 2) { // start at 2 to ignore version
        if (i + 1 < data->bytes_read) {                // third loop adjusts columns and prints
            uint16_t instruction = (data->buffer[i] << 8) | data->buffer[i + 1];
            if (i > 0 && instruction >> 8 == 1) {
                continue; // metadata
            }
            uint16_t next_instruction = (data->buffer[i + 2] << 8) | data->buffer[i + 3];
            Instruction ins = parse_instruction(instruction);
            Instruction ins2 = parse_instruction(next_instruction);
            JumpVector *jumpsHere = find_jumps_at_address(jump_map_global, current_addr);
            len = 0;
            print_instruction(&ins, &ins2, jumpsHere);
            free_jump_vector(jumpsHere);
        }
    }
    print_footer();
    free_map(jump_map_global);
    // jump_map_print(jump_map_global);
    return NULL;
}
int main(int argc, char *argv[]) {
    is_term = is_terminal();
    args = parse_arguments(argc, argv);
    if (args.num_files == 0) {
        print_help(argv[0]);
        return EXIT_FAILURE;
    }

    ThreadData thread_data[THREAD_COUNT];

    for (uint8_t i = 0; i < args.num_files; i++) {
        FILE *input = fopen(args.input_files[i], "rb");
        if (!input) {
            fputs(ANSI_RED_CONST ANSI_BOLD_CONST "Failed to open file \033[0m", stderr);
            perror(args.input_files[i]);
            return EXIT_FAILURE;
        }

        size_t bytes_read = fread(thread_data[0].buffer, sizeof(uint8_t), BUFFER_SIZE, input);
        if (bytes_read > 0) {
            thread_data[0].bytes_read = bytes_read;
            thread_data[0].input = input;
            process_instructions(&thread_data[0], args.input_files[i]);
        }

        fclose(input);
    }

    return EXIT_SUCCESS;
}
char *match_opcode(Instruction *s) {
    switch (s->opcode) {
    case HLT_OP:
        return "hlt";

    case ADD_OP:
        return "add";

    case BO_OP:
        return "bo";

    case POP_OP:
        return "pop";

    case DIV_OP:
        return "div";

    case RET_OP:
        return "ret";

    case LD_OP:
        return "ld";

    case ST_OP:
        return "st";

    case JMP_OP:
        return "jmp";

    case BZ_OP:
        return "bz";

    case CMP_OP:
        return "cmp";

    case NAND_OP:
        return "nand";

    case PUSH_OP:
        return "push";

    case INT_OP:
        return "int";

    case MOV_OP:
        return "mov";

    case LEA_OP:
        return "lea";

    default:
        puts("OPCODE not recognized.");
        exit(1);
    }
}

Instruction parse_instruction(uint32_t instruction) {
    Instruction parsed_ins;
    parsed_ins.opcode = instruction >> 12;
    parsed_ins.destination = (instruction >> 9) & 0x7;
    parsed_ins.source = instruction & 0xFF;
    if (((instruction >> 8) & 1) == 1) {
        parsed_ins.type = 1;
    } else {
        parsed_ins.type = 0;
        if (((instruction >> 7) & 1) == 1)
            parsed_ins.type = 2;
        else if (((instruction >> 6) & 1) == 1)
            parsed_ins.type = 3;
    }
    parsed_ins.full_ins = (int16_t) instruction;
    return parsed_ins;
}
