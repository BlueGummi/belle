#include "jump_map.c"
size_t current_addr = 100;
CLI args = {0};
uint64_t len = 0;
void print_operation(Instruction *ins, char *op, bool colors) {
    bool is_jump = strcmp(op, "jz") == 0 || strcmp(op, "jo") == 0 || strcmp(op, "jmp") == 0;
    bool invert = ins->destination >> 2 == 1;
    if (is_jump && invert) {
        if (strcmp(op, "jz") == 0)
            op = "jnz";
        else if (strcmp(op, "jo") == 0)
            op = "jno";
        else if (strcmp(op, "jmp") == 0)
            op = "jr";
    }

    if (colors) {
        PRINTF("%s%s%s ", ANSI_BLUE, op, ANSI_RESET);
    } else {
        PRINTF("%s ", op);
    }
    char tempstr[40];
    snprintf(tempstr, sizeof(tempstr), "%s ", op);
    len += strlen(tempstr);
}

void print_two_reg_args(Instruction *ins, bool colors) {
    if (colors) {
        PRINTF("%sr%d%s, ", ANSI_YELLOW, ins->destination, ANSI_RESET);
    } else {
        PRINTF("r%d, ", ins->destination);
    }
    char str[20];
    snprintf(str, sizeof(str), "r%d, ", ins->destination);
    len += strlen(str);
    switch (ins->type) {
    case 0: // register
        if (colors) {
            PRINTF("%sr%d%s", ANSI_YELLOW, ins->source, ANSI_RESET);
        } else {
            PRINTF("r%d", ins->source);
        }
        snprintf(str, sizeof(str), "r%d", ins->source);
        break;
    case 1: // literal
    {
        bool sign = (ins->source >> 7) == 1;

        int8_t val = (int8_t) ins->source & 0x7f;

        if (colors) {
            PRINTF(FMTSC, ANSI_VARIED, args.hex_operands ? (sign ? ins->source : val) : val, ANSI_RESET);
        } else {
            PRINTF(FMTS, args.hex_operands ? (sign ? ins->source : val) : val);
        }
        snprintf(str, sizeof(str), FMTS, args.hex_operands ? (sign ? ins->source : val) : val);
    } break;

    case 2: // memory address indirect
    {
        int memaddr = ins->full_ins & 0x7f;

        if (colors) {
            PRINTF(FORMAT_STRING_MEMPTR_COLORED, ANSI_VARIED, memaddr, ANSI_RESET);
        } else {
            PRINTF(FORMAT_STRING_MEMPTR, memaddr);
        }
        snprintf(str, sizeof(str), FORMAT_STRING_MEMPTR, memaddr);
    } break;

    case 3: // register indirect
    {
        if (colors) {
            PRINTF("%s&r%d%s", ANSI_YELLOW, ins->source & 7, ANSI_RESET);
        } else {
            PRINTF("&r%d", ins->source & 7);
        }
        snprintf(str, sizeof(str), "&r%d", ins->source & 7);
    } break;

    default:
        perror("Unknown instruction type\n");
        exit(1);
    }
    len += strlen(str);
}

void print_jump_instruction(Instruction *ins, bool colors) {
    char str[20];
    if (((ins->destination >> 1) & 1) == 1) {
        if (colors) {
            PRINTF("%s&r%d%s", ANSI_YELLOW, ins->source & 7, ANSI_RESET);
        } else {
            PRINTF("&r%d", ins->source & 7);
        }
        snprintf(str, sizeof(str), "&r%d", ins->source & 7);
        len += strlen(str);
        return;
    }

    if (colors) {
        PRINTF(FORMAT_STRING_MEM_COLORED, ANSI_VARIED, ins->full_ins & 1023, ANSI_RESET);
    } else {
        PRINTF(FORMAT_STRING_MEM, ins->full_ins & 1023);
    }
    snprintf(str, sizeof(str), FORMAT_STRING_MEM, ins->full_ins & 1023);
    len += strlen(str);
}

