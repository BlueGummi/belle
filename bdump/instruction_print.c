#include "bdump.h"

void print_operation(Instruction *ins, char *op) {
    bool is_jump = strcmp(op, "bz") == 0 || strcmp(op, "bo") == 0 || strcmp(op, "jmp") == 0;
    bool invert = ins->destination >> 2 == 1;
    if (is_jump && invert) {
        if (strcmp(op, "bz") == 0)
            op = "bnz";
        else if (strcmp(op, "bo") == 0)
            op = "bno";
    }

    PRINTF("%s%s%s ", ANSI_BLUE, op, ANSI_RESET);
    char tempstr[40];
    snprintf(tempstr, sizeof(tempstr), "%s ", op);
    len += strlen(tempstr);
}

void print_two_reg_args(Instruction *ins) {
    PRINTF("%sr%d%s, ", ANSI_YELLOW, ins->destination, ANSI_RESET);
    char str[20];
    snprintf(str, sizeof(str), "r%d, ", ins->destination);
    len += strlen(str);
    switch (ins->type) {
    case 0: // register
        PRINTF("%sr%d%s", ANSI_YELLOW, ins->source, ANSI_RESET);
        snprintf(str, sizeof(str), "r%d", ins->source);
        break;
    case 1: // literal
    {
        bool sign = (ins->source >> 7) == 1;

        int8_t val = (int8_t) ins->source & 0x7f;

        PRINTF(FMTSC, ANSI_VARIED, args.hex_operands ? (sign ? ins->source : val) : val, ANSI_RESET);
        snprintf(str, sizeof(str), FMTS, args.hex_operands ? (sign ? ins->source : val) : val);
    } break;

    case 2: // memory address indirect
    {
        int memaddr = ins->full_ins & 0x7f;

        PRINTF(FORMAT_STRING_MEMPTR_COLORED, ANSI_VARIED, memaddr, ANSI_RESET);
        snprintf(str, sizeof(str), FORMAT_STRING_MEMPTR, memaddr);
    } break;

    case 3: // register indirect
    {
        PRINTF("%s&r%d%s", ANSI_YELLOW, ins->source & 0xF, ANSI_RESET);
        snprintf(str, sizeof(str), "&r%d", ins->source & 0xF);
    } break;

    default:
        perror("Unknown instruction type\n");
        exit(1);
    }
    len += strlen(str);
}

void print_jump_instruction(Instruction *ins) {
    char str[20];
    if (((ins->destination >> 1) & 1) == 1) {
        PRINTF("%s&r%d%s", ANSI_YELLOW, ins->source & 0xF, ANSI_RESET);
        snprintf(str, sizeof(str), "&r%d", ins->source & 0xF);
        len += strlen(str);
        return;
    }

    PRINTF(FORMAT_STRING_MEM_COLORED, ANSI_VARIED, ins->full_ins & 1023, ANSI_RESET);
    snprintf(str, sizeof(str), FORMAT_STRING_MEM, ins->full_ins & 1023);
    len += strlen(str);
}

void print_hlt_instruction(Instruction *ins) {
    char str[40];
    if (ins->full_ins == 0) {
        PRINTF("%shlt%s", ANSI_BLUE, ANSI_RESET);
        len += 3;
        return;
    } else if ((ins->full_ins >> 8) == 0 && args.concat_chars) {
        return;
    }
    if (!args.only_code) {
        PRINTF(FORMAT_STRING_ASCII_COLORED, ANSI_BLUE,
               (ins->full_ins == '\n'                          ? "\\n"
                : ins->full_ins == '\t'                        ? "\\t"
                : ins->full_ins == '\\'                        ? "\\\\"
                : (ins->full_ins >= 32 && ins->full_ins < 127) ? (char[]) {(char) ins->full_ins, '\0'}
                                                               : "???"),
               ANSI_RESET, ANSI_VARIED, ins->full_ins, ANSI_RESET);
        snprintf(str, sizeof(str), FORMAT_STRING_ASCII,
                 (ins->full_ins == '\n'                          ? "\\n"
                  : ins->full_ins == '\t'                        ? "\\t"
                  : ins->full_ins == '\\'                        ? "\\\\"
                  : (ins->full_ins >= 32 && ins->full_ins < 127) ? (char[]) {(char) ins->full_ins, '\0'}
                                                                 : "???"),
                 ins->full_ins);
        len += strlen(str);
    } else {
        PRINTF(FORMAT_STRING_WORD_COLORED, ANSI_BLUE, ANSI_RESET, ANSI_VARIED, ins->full_ins, ANSI_RESET);
        snprintf(str, sizeof(str), FORMAT_STRING_WORD, ins->full_ins);
        len += strlen(str);
    }
}

