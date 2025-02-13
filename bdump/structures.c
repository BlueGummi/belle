#include "bdump.h"

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
    map->table = malloc(sizeof(Node *) * TABLE_SIZE);
    for (int i = 0; i < TABLE_SIZE; i++) {
        map->table[i] = NULL;
    }
    return map;
}

void jump_map_insert(HashMap *map, size_t key, Jump value) {
    unsigned int index = hash(key);
    Node *new_node = malloc(sizeof(Node));
    if (new_node == NULL) {
        perror("Node creation during hashmap insertion memory allocation failed");
        PRINT_LINE_AND_FILE;
        exit(EXIT_FAILURE);
    }
    new_node->key = key;
    new_node->value = value;
    new_node->next = map->table[index];
    map->table[index] = new_node;
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
