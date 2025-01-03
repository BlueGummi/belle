#include "bdump.h"
#include "consts.h"
#pragma once

void print_operation(const char *op, int destination, bool colors) {
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
            printf("%sr%d%s\n", ANSI_YELLOW, ins->source, ANSI_RESET);
        } else {
            printf("r%d\n", ins->source);
        }
        break;

    case 1: // literal
    {
        bool sign = (ins->source >> 7) == 1;
        ins->source &= 0b01111111; // Clear the sign bit

        if (colors) {
            printf("%s%d%s\n", ANSI_YELLOW, sign ? -ins->source : ins->source, ANSI_RESET);
        } else {
            printf("%d\n", sign ? -ins->source : ins->source);
        }
    } break;

    case 2: // memory address
    {
        int memaddr = ((ins->source << 1) & 0b1111111) >> 1;

        if (colors) {
            printf("%s&%d%s\n", ANSI_YELLOW, memaddr, ANSI_RESET);
        } else {
            printf("&%d\n", memaddr);
        }
    } break;

    case 3: // register indirect
    {
        int reg = ((ins->source << 3) & 0b1111111) >> 3;

        if (colors) {
            printf("%s&r%d%s\n", ANSI_YELLOW, reg, ANSI_RESET);
        } else {
            printf("&r%d\n", reg);
        }
    } break;

    default: fprintf(stderr, "Unknown instruction type\n"); exit(1);
    }
}

void print_jump_instruction(Instruction *ins, bool colors) {
    if (ins->destination >> 2 == 1) {
        if (colors) {
            printf("%s&r%d%s\n", ANSI_YELLOW, ins->source, ANSI_RESET);
        } else {
            printf("&r%d\n", ins->source);
        }
        return;
    }

    ins->source = (ins->destination << 8) | ins->source;

    if (colors) {
        printf("%s$%d%s\n", ANSI_YELLOW, ins->source, ANSI_RESET);
    } else {
        printf("$%d\n", ins->source);
    }
}

void print_hlt_instruction(Instruction *ins, bool colors) {
    if (ins->destination == 1) {
        if (colors) {
            printf("%s.start%s%s $%d%s\n", ANSI_GREEN, ANSI_RESET, ANSI_YELLOW, ins->source,
                   ANSI_RESET);
        } else {
            printf(".start $%d\n", ins->source);
        }
    } else if (ins->destination == 2) {
        if (colors) {
            printf("%s.ssp%s%s $%d%s\n", ANSI_GREEN, ANSI_RESET, ANSI_YELLOW, ins->source,
                   ANSI_RESET);
        } else {
            printf(".ssp $%d\n", ins->source);
        }
    } else if (ins->destination == 3) {
        if (colors) {
            printf("%s.sbp%s%s $%d%s\n", ANSI_GREEN, ANSI_RESET, ANSI_YELLOW, ins->source,
                   ANSI_RESET);
        } else {
            printf(".sbp $%d\n", ins->source);
        }
    } else if (ins->full_ins == 0) {
        if (colors) {
            printf("%shlt%s\n", ANSI_YELLOW, ANSI_RESET);
        } else {
            printf("hlt\n");
        }
    } else {
        if (colors) {
            printf("%s%c%s (%s%d%s)\n", ANSI_BLUE, (char) ins->full_ins, ANSI_RESET, ANSI_YELLOW,
                   ins->full_ins, ANSI_RESET);
        } else {
            printf("%c (%d)\n", (char) ins->full_ins, ins->full_ins);
        }
    }
}

void print_output(Instruction *ins) {
    bool colors = args.colors == 1;
    char *op = match_opcode(ins);

    if (args.line_num == 1) {
        print_instruction_header(line, colors);
    }

    if (strcmp(op, "sr") != 0 && strcmp(op, "hlt") != 0) {
        print_operation(op, ins->destination, colors);
    }

    if (strcmp(op, "nop") == 0) {
        printf("\n");
        return;
    }

    bool two_reg_args =
        (strcmp(op, "add") == 0 || strcmp(op, "div") == 0 || strcmp(op, "cmp") == 0 ||
         strcmp(op, "mul") == 0 || strcmp(op, "mov") == 0);

    if (two_reg_args) {
        print_two_reg_args(ins, colors);
    } else if (strcmp(op, "sr") == 0) {
        if (colors) {
            printf("%sasdfnop%s\n", ANSI_GREEN, ANSI_RESET);
        } else {
            printf("%snop\n", op);
        }
    } else if (strcmp(op, "jz") == 0 || strcmp(op, "jo") == 0 || strcmp(op, "jmp") == 0) {
        print_jump_instruction(ins, colors);
    } else if (strcmp(op, "ret") == 0) {
        printf("\n");
    } else if (strcmp(op, "int") == 0) {
        if (colors) {
            printf("%s%d%s\n", ANSI_YELLOW, ins->source, ANSI_RESET);
        } else {
            printf("%d\n", ins->source);
        }
    } else if (strcmp(op, "hlt") == 0) {
        print_hlt_instruction(ins, colors);
    } else if (strcmp(op, "ld") == 0) {
        if (colors) {
            printf("%sr%d%s, ", ANSI_YELLOW, ins->destination, ANSI_RESET);
        } else {
            printf("r%d, ", ins->destination);
        }

        ins->type = (ins->type << 8) | ins->source;

        if (colors) {
            printf("%s$%d%s\n", ANSI_YELLOW, ins->source, ANSI_RESET);
        } else {
            printf("$%d\n", ins->source);
        }
    } else if (strcmp(op, "st") == 0) {
        if (ins->destination >> 2 == 1) {
            if (colors) {
                printf("%s&r%d%s, %sr%d%s\n", ANSI_YELLOW,
                       ins->type << 1 | (ins->source & 0b10000000) >> 7, ANSI_RESET, ANSI_YELLOW,
                       ins->source & 7, ANSI_RESET);
            } else {
                printf("&r%d, r%d\n", ins->type << 1 | (ins->source & 0b10000000) >> 7,
                       (ins->source & 0b111));
            }
            return;
        }

        int reconstructed = (ins->destination << 9) | (ins->type << 8) | ins->source;
        ins->source &= 0x07;
        ins->destination = (reconstructed & 0xFFF8) >> 3;

        if (colors) {
            printf("%s$%d%s, %sr%d%s\n", ANSI_YELLOW, ins->destination, ANSI_RESET, ANSI_YELLOW,
                   ins->source, ANSI_RESET);
        } else {
            printf("$%d, r%d\n", ins->destination, ins->source);
        }
    } else if (strcmp(op, "push") == 0 || strcmp(op, "pop") == 0) {
        if ((ins->type == 0 && strcmp(op, "push") == 0) || (strcmp(op, "pop") == 0 && (ins->full_ins & 2048) == 1)) {
            if (colors) {
                printf("%sr%d%s\n", ANSI_YELLOW, ins->full_ins & 2047, ANSI_RESET);
            } else {
                printf("r%d\n", ins->source);
            }
        } else {
            if (strcmp(op, "push") == 0) {
                if (colors) {
                    printf("%s%d%s\n", ANSI_YELLOW, ins->full_ins & 2047, ANSI_RESET);
                } else {
                    printf("%d\n", ins->source);
                }
            } else {
                if (colors) {
                    printf("%s$%d%s\n", ANSI_YELLOW, ins->full_ins & 2047, ANSI_RESET);
                } else {

                    printf("$%d\n", ins->full_ins & 2047);
                }
            }
        }
    }

    line++;
}
