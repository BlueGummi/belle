#include "ins_print_helpers.c"

void print_instruction(Instruction *ins, Instruction *ins2, JumpVector *jumpsHere) {
    bool colors = args.colors;
    char *op = match_opcode(ins);
    char str[50] = "";
    int counter = 0;
    if (((ins->full_ins & 0xff) != 0) && ((ins->full_ins & 0xff00) >> 8 == 0)) {
        in_char = true;
        if (((ins2->full_ins & 0xff) != 0) && ((ins2->full_ins & 0xff00) >> 8 == 0)) {
            next_in_char = true;
        } else {
            next_in_char = false;
        }
    } else {
        in_char = false;
        printed_addr = false;
        next_in_char = false;
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
        if ((ins->full_ins & 0xfff) == 0) {
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
            printf("%sr%d%s, ", ANSI_YELLOW, ins->destination & 7, ANSI_RESET);
        } else {
            printf("r%d, ", ins->destination & 7);
        }
        snprintf(str, sizeof(str), "r%d, ", ins->destination);

        if (colors) {
            printf(FORMAT_STRING_MEM_COLORED, ANSI_VARIED, ins->full_ins & 511, ANSI_RESET);
        } else {
            printf(FORMAT_STRING_MEM, ins->full_ins & 511);
        }
        char tempstr[30];
        snprintf(tempstr, sizeof(tempstr), FORMAT_STRING_MEM, ins->source);
        len += strlen(tempstr);
    } else if (strcmp(op, "st") == 0) {
        if (ins->destination >> 2 == 1) {
            if (colors) {
                printf("%s&r%d%s, %sr%d%s", ANSI_YELLOW, (ins->full_ins & 0x380) >> 7,
                       ANSI_RESET, ANSI_YELLOW, ins->source & 7, ANSI_RESET);
            } else {
                printf("&r%d, r%d", (ins->full_ins & 0x380) >> 7,
                       (ins->source & 0x7));
            }
            snprintf(str, sizeof(str), "&r%d, r%d", (ins->full_ins & 0x380) >> 7, ins->source & 0x7);
        } else {
            ins->source &= 0x7;
            ins->destination = (ins->full_ins & 0xff8) >> 3;
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
                printf("%sr%d%s", ANSI_YELLOW, ins->source & 7, ANSI_RESET);
            } else {
                printf("r%d", ins->source & 7);
            }
            snprintf(str, sizeof(str), "r%d", ins->source & 7);
        } else {
            if (strcmp(op, "push") == 0) {
                if (colors) {
                    printf(FORMAT_STRING_COLORED, ANSI_VARIED, ins->full_ins & 2047, ANSI_RESET);
                } else {
                    printf(FORMAT_STRING, ins->source & 2047);
                }
                snprintf(str, sizeof(str), FORMAT_STRING, ins->source);
            } else {
                if (ins->destination == 0x4) {
                    if (colors) {
                        printf(FORMAT_STRING_MEM_COLORED, ANSI_VARIED, ins->full_ins & 2047, ANSI_RESET);
                    } else {
                        printf(FORMAT_STRING_MEM, ins->full_ins & 2047);
                    }
                    snprintf(str, sizeof(str), FORMAT_STRING_MEM, ins->full_ins & 2047);
                } else {
                    if (colors) {
                        printf("%sr%d%s", ANSI_YELLOW, ins->source & 7, ANSI_RESET);
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
    if (in_char && args.concat_chars) {
        if (!is_directive(ins)) {
            current_addr++;
        }
        return;
    }
    size_t spaces = 16 - len;
    for (size_t s = 0; s < spaces; s++)
        printf(" ");
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
                        printf(" ");
                    printf("%s◀%s", color, ANSI_RESET);
                    if (likely_label) {
                        printf("%s [ LIKELY LABEL ]%s", POSSIBLE_ANSI_BOLD, ANSI_RESET);
                    }
#if defined(_WIN32)
                    printf("%s from 0x%llX%s", color, jumpsHere->data[i].source, ANSI_RESET);
#elif defined(__APPLE__)
                    printf("%s from 0x%lX%s", color, jumpsHere->data[i].source, ANSI_RESET);
#else
                    printf("%s from 0x%lX%s", color, jumpsHere->data[i].source, ANSI_RESET);
#endif
                    if (i < jumpsHere->size - 1) {
                        printf(", ");
                    }
                    has_jump = true;
                } else if (current_addr == jumpsHere->data[i].source) {
                    printf("%s▶%s", color, ANSI_RESET);
#if defined(_WIN32)
                    printf("%s to 0x%llX%s ", color, jumpsHere->data[i].destination, ANSI_RESET);
#elif defined(__APPLE__)
                    printf("%s to 0x%lX%s ", color, jumpsHere->data[i].destination, ANSI_RESET);
#else
                    printf("%s to 0x%lX%s ", color, jumpsHere->data[i].destination, ANSI_RESET);
#endif
                    has_outgoing_jump = true;
                } else if (current_addr == jumpsHere->data[i].destination && has_jump) {
#if defined(_WIN32)
                    printf("%s0x%llX%s", color, jumpsHere->data[i].source, ANSI_RESET);
#elif defined(__APPLE__)
                    printf("%s0x%lX%s", color, jumpsHere->data[i].source, ANSI_RESET);
#else
                    printf("%s0x%lX%s", color, jumpsHere->data[i].source, ANSI_RESET);
#endif
                    if (i < jumpsHere->size - 1) {
                        printf(", ");
                    } else {
                        printf(" ");
                    }
                }
            }
        }
        current_addr++;
    }
    printf("\n");
}
