#include "bdump.h"
size_t current_addr = 100;
CLI    args         = {0};
void   print_operation(Instruction *ins, char *op, bool colors) {
    bool is_jump = strcmp(op, "jz") == 0 || strcmp(op, "jo") == 0 || strcmp(op, "jmp") == 0;
    bool invert  = ins->destination >> 2 == 1;
    if (is_jump && invert) {
        if (strcmp(op, "jz") == 0)
            op = "jnz";
        else if (strcmp(op, "jo") == 0)
            op = "jno";
        else if (strcmp(op, "jmp") == 0)
            op = "jr";
    }

    if (colors) {
        printf("%s%s%s ", ANSI_BLUE, op, ANSI_RESET);
    } else {
        printf("%s ", op);
    }
}

void print_two_reg_args(Instruction *ins, bool colors) {
    if (colors) {
        printf("%sr%d%s, ", ANSI_GREEN, ins->destination, ANSI_RESET);
    } else {
        printf("r%d, ", ins->destination);
    }

    switch (ins->type) {
    case 0: // register
        if (colors) {
            printf("%sr%d%s", ANSI_YELLOW, ins->source, ANSI_RESET);
        } else {
            printf("r%d", ins->source);
        }
        break;

    case 1: // literal
    {
        bool sign = (ins->source >> 7) == 1;

        int8_t val = (int8_t)ins->source & 0b1111111;

        if (colors) {
            printf(FMTSC, ANSI_VARIED, args.hex_operands ? (sign ? ins->source : val) : val, ANSI_RESET);
        } else {
            printf(FMTS, args.hex_operands ? (sign ? ins->source : val) : val);
        }
    } break;

    case 2: // memory address indirect
    {
        int memaddr = ins->full_ins & 0b1111111;

        if (colors) {
            printf(FORMAT_STRING_MEMPTR_COLORED, ANSI_VARIED, memaddr, ANSI_RESET);
        } else {
            printf(FORMAT_STRING_MEMPTR, memaddr);
        }
    } break;

    case 3: // register indirect
    {
        if (colors) {
            printf("%s&r%d%s", ANSI_YELLOW, ins->source & 7, ANSI_RESET);
        } else {
            printf("&r%d", ins->source & 7);
        }
    } break;

    default:
        perror("Unknown instruction type\n");
        exit(1);
    }
}

void print_jump_instruction(Instruction *ins, bool colors) {
    if (((ins->destination >> 1) & 1) == 1) {
        if (colors) {
            printf("%s&r%d%s", ANSI_GREEN, ins->source, ANSI_RESET);
        } else {
            printf("&r%d", ins->source);
        }
        return;
    }

    if (colors) {
        printf(FORMAT_STRING_MEM_COLORED, ANSI_VARIED, ins->full_ins & 1023, ANSI_RESET);
    } else {
        printf(FORMAT_STRING_MEM, ins->full_ins & 1023);
    }
}

void print_hlt_instruction(Instruction *ins, bool colors) {
    if (ins->destination == 1) {
        if (colors) {
            printf(FORMAT_STRING_START_COLORED, ANSI_BLUE, ANSI_RESET, ANSI_VARIED, ins->full_ins & 0b111111111, ANSI_RESET);
        } else {
            printf(FORMAT_STRING_START, ins->full_ins & 0b111111111);
        }
    } else if (ins->destination == 2) {
        if (colors) {
            printf(FORMAT_STRING_SSP_COLORED, ANSI_BLUE, ANSI_RESET, ANSI_VARIED, ins->full_ins & 0b111111111, ANSI_RESET);
        } else {
            printf(FORMAT_STRING_SSP, ins->full_ins & 0b111111111);
        }
    } else if (ins->destination == 3) {
        if (colors) {
            printf(FORMAT_STRING_SBP_COLORED, ANSI_BLUE, ANSI_RESET, ANSI_VARIED, ins->full_ins & 0b111111111, ANSI_RESET);
        } else {
            printf(FORMAT_STRING_SBP, ins->full_ins & 0b111111111);
        }
    } else if (ins->full_ins == 0) {
        if (colors) {
            printf("%shlt%s", ANSI_BLUE, ANSI_RESET);
        } else {
            printf("hlt");
        }
    } else {
        if (!args.only_code) {
            if (colors) {
                printf(FORMAT_STRING_ASCII_COLORED, ANSI_BLUE,
                       (ins->full_ins == '\n'                          ? "\\n"
                        : ins->full_ins == '\t'                        ? "\\t"
                        : ins->full_ins == '\\'                        ? "\\\\"
                        : (ins->full_ins >= 32 && ins->full_ins < 127) ? (char[]){(char)ins->full_ins, '\0'}
                                                                       : "?"),
                       ANSI_RESET, ANSI_VARIED, ins->full_ins, ANSI_RESET);
            } else {
                printf(FORMAT_STRING_ASCII,
                       (ins->full_ins == '\n'                          ? "\\n"
                        : ins->full_ins == '\t'                        ? "\\t"
                        : ins->full_ins == '\\'                        ? "\\\\"
                        : (ins->full_ins >= 32 && ins->full_ins < 127) ? (char[]){(char)ins->full_ins, '\0'}
                                                                       : "?"),
                       ins->full_ins);
            }
        } else {
            if (colors) {
                printf(FORMAT_STRING_WORD_COLORED, ANSI_BLUE, ANSI_RESET, ANSI_VARIED, ins->full_ins, ANSI_RESET);
            } else {
                printf(FORMAT_STRING_WORD, ins->full_ins);
            }
        }
    }
}

bool is_directive(Instruction *ins) {
    int part = ins->full_ins >> 9;
    return (part == 1) || (part == 2) || (part == 3);
}
