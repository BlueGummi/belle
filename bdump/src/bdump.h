#ifndef BDUMP_H
#define BDUMP_H
#define TABLE_SIZE 512
#include "consts.h"

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
void print_jump_instruction(Instruction *ins, bool colors);
void print_hlt_instruction(Instruction *ins, bool colors);
bool is_directive(Instruction *ins);
void print_two_reg_args(Instruction *ins, bool colors);
void jump_map_insert(HashMap *map, size_t key, Jump value);
Jump *jump_map_get(HashMap *map, size_t key);
void free_jump_vector(JumpVector *vector);
void print_operation(Instruction *ins, char *op, bool colors);
void init_jump_vector(JumpVector *vector);
JumpVector *find_jumps_at_address(HashMap *jump_map, uint64_t address);
void add_jump(JumpVector *vector, Jump jump);
void free_map(HashMap *map);
unsigned int hash(size_t key);
char *match_opcode(Instruction *s);
int main(int argc, char *argv[]);
void print_instruction_header(size_t line, bool colors, bool is_directive);
HashMap *jump_map_create(void);
void print_header(char *filename);
void print_footer(void);
void suggest_option(const char *invalid_option, int valid_count);
int levenshtein_distance(const char *s1, const char *s2);
Color get_color(int index);
char *color_to_ansi(Color color);
void get_file_size(const char *filename, char *size_str, size_t size_str_len);
#endif
#define FORMAT_STRING_MEMPTR (args.hex_operands ? "&0x%X" : "&%d")
#define FORMAT_STRING_MEMPTR_COLORED (args.hex_operands ? "&%s0x%X%s" : "&%s%d%s")
#define FORMAT_STRING_MEM_COLORED (args.hex_operands ? "[%s0x%X%s]" : "[%s%d%s]")
#define FORMAT_STRING_MEM (args.hex_operands ? "[0x%X]" : "[%d]")
#define FORMAT_STRING_COLORED (args.hex_operands ? "%s0x%X%s" : "%s%d%s")
#define FORMAT_STRING (args.hex_operands ? "0x%X" : "%d")
#define FORMAT_STRING_SIGNED (args.hex_operands ? "0x%X" : "-%d")

#define FORMAT_STRING_WORD_COLORED (args.hex_operands ? "%s.word%s %s0x%X%s" : "%s.word%s %s%d%s")
#define FORMAT_STRING_WORD (args.hex_operands ? ".word 0x%X" : ".word %d")

#define FORMAT_STRING_START (args.hex_operands ? ".start [0x%X]" : ".start [%d]")
#define FORMAT_STRING_START_COLORED (args.hex_operands ? "%s.start%s [%s0x%X%s]" : "%s.start%s [%s%d%s]")
#define FORMAT_STRING_SSP (args.hex_operands ? ".ssp [0x%X]" : ".ssp [%d]")
#define FORMAT_STRING_SBP (args.hex_operands ? ".sbp [0x%X]" : ".sbp [%d]")
#define FORMAT_STRING_SSP_COLORED (args.hex_operands ? "%s.ssp%s [%s0x%X%s]" : "%s.ssp%s [%s%d%s]")
#define FORMAT_STRING_SBP_COLORED (args.hex_operands ? "%s.sbp%s [%s0x%X%s]" : "%s.sbp%s [%s%d%s]")

#define FORMAT_STRING_ASCII_COLORED (args.hex_operands ? "%s%s%s (%s0x%X%s)" : "%s%s%s (%s%d%s)")
#define FORMAT_STRING_ASCII (args.hex_operands ? "%s (0x%X)" : "%s (%d)")

#define FORMAT_STRING_ST_COLORED (args.hex_operands ? "[%s0x%X%s], %sr%d%s" : "[%s%d%s], %sr%d%s")
#define FORMAT_STRING_ST (args.hex_operands ? "[0x%X], r%d" : "[%d], r%d")
#define ANSI_VARIED (args.hex_operands ? ANSI_CYAN : ANSI_GREEN)
#define FORMAT_STRING_COLORED_SIGNED (args.hex_operands ? "%s0x%X%s" : "-%s%d%s")
#define POSSIBLE_ANSI_BOLD (args.colors ? ANSI_BOLD : "")
#define FMTS (sign ? FORMAT_STRING_SIGNED : FORMAT_STRING)
#define FMTSC (sign ? FORMAT_STRING_COLORED_SIGNED : FORMAT_STRING_COLORED)
