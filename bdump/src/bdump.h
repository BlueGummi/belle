#ifndef BDUMP_H
#define BDUMP_H
#define TABLE_SIZE 4096
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
#endif
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
#define ANSI_RESET "\033[0m"
#define ANSI_BOLD (args.colors ? "\033[1m" : "")
#define ANSI_UNDERLINE (args.colors ? "\033[4m" : "")
#define ANSI_BLACK (args.colors ? "\033[30m" : "")
#define ANSI_RED (args.colors ? "\033[31m" : "")
#define ANSI_GREEN (args.colors ? "\033[32m" : "")
#define ANSI_YELLOW (args.colors ? "\033[33m" : "")
#define ANSI_BLUE (args.colors ? "\033[34m" : "")
#define ANSI_MAGENTA (args.colors ? "\033[35m" : "")
#define ANSI_CYAN (args.colors ? "\033[36m" : "")
#define ANSI_WHITE (args.colors ? "\033[37m" : "")
#define ANSI_GRAY (args.colors ? "\033[90m" : "")
#define ANSI_LIGHT_GRAY (args.colors ? "\033[37m" : "")
#define ANSI_BG_BLACK (args.colors ? "\033[40m" : "")
#define ANSI_BG_RED (args.colors ? "\033[41m" : "")
#define ANSI_BG_GREEN (args.colors ? "\033[42m" : "")
#define ANSI_BG_YELLOW (args.colors ? "\033[43m" : "")
#define ANSI_BG_BLUE (args.colors ? "\033[44m" : "")
#define ANSI_BG_MAGENTA (args.colors ? "\033[45m" : "")
#define ANSI_BG_CYAN (args.colors ? "\033[46m" : "")
#define ANSI_BG_WHITE (args.colors ? "\033[47m" : "")
#define ANSI_BRIGHT_BLACK (args.colors ? "\033[90m" : "")
#define ANSI_BRIGHT_RED (args.colors ? "\033[91m" : "")
#define ANSI_BRIGHT_GREEN (args.colors ? "\033[92m" : "")
#define ANSI_BRIGHT_YELLOW (args.colors ? "\033[93m" : "")
#define ANSI_BRIGHT_BLUE (args.colors ? "\033[94m" : "")
#define ANSI_BRIGHT_MAGENTA (args.colors ? "\033[95m" : "")
#define ANSI_BRIGHT_CYAN (args.colors ? "\033[96m" : "")
#define ANSI_BRIGHT_WHITE (args.colors ? "\033[97m" : "")
#define ANSI_RED_CONST "\033[31m"
#define ANSI_BOLD_CONST "\033[1m"
