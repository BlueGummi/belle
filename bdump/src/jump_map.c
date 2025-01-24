#include "misc.c"
unsigned int hash(size_t key) {
    return key % TABLE_SIZE;
}

size_t max_columns = 0;
HashMap *jump_map_create(void) {
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
}

void add_jump(JumpVector *vector, Jump jump) {
    if (vector->size >= vector->capacity) {
        vector->capacity *= 2;
        vector->data = realloc(vector->data, vector->capacity * sizeof(Jump));
        if (vector->data == NULL) {
            perror("Memory allocation failed\n");
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
        perror("Memory allocation failed for JumpVector\n");
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
        free_node(current);
    }
    return jump_vector;
}

HashMap *jump_map_global;
