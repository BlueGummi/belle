#include "bdump.h"
size_t current_addr = 100;
CLI    args         = {0};

void print_operation(Instruction *ins, char *op, bool colors) {
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
        ins->source &= 0b01111111; // Clear the sign bit

        if (colors) {
            printf("%s%d%s", ANSI_YELLOW, sign ? -ins->source : ins->source, ANSI_RESET);
        } else {
            printf("%d", sign ? -ins->source : ins->source);
        }
    } break;

    case 2: // memory address indirect
    {
        int memaddr = ((ins->source << 1) & 0b1111111) >> 1;

        if (colors) {
            printf("%s&%d%s", ANSI_YELLOW, memaddr, ANSI_RESET);
        } else {
            printf("&%d", memaddr);
        }
    } break;

    case 3: // register indirect
    {
        int reg = ((ins->source << 3) & 0b1111111) >> 3;

        if (colors) {
            printf("%s&r%d%s", ANSI_YELLOW, reg, ANSI_RESET);
        } else {
            printf("&r%d", reg);
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
        printf("[%s%d%s]", ANSI_GREEN, ins->full_ins & 1023, ANSI_RESET);
    } else {
        printf("[%d]", ins->full_ins & 1023);
    }
}

void print_hlt_instruction(Instruction *ins, bool colors) {
    if (ins->destination == 1) {
        if (colors) {
            printf("%s.start%s [%s%d%s]", ANSI_BLUE, ANSI_RESET, ANSI_GREEN, ins->full_ins & 0b111111111, ANSI_RESET);
        } else {
            printf(".start [%d]", ins->full_ins & 0b111111111);
        }
    } else if (ins->destination == 2) {
        if (colors) {
            printf("%s.ssp%s [%s%d%s]", ANSI_BLUE, ANSI_RESET, ANSI_GREEN, ins->full_ins & 0b111111111, ANSI_RESET);
        } else {
            printf(".ssp [%d]", ins->full_ins & 0b111111111);
        }
    } else if (ins->destination == 3) {
        if (colors) {
            printf("%s.sbp%s [%s%d%s]", ANSI_BLUE, ANSI_RESET, ANSI_GREEN, ins->full_ins & 0b111111111, ANSI_RESET);
        } else {
            printf(".sbp [%d]", ins->full_ins & 0b111111111);
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
                printf("%s%s%s (%s%d%s)", ANSI_BLUE,
                       (ins->full_ins == '\n'                          ? "\\n"
                        : ins->full_ins == '\t'                        ? "\\t"
                        : ins->full_ins == '\\'                        ? "\\\\"
                        : (ins->full_ins >= 32 && ins->full_ins < 127) ? (char[]){(char)ins->full_ins, '\0'}
                                                                       : "?"),
                       ANSI_RESET, ANSI_YELLOW, ins->full_ins, ANSI_RESET);
            } else {
                printf("%s (%d)",
                       (ins->full_ins == '\n'                          ? "\\n"
                        : ins->full_ins == '\t'                        ? "\\t"
                        : ins->full_ins == '\\'                        ? "\\\\"
                        : (ins->full_ins >= 32 && ins->full_ins < 127) ? (char[]){(char)ins->full_ins, '\0'}
                                                                       : "?"),
                       ins->full_ins);
            }
        } else {
            if (colors) {
                printf("%s.word%s %s%d%s", ANSI_BLUE, ANSI_RESET, ANSI_GREEN, ins->full_ins, ANSI_RESET);
            } else {
                printf(".word %d", ins->full_ins);
            }
        }
    }
}

bool is_directive(Instruction *ins) {
    int part = ins->full_ins >> 9;
    return (part == 1) || (part == 2) || (part == 3);
}

