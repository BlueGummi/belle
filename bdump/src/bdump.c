/*
 * Copyright (c) 2024 BlueGummi
 * All rights reserved.
 *
 * This code is licensed under the BSD 3-Clause License.
 */

#include "print_helpers.c"

void *process_instructions(void *arg) {
    ThreadData *data = (ThreadData *)arg;
    for (size_t i = 0; i < data->bytes_read; i += 2) {
        if (i + 1 < data->bytes_read) {
            uint16_t    instruction = (data->buffer[i] << 8) | data->buffer[i + 1];
            Instruction ins         = parse_instruction(instruction);
            if ((ins.full_ins >> 9) == 1) {
                current_addr = ins.full_ins & 0b111111111;
            }
        }
    }
    for (size_t i = 0; i < data->bytes_read; i += 2) {
        if (i + 1 < data->bytes_read) {
            uint16_t    instruction = (data->buffer[i] << 8) | data->buffer[i + 1];
            Instruction ins         = parse_instruction(instruction);
            print_instruction(&ins);
        }
    }
    return NULL;
}

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

    ThreadData thread_data[THREAD_COUNT];
    size_t     bytes_read;

#ifdef _WIN32
    HANDLE thread_handles[THREAD_COUNT];
#else
    pthread_t thread_handles[THREAD_COUNT];
#endif

    while ((bytes_read = fread(thread_data[0].buffer, sizeof(uint8_t), BUFFER_SIZE, input)) > 0) {
        thread_data[0].bytes_read = bytes_read;
        thread_data[0].input      = input;

#ifdef _WIN32
        thread_handles[0] = CreateThread(NULL, 0, (LPTHREAD_START_ROUTINE)process_instructions,
                                         &thread_data[0], 0, NULL);
        if (thread_handles[0] == NULL) {
            fputs(ANSI_RED "Failed to create thread\n" ANSI_RESET, stderr);
            fclose(input);
            return EXIT_FAILURE;
        }

        WaitForSingleObject(thread_handles[0], INFINITE);
#else
        if (pthread_create(&thread_handles[0], NULL, process_instructions, &thread_data[0]) != 0) {
            fputs(ANSI_RED "Failed to create thread\n" ANSI_RESET, stderr);
            fclose(input);
            return EXIT_FAILURE;
        }

        pthread_join(thread_handles[0], NULL);
#endif
    }

    fclose(input);
    return EXIT_SUCCESS;
}
char *match_opcode(Instruction *s) {
    char *opcode;
    switch (s->opcode) {
    case HLT_OP:
        opcode = "hlt";
        break;
    case ADD_OP:
        opcode = "add";
        break;
    case JO_OP:
        opcode = "jo";
        break;
    case POP_OP:
        opcode = "pop";
        break;
    case DIV_OP:
        opcode = "div";
        break;
    case RET_OP:
        opcode = "ret";
        break;
    case LD_OP:
        opcode = "ld";
        break;
    case ST_OP:
        opcode = "st";
        break;
    case JMP_OP:
        opcode = "jmp";
        break;
    case JZ_OP:
        opcode = "jz";
        break;
    case CMP_OP:
        opcode = "cmp";
        break;
    case MUL_OP:
        opcode = "mul";
        break;
    case PUSH_OP:
        opcode = "push";
        break;
    case INT_OP:
        opcode = "int";
        break;
    case MOV_OP:
        opcode = "mov";
        break;
    case LEA_OP:
        opcode = "lea";
        break;
    default:
        puts("OPCODE not recognized.");
        exit(1);
    }
    return opcode;
}

void print_instruction(Instruction *s) {
    print_output(s);
}

Instruction parse_instruction(int instruction) {
    Instruction parsed_ins;
    parsed_ins.opcode      = instruction >> 12;
    parsed_ins.destination = (instruction >> 9) & 0b111;
    parsed_ins.source      = instruction & 0xFF;
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
    return parsed_ins;
}

CLI parse_arguments(int argc, char *argv[]) {
    CLI opts        = {0};
    opts.input_file = NULL;

    for (int i = 1; i < argc; i++) {
        if (strcmp(argv[i], "--help") == 0 || strcmp(argv[i], "-h") == 0) {
            print_help(argv[0]);
            exit(EXIT_SUCCESS);
        } else if (argv[i][0] == '-') {
            if (argv[i][1] == '-') {
                if (strcmp(argv[i], "--address") == 0) {
                    opts.address = 1;
                } else if (strcmp(argv[i], "--colors") == 0) {
                    opts.colors = 1;
                } else if (strcmp(argv[i], "--verbose") == 0) {
                    opts.verbosity++;
                } else if (strcmp(argv[i], "--binary") == 0) {
                    opts.binary = 1;
                } else {
                    fputs("Error: Unknown option ", stderr);
                    fputs(argv[i], stderr);
                    fputc('\n', stderr);
                    print_help(argv[0]);
                    exit(EXIT_FAILURE);
                }
            } else {
                for (int j = 1; argv[i][j] != '\0'; j++) {
                    switch (argv[i][j]) {
                    case 'a':
                        opts.address = 1;
                        break;
                    case 'c':
                        opts.colors = 1;
                        break;
                    case 'v':
                        opts.verbosity++;
                        break;
                    case 'b':
                        opts.binary = 1;
                        break;
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
