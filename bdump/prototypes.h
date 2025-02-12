#ifndef PROTOTYPES_H
#define PROTOTYPES_H
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