bool is_directive(Instruction *ins) {
    int part = ins->full_ins >> 9;
    return (part == 1) || (part == 2) || (part == 3) || (ins->full_ins >> 8) == 1;
}

void print_instruction(Instruction *ins, Instruction *ins2, JumpVector *jumpsHere) {
    char *op = match_opcode(ins);
    char str[50] = "";
    int counter = 0;
    if (((ins->full_ins & 0xff) != 0) && ((ins->full_ins & 0xff00) >> 8 == 0)) { // check upper and lower 8 bits
        in_char = true;
        next_in_char = ((ins2->full_ins & 0xff) != 0) && ((ins2->full_ins & 0xff00) >> 8 == 0);
    } else {
        in_char = false;
        printed_addr = false;
        next_in_char = false;
    }
    if (args.concat_chars && in_char) {
        char temp[10];
        switch (ins->full_ins) {
        case '\n':
            strcpy(temp, "\\n");
            break;
        case '\t':
            strcpy(temp, "\\t");
            break;
        case '\\':
            strcpy(temp, "\\\\");
            break;
        default:
            if (ins->full_ins >= 32 && ins->full_ins < 127) {
                temp[0] = (char) ins->full_ins;
                temp[1] = '\0';
            } else {
                strcpy(temp, "?");
            }
        }

        strcat(global_str, temp);
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
        if ((ins->full_ins >> 8) != 1) {
            print_instruction_header(current_addr, is_directive(ins));
            print_binary(ins->full_ins);
        } else {
            return;
        }
    }

    // Instruction printing begins here
    if (ins->opcode != RET_OP && ins->opcode != HLT_OP) {
        print_operation(ins, op);
    }
    bool two_reg_args = ins->opcode == ADD_OP || ins->opcode == DIV_OP || ins->opcode == NAND_OP || ins->opcode == MOV_OP || ins->opcode == CMP_OP;

    if (two_reg_args) {
        print_two_reg_args(ins); // add, mov, div, etc.
        goto finish;
    }
    bool sign;
    int8_t val;
    switch (ins->opcode) {
    case BO_OP:
    case BZ_OP:
    case JMP_OP:
        print_jump_instruction(ins);
        break;
    case RET_OP:
        if ((ins->full_ins & 0xfff) == 0) {
            PRINTF("%sret%s", ANSI_BLUE, ANSI_RESET);
            len += 3;
        } else {
            if ((ins->destination >> 2) == 1) {
                PRINTF("%sbg %s", ANSI_BLUE, ANSI_RESET);
                len += 3;
            } else if ((ins->destination >> 2) == 0) {
                PRINTF("%sbl %s", ANSI_BLUE, ANSI_RESET);
                len += 3;
            }
            print_jump_instruction(ins);
        }
        break;
    case INT_OP:
        sign = (ins->source >> 7) == 1;

        val = (int8_t) ins->source & 0x7f;

        PRINTF(FMTSC, ANSI_VARIED, args.hex_operands ? (sign ? ins->source : val) : val, ANSI_RESET);
        snprintf(str, sizeof(str), FORMAT_STRING, args.hex_operands ? (sign ? ins->source : val) : val);
        break;
    case HLT_OP:
        print_hlt_instruction(ins);
        break;
    case LD_OP:
    case LEA_OP:
        PRINTF("%sr%d%s, ", ANSI_YELLOW, ins->destination & 0xF, ANSI_RESET);
        snprintf(str, sizeof(str), "r%d, ", ins->destination);

        PRINTF(FORMAT_STRING_MEM_COLORED, ANSI_VARIED, ins->full_ins & 511, ANSI_RESET);
        char tempstr[30];
        snprintf(tempstr, sizeof(tempstr), FORMAT_STRING_MEM, ins->source);
        len += strlen(tempstr);
        break;
    case ST_OP:
        if (ins->destination >> 2 == 1) {
            PRINTF("%s&r%d%s, %sr%d%s", ANSI_YELLOW, (ins->full_ins & 0x380) >> 7,
                   ANSI_RESET, ANSI_YELLOW, ins->source & 0xF, ANSI_RESET);
            snprintf(str, sizeof(str), "&r%d, r%d", (ins->full_ins & 0x380) >> 7, ins->source & 0x7);
        } else {
            ins->source &= 0x7;
            ins->destination = (ins->full_ins & 0xff8) >> 3;
            PRINTF(FORMAT_STRING_ST_COLORED, ANSI_VARIED, ins->destination, ANSI_RESET, ANSI_YELLOW,
                   ins->source, ANSI_RESET);
            snprintf(str, sizeof(str), FORMAT_STRING_ST, ins->destination, ins->source);
        }
        break;
    case PUSH_OP:
    case POP_OP:
        if ((ins->type == 0 && strcmp(op, "push") == 0)) {
            PRINTF("%sr%d%s", ANSI_YELLOW, ins->source & 0xF, ANSI_RESET);
            snprintf(str, sizeof(str), "r%d", ins->source & 0xF);
        } else {
            if (strcmp(op, "push") == 0) {
                PRINTF(FORMAT_STRING_COLORED, ANSI_VARIED, ins->full_ins & 2047, ANSI_RESET);
                snprintf(str, sizeof(str), FORMAT_STRING, ins->source);
            } else {
                if (ins->destination == 0x4) {
                    PRINTF(FORMAT_STRING_MEM_COLORED, ANSI_VARIED, ins->full_ins & 2047, ANSI_RESET);
                    snprintf(str, sizeof(str), FORMAT_STRING_MEM, ins->full_ins & 2047);
                } else {
                    PRINTF("%sr%d%s", ANSI_YELLOW, ins->source & 0xF, ANSI_RESET);
                    snprintf(str, sizeof(str), "r%d", ins->source & 0xF);
                }
            }
        } // push + pop
        break;
    }
    //
finish:
    if (strcmp(op, "bz") != 0 && strcmp(op, "bo") != 0 && strcmp(op, "jmp") != 0 && !two_reg_args) {
        len += strlen(str);
    }
    if (in_char && args.concat_chars) {
        if (!is_directive(ins)) {
            current_addr++;
        }
        return;
    }
    size_t spaces = 16 - len;
    if (!args.only_code) {
        for (size_t s = 0; s < spaces; s++) {
            PRINTF(" ");
        }
    }
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
                        PRINTF(" ");
                    PRINTF("%s◀%s", color, ANSI_RESET);
                    if (likely_label) {
                        PRINTF("%s [ LIKELY LABEL ]%s", POSSIBLE_ANSI_BOLD, ANSI_RESET);
                    }
#if defined(_WIN32)
                    PRINTF("%s from 0x%llX%s", color, jumpsHere->data[i].source, ANSI_RESET);
#elif defined(__APPLE__)
                    PRINTF("%s from 0x%llX%s", color, jumpsHere->data[i].source, ANSI_RESET);
#else
                    PRINTF("%s from 0x%lX%s", color, jumpsHere->data[i].source, ANSI_RESET);
#endif
                    if (i < jumpsHere->size - 1) {
                        PRINTF(", ");
                    }
                    has_jump = true;
                } else if (current_addr == jumpsHere->data[i].source) {
                    PRINTF("%s▶%s", color, ANSI_RESET);
#if defined(_WIN32)
                    PRINTF("%s to 0x%llX%s ", color, jumpsHere->data[i].destination, ANSI_RESET);
#elif defined(__APPLE__)
                    PRINTF("%s to 0x%llX%s ", color, jumpsHere->data[i].destination, ANSI_RESET);
#else
                    PRINTF("%s to 0x%lX%s ", color, jumpsHere->data[i].destination, ANSI_RESET);
#endif
                    has_outgoing_jump = true;
                } else if (current_addr == jumpsHere->data[i].destination && has_jump) {
#if defined(_WIN32)
                    PRINTF("%s0x%llX%s", color, jumpsHere->data[i].source, ANSI_RESET);
#elif defined(__APPLE__)
                    PRINTF("%s0x%llX%s", color, jumpsHere->data[i].source, ANSI_RESET);
#else
                    PRINTF("%s0x%lX%s", color, jumpsHere->data[i].source, ANSI_RESET);
#endif
                    if (i < jumpsHere->size - 1) {
                        PRINTF(", ");
                    } else {
                        PRINTF(" ");
                    }
                }
            }
        }
        current_addr++;
    }
    PRINTF("\n");
}