void print_instruction(Instruction *ins) {
    bool  colors = args.colors == 1;
    char *op     = match_opcode(ins);

    if (args.only_code != 1) {
        if (!is_directive(ins)) {
            print_instruction_header(current_addr, colors, false);
        } else {
            print_instruction_header(current_addr, colors, true);
        }
        print_binary(ins->full_ins);
    }

    if (!is_directive(ins)) {
        current_addr++;
    }
    if (strcmp(op, "ret") != 0 && strcmp(op, "hlt") != 0) {
        print_operation(ins, op, colors);
    }
    bool two_reg_args = (strcmp(op, "add") == 0 || strcmp(op, "div") == 0 || strcmp(op, "cmp") == 0 ||
                         strcmp(op, "mul") == 0 || strcmp(op, "mov") == 0);

    if (two_reg_args) {
        print_two_reg_args(ins, colors);
    } else if (strcmp(op, "jz") == 0 || strcmp(op, "jo") == 0 || strcmp(op, "jmp") == 0) {
        print_jump_instruction(ins, colors);
    } else if (strcmp(op, "ret") == 0) {
        if ((ins->full_ins & 0b111111111111) == 0) {
            if (colors)
                printf("%sret%s", ANSI_BLUE, ANSI_RESET);
            else
                printf("ret");
        } else {
            if ((ins->destination >> 2) == 1) {
                if (colors)
                    printf("%sjg %s", ANSI_BLUE, ANSI_RESET);
                else
                    printf("jg ");
            } else if ((ins->destination >> 2) == 0) {
                if (colors)
                    printf("%sjl %s", ANSI_BLUE, ANSI_RESET);
                else
                    printf("jl ");
            }
            print_jump_instruction(ins, colors);
        }
    } else if (strcmp(op, "int") == 0) {
        if (colors) {
            printf("%s%d%s", ANSI_GREEN, ins->source, ANSI_RESET);
        } else {
            printf("%d", ins->source);
        }
    } else if (strcmp(op, "hlt") == 0) {
        print_hlt_instruction(ins, colors);
    } else if (strcmp(op, "ld") == 0 || strcmp(op, "lea") == 0) {
        if (colors) {
            printf("%sr%d%s, ", ANSI_GREEN, ins->destination, ANSI_RESET);
        } else {
            printf("r%d, ", ins->destination);
        }

        ins->type = (ins->type << 8) | ins->source;

        if (colors) {
            printf("[%s%d%s]", ANSI_YELLOW, ins->source, ANSI_RESET);
        } else {
            printf("[%d]", ins->source);
        }
    } else if (strcmp(op, "st") == 0) {
        if (ins->destination >> 2 == 1) {
            if (colors) {
                printf("%s&r%d%s, %sr%d%s", ANSI_GREEN, (ins->full_ins & 0b1110000000) >> 7,
                       ANSI_RESET, ANSI_YELLOW, ins->source & 7, ANSI_RESET);
            } else {
                printf("&r%d, r%d", (ins->full_ins & 0b1110000000) >> 7,
                       (ins->source & 0b111));
            }
        } else {

            ins->source &= 0b111;
            ins->destination = (ins->full_ins & 0b111111111000) >> 3;
            if (colors) {
                printf("[%s%d%s], %sr%d%s", ANSI_GREEN, ins->destination, ANSI_RESET, ANSI_YELLOW,
                       ins->source, ANSI_RESET);
            } else {
                printf("[%d], r%d", ins->destination, ins->source);
            }
        }
    } else if (strcmp(op, "push") == 0 || strcmp(op, "pop") == 0) {
        if ((ins->type == 0 && strcmp(op, "push") == 0)) {
            if (colors) {
                printf("%sr%d%s", ANSI_GREEN, ins->full_ins & 2047, ANSI_RESET);
            } else {
                printf("r%d", ins->source);
            }
        } else {
            if (strcmp(op, "push") == 0) {
                if (colors) {
                    printf("%s%d%s", ANSI_GREEN, ins->full_ins & 2047, ANSI_RESET);
                } else {
                    printf("%d", ins->source);
                }
            } else {
                if (ins->destination == 0b100) {
                    if (colors) {
                        printf("[%s%d%s]", ANSI_GREEN, ins->full_ins & 2047, ANSI_RESET);
                    } else {
                        printf("[%d]", ins->full_ins & 2047);
                    }
                } else {
                    if (colors) {
                        printf("%sr%d%s", ANSI_GREEN, ins->full_ins & 2047, ANSI_RESET);
                    } else {
                        printf("r%d", ins->full_ins & 2047);
                    }
                }
            }
        }
    }

    if (args.verbosity == 1 && args.only_code == 1) {
	if ((ins->full_ins >> 9) != 1 && (ins->full_ins >> 9) != 2 && (ins->full_ins >> 9) != 3) { 
        	printf(" ; address %ld", current_addr);
	} else {
		printf(" ; no address");
	}
    }
    printf("\n");
}