void print_hlt_instruction(Instruction *ins, bool colors) {
    char str[40];
    if (ins->destination == 1) {
        if (colors) {
            PRINTF(FORMAT_STRING_START_COLORED, ANSI_BLUE, ANSI_RESET, ANSI_VARIED, ins->full_ins & 0x1ff, ANSI_RESET);
        } else {
            PRINTF(FORMAT_STRING_START, ins->full_ins & 0x1ff);
        }
        snprintf(str, sizeof(str), FORMAT_STRING_START, ins->full_ins & 0x1ff);
        len += strlen(str);
    } else if (ins->destination == 2) {
        if (colors) {
            PRINTF(FORMAT_STRING_SSP_COLORED, ANSI_BLUE, ANSI_RESET, ANSI_VARIED, ins->full_ins & 0x1ff, ANSI_RESET);
        } else {
            PRINTF(FORMAT_STRING_SSP, ins->full_ins & 0x1ff);
        }
        snprintf(str, sizeof(str), FORMAT_STRING_SSP, ins->full_ins & 0x1ff);
        len += strlen(str);
    } else if (ins->destination == 3) {
        if (colors) {
            PRINTF(FORMAT_STRING_SBP_COLORED, ANSI_BLUE, ANSI_RESET, ANSI_VARIED, ins->full_ins & 0x1ff, ANSI_RESET);
        } else {
            PRINTF(FORMAT_STRING_SBP, ins->full_ins & 0x1ff);
        }
        snprintf(str, sizeof(str), FORMAT_STRING_SBP, ins->full_ins & 0x1ff);
        len += strlen(str);
    } else if (ins->full_ins == 0) {
        if (colors) {
            PRINTF("%shlt%s", ANSI_BLUE, ANSI_RESET);
        } else {
            PRINTF("hlt");
        }
        len += 3;
    } else {
        if (args.concat_chars) {
            char temp[10];
            if (ins->full_ins == '\n') {
                strcpy(temp, "\\n");
            } else if (ins->full_ins == '\t') {
                strcpy(temp, "\\t");
            } else if (ins->full_ins == '\\') {
                strcpy(temp, "\\\\");
            } else if (ins->full_ins >= 32 && ins->full_ins < 127) {
                temp[0] = (char) ins->full_ins;
                temp[1] = '\0';
            } else {
                strcpy(temp, "?");
            }

            strcat(global_str, temp);
            return;
        }
        if (!args.only_code) {
            if (colors) {
                PRINTF(FORMAT_STRING_ASCII_COLORED, ANSI_BLUE,
                       (ins->full_ins == '\n'                          ? "\\n"
                        : ins->full_ins == '\t'                        ? "\\t"
                        : ins->full_ins == '\\'                        ? "\\\\"
                        : (ins->full_ins >= 32 && ins->full_ins < 127) ? (char[]) {(char) ins->full_ins, '\0'}
                                                                       : "???"),
                       ANSI_RESET, ANSI_VARIED, ins->full_ins, ANSI_RESET);
            } else {
                PRINTF(FORMAT_STRING_ASCII,
                       (ins->full_ins == '\n'                          ? "\\n"
                        : ins->full_ins == '\t'                        ? "\\t"
                        : ins->full_ins == '\\'                        ? "\\\\"
                        : (ins->full_ins >= 32 && ins->full_ins < 127) ? (char[]) {(char) ins->full_ins, '\0'}
                                                                       : "???"),
                       ins->full_ins);
            }
            snprintf(str, sizeof(str), FORMAT_STRING_ASCII,
                     (ins->full_ins == '\n'                          ? "\\n"
                      : ins->full_ins == '\t'                        ? "\\t"
                      : ins->full_ins == '\\'                        ? "\\\\"
                      : (ins->full_ins >= 32 && ins->full_ins < 127) ? (char[]) {(char) ins->full_ins, '\0'}
                                                                     : "???"),
                     ins->full_ins);
            len += strlen(str);
        } else {
            if (colors) {
                PRINTF(FORMAT_STRING_WORD_COLORED, ANSI_BLUE, ANSI_RESET, ANSI_VARIED, ins->full_ins, ANSI_RESET);
            } else {
                PRINTF(FORMAT_STRING_WORD, ins->full_ins);
            }
            snprintf(str, sizeof(str), FORMAT_STRING_WORD, ins->full_ins);
            len += strlen(str);
        }
    }
}

bool is_directive(Instruction *ins) {
    int part = ins->full_ins >> 9;
    return (part == 1) || (part == 2) || (part == 3);
}
