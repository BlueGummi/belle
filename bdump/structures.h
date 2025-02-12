#ifndef STRUCTURES_H
#define STRUCTURES_H

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
#endif
