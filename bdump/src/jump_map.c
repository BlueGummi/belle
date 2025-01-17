#include "misc.c"
#define TABLE_SIZE 512

typedef struct Node {
    size_t key;
    Jump value;
    struct Node *next;
} Node;

typedef struct {
    Node *table[TABLE_SIZE];
} HashMap;

unsigned int hash(size_t key) {
    return key % TABLE_SIZE;
}
size_t max_columns = 0;
HashMap *jump_map_create() {
    HashMap *map = malloc(sizeof(HashMap));
    for (int i = 0; i < TABLE_SIZE; i++) {
        map->table[i] = NULL;
    }
    return map;
}

void jump_map_insert(HashMap *map, size_t key, Jump value) {
    unsigned int index = hash(key);
    Node *newNode = malloc(sizeof(Node));
    newNode->key = key;
    newNode->value = value;
    newNode->next = map->table[index];
    map->table[index] = newNode;
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

void print_jump(Jump *jump) {

    printf("Jump: { ");
    printf("color: %s, ", get_color_name(jump->color));
    printf("source: %" PRIu64 ", ", jump->source);
    printf("destination: %" PRIu64 ", ", jump->destination);
    printf("column: %d, ", jump->column);
    printf("reverse: %s ", jump->reverse ? "\033[32mtrue\033[0m" : "\033[31mfalse\033[0m");
    printf("}");
}
void jump_map_print(HashMap *map) {
    for (int i = 0; i < TABLE_SIZE; i++) {
        Node *current = map->table[i];
        if (current == NULL) {
            continue;
        }
        while (current) {
            printf("Key: %zu, ", current->key);
            print_jump(&current->value);
            current = current->next;
        }
    }
}
void init_jump_vector(JumpVector *vector) {
    vector->size = 0;
    vector->capacity = 4;
    vector->data = malloc(vector->capacity * sizeof(Jump));
}

void add_jump(JumpVector *vector, Jump jump) {
    if (vector->size >= vector->capacity) {
        vector->capacity *= 2;
        vector->data = realloc(vector->data, vector->capacity * sizeof(Jump));
        if (vector->data == NULL) {
            fprintf(stderr, "Memory allocation failed\n");
            exit(EXIT_FAILURE);
        }
    }
    vector->data[vector->size++] = jump;
}

void free_jump_vector(JumpVector *vector) {
    free(vector->data);
    vector->data = NULL;
    vector->size = 0;
    vector->capacity = 0;
}

JumpVector *find_jumps_at_address(HashMap *jump_map, uint64_t address) {
    JumpVector *jump_vector = malloc(sizeof(JumpVector));
    if (jump_vector == NULL) {
        fprintf(stderr, "Memory allocation failed for JumpVector\n");
        return NULL;
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
    }

    return jump_vector;
}

HashMap *jump_map_global;
void adjust_jump_vector(JumpVector *vector) {
    for (size_t i = 0; i < vector->size; i++) {
        JumpVector *tempvector = find_jumps_at_address(jump_map_global, vector->data[i].source);
        for (size_t s = 0; s < tempvector->size; s++) {
            tempvector->data[s].column = s;
        }
        vector->data[i] = tempvector->data[i];
        free(tempvector);
        if ((i + 1) > max_columns)
            max_columns = i + 1;
    }
}
