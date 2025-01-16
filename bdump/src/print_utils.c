#include "ins_print_helpers.c"

void print_instruction(Instruction *ins, JumpVector *jumpsHere) {
    bool  colors  = args.colors == 1;
    char *op      = match_opcode(ins);
    char  str[50] = "";
    if (args.only_code != 1) {
        if (!is_directive(ins)) {
            print_instruction_header(current_addr, colors, false);
        } else {
            print_instruction_header(current_addr, colors, true);
        }
        print_binary(ins->full_ins);
    }

    // Instruction printing begins here
    if (strcmp(op, "ret") != 0 && strcmp(op, "hlt") != 0) {
        print_operation(ins, op, colors);
    }
    bool two_reg_args = (strcmp(op, "add") == 0 || strcmp(op, "div") == 0 || strcmp(op, "cmp") == 0 ||
                         strcmp(op, "mul") == 0 || strcmp(op, "mov") == 0);

    if (two_reg_args) {
        print_two_reg_args(ins, colors); // add, mov, div, etc.
    } else if (strcmp(op, "jz") == 0 || strcmp(op, "jo") == 0 || strcmp(op, "jmp") == 0) {
        print_jump_instruction(ins, colors);
    } else if (strcmp(op, "ret") == 0) {
        if ((ins->full_ins & 0b111111111111) == 0) {
            if (colors)
                printf("%sret%s", ANSI_BLUE, ANSI_RESET);
            else
                printf("ret");
            len += 3;
        } else {
            if ((ins->destination >> 2) == 1) {
                if (colors)
                    printf("%sjg %s", ANSI_BLUE, ANSI_RESET);
                else
                    printf("jg ");
                len += 3;
            } else if ((ins->destination >> 2) == 0) {
                if (colors)
                    printf("%sjl %s", ANSI_BLUE, ANSI_RESET);
                else
                    printf("jl ");
                len += 3;
            }
            print_jump_instruction(ins, colors);
        }
    } else if (strcmp(op, "int") == 0) {
        if (colors) {
            printf(FORMAT_STRING_COLORED, ANSI_VARIED, ins->source, ANSI_RESET);
        } else {
            printf(FORMAT_STRING, ins->source);
        }
        snprintf(str, sizeof(str), FORMAT_STRING, ins->source);
    } else if (strcmp(op, "hlt") == 0) {
        print_hlt_instruction(ins, colors);
    } else if (strcmp(op, "ld") == 0 || strcmp(op, "lea") == 0) {
        if (colors) {
            printf("%sr%d%s, ", ANSI_GREEN, ins->destination, ANSI_RESET);
        } else {
            printf("r%d, ", ins->destination);
        }
        snprintf(str, sizeof(str), "r%d, ", ins->destination);
        ins->type = (ins->type << 8) | ins->source;

        if (colors) {
            printf(FORMAT_STRING_MEM_COLORED, ANSI_VARIED, ins->source, ANSI_RESET);
        } else {
            printf(FORMAT_STRING_MEM, ins->source);
        }
        char tempstr[30];
        snprintf(tempstr, sizeof(tempstr), FORMAT_STRING_MEM, ins->source);
        len += strlen(tempstr);
    } else if (strcmp(op, "st") == 0) {
        if (ins->destination >> 2 == 1) {
            if (colors) {
                printf("%s&r%d%s, %sr%d%s", ANSI_GREEN, (ins->full_ins & 0b1110000000) >> 7,
                       ANSI_RESET, ANSI_YELLOW, ins->source & 7, ANSI_RESET);
            } else {
                printf("&r%d, r%d", (ins->full_ins & 0b1110000000) >> 7,
                       (ins->source & 0b111));
            }
            snprintf(str, sizeof(str), "&r%d, r%d", (ins->full_ins & 0b1110000000) >> 7, ins->source & 0b111);
        } else {
            ins->source &= 0b111;
            ins->destination = (ins->full_ins & 0b111111111000) >> 3;
            if (colors) {
                printf(FORMAT_STRING_ST_COLORED, ANSI_VARIED, ins->destination, ANSI_RESET, ANSI_YELLOW,
                       ins->source, ANSI_RESET);
            } else {
                printf(FORMAT_STRING_ST, ins->destination, ins->source);
            }
            snprintf(str, sizeof(str), FORMAT_STRING_ST, ins->destination, ins->source);
        }
    } else if (strcmp(op, "push") == 0 || strcmp(op, "pop") == 0) {
        if ((ins->type == 0 && strcmp(op, "push") == 0)) {
            if (colors) {
                printf("%sr%d%s", ANSI_GREEN, ins->source, ANSI_RESET);
            } else {
                printf("r%d", ins->source);
            }
            snprintf(str, sizeof(str), "r%d", ins->source);
        } else {
            if (strcmp(op, "push") == 0) {
                if (colors) {
                    printf(FORMAT_STRING_COLORED, ANSI_VARIED, ins->full_ins & 2047, ANSI_RESET);
                } else {
                    printf(FORMAT_STRING, ins->source & 2047);
                }
                snprintf(str, sizeof(str), FORMAT_STRING, ins->source);
            } else {
                if (ins->destination == 0b100) {
                    if (colors) {
                        printf(FORMAT_STRING_MEM_COLORED, ANSI_VARIED, ins->full_ins & 2047, ANSI_RESET);
                    } else {
                        printf(FORMAT_STRING_MEM, ins->full_ins & 2047);
                    }
                    snprintf(str, sizeof(str), FORMAT_STRING_MEM, ins->full_ins & 2047);
                } else {
                    if (colors) {
                        printf("%sr%d%s", ANSI_GREEN, ins->source & 7, ANSI_RESET);
                    } else {
                        printf("r%d", ins->source & 7);
                    }
                    snprintf(str, sizeof(str), "r%d", ins->source & 7);
                }
            }
        } // Push
    } // push + pop
    //
    if (strcmp(op, "jz") != 0 && strcmp(op, "jo") != 0 && strcmp(op, "jmp") != 0 && !two_reg_args)
        len += strlen(str);
    if (args.verbosity == 1 && args.only_code == 1) {
        if ((ins->full_ins >> 9) != 1 && (ins->full_ins >> 9) != 2 && (ins->full_ins >> 9) != 3) {
#ifdef _WIN32
            printf(" ; address %lld", current_addr - 1);
#else
            printf(" ; address %ld", current_addr - 1);
#endif
        } else {
            printf(" ; no address");
        }
    }
    size_t spaces = 14 - len;
    for (size_t s = 0; s < spaces; s++)
        printf(" ");
    bool has_jump = false;
    if (!is_directive(ins)) {
        for (size_t i = 0; i < jumpsHere->size; i++) {
            if (current_addr == jumpsHere->data[i].destination && !has_jump) {
                printf("<-");
                has_jump = true;
            } else if (current_addr == jumpsHere->data[i].source) {
                printf("->");
            }
        }

        current_addr++;
    }
    printf("\n");
}
