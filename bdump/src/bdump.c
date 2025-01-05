/*
 * Copyright (c) 2024 BlueGummi
 * All rights reserved.
 *
 * This code is licensed under the BSD 3-Clause License.
 */
#include "bdump.h"

int main(int argc, char *argv[]) {
    args = parse_arguments(argc, argv);
    if (args.input_file == NULL) {
        print_help(argv[0]);
        return EXIT_FAILURE;
    }

    FILE *input = fopen(args.input_file, "rb");
    if (!input) {
        fputs(ANSI_RED ANSI_BOLD "Failed to open file: " ANSI_RESET, stderr);
        fputs(args.input_file, stderr);
        fputc('\n', stderr);
        return EXIT_FAILURE;
    }

    uint8_t buffer[2]; 
    while (fread(buffer, sizeof(uint8_t), 2, input) == 2) {
        uint16_t instruction = (buffer[0] << 8) | buffer[1];
        Instruction ins = parse_instruction(instruction);
        print_instruction(&ins);
    }

    fclose(input);
    return EXIT_SUCCESS;
}

char *match_opcode(Instruction *s) {
    char *opcode;
    switch (s->opcode) {
    case HLT_OP: opcode = "hlt"; break;
    case ADD_OP: opcode = "add"; break;
    case JO_OP: opcode = "jo"; break;
    case POP_OP: opcode = "pop"; break;
    case DIV_OP: opcode = "div"; break;
    case RET_OP: opcode = "ret"; break;
    case LD_OP: opcode = "ld"; break;
    case ST_OP: opcode = "st"; break;
    case JMP_OP: opcode = "jmp"; break;
    case JZ_OP: opcode = "jz"; break;
    case CMP_OP: opcode = "cmp"; break;
    case MUL_OP: opcode = "mul"; break;
    case PUSH_OP: opcode = "push"; break;
    case INT_OP: opcode = "int"; break;
    case MOV_OP: opcode = "mov"; break;
    case 0b1111: opcode = "nop"; break;
    default: puts("OPCODE not recognized."); exit(1);
    }
    return opcode;
}

void print_instruction(Instruction *s) {
    print_output(s);
    if (args.debug == 1) {
        print("type %d\n", s->type);
    }
    if (args.debug == 1)
        putchar('\n');
}

Instruction parse_instruction(int instruction) {
    Instruction parsed_ins;
    parsed_ins.opcode = instruction >> 12;
    parsed_ins.destination = (instruction >> 9) & 0b111;
    parsed_ins.source = instruction & 0xFF;
    if (((instruction >> 8) & 1) == 1) {
        parsed_ins.type = 1;
    } else {
        parsed_ins.type = 0;
        if (((instruction >> 7) & 1) == 1)
            parsed_ins.type = 2;
        else if (((instruction >> 6) & 1) == 1)
            parsed_ins.type = 3;
    }
    parsed_ins.full_ins = instruction;
    print_binary(instruction, 16);
    return parsed_ins;
}

CLI parse_arguments(int argc, char *argv[]) {
    CLI opts = {0};
    opts.input_file = NULL;

    for (int i = 1; i < argc; i++) {
        if (strcmp(argv[i], "--help") == 0 || strcmp(argv[i], "-h") == 0) {
            print_help(argv[0]);
            exit(EXIT_SUCCESS);
        } else if (argv[i][0] == '-') {
            if (argv[i][1] == '-') { // Handle long options
                if (strcmp(argv[i], "--line-num") == 0) {
                    opts.line_num = 1;
                } else if (strcmp(argv[i], "--colors") == 0) {
                    opts.colors = 1;
                } else if (strcmp(argv[i], "--verbose") == 0) {
                    opts.verbosity++;
                } else if (strcmp(argv[i], "--debug") == 0) {
                    opts.debug = 1;
                } else if (strcmp(argv[i], "--binary") == 0) {
                    opts.binary = 1;
                } else {
                    fputs("Error: Unknown option ", stderr);
                    fputs(argv[i], stderr);
                    fputc('\n', stderr);
                    print_help(argv[0]);
                    exit(EXIT_FAILURE);
                }
            } else { // Handle short options
                for (int j = 1; argv[i][j] != '\0'; j++) {
                    switch (argv[i][j]) {
                    case 'l': opts.line_num = 1; break;
                    case 'c': opts.colors = 1; break;
                    case 'v': opts.verbosity++; break;
                    case 'd': opts.debug = 1; break;
                    case 'b': opts.binary = 1; break;
                    default:
                        fputs("Error: Unknown option -", stderr);
                        fputc(argv[i][j], stderr);
                        fputc('\n', stderr);
                        print_help(argv[0]);
                        exit(EXIT_FAILURE);
                    }
                }
            }
        } else {
            if (opts.input_file == NULL) {
                opts.input_file = argv[i];
            } else {
                fputs("Error: Unexpected argument: ", stderr);
                fputs(argv[i], stderr);
                fputc('\n', stderr);
                print_help(argv[0]);
                exit(EXIT_FAILURE);
            }
        }
    }
    return opts;
}
