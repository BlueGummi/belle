/*
 * Copyright (c) 2024 BlueGummi
 * All rights reserved.
 *
 * This code is licensed under the BSD 3-Clause License.
 */

#include "cli.c"

void *process_instructions(void *arg) {
#ifdef _WIN32
    SetConsoleOutputCP(CP_UTF8);
#endif
    ThreadData *data = (ThreadData *)arg;
    jump_map_global  = jump_map_create();
    for (size_t i = 0; i < data->bytes_read; i += 2) {
        if (i + 1 < data->bytes_read) {
            uint16_t instruction = (data->buffer[i] << 8) | data->buffer[i + 1];
            if ((instruction >> 9) == 1) {
                current_addr = instruction & 0b111111111;
                break;
            } // First loop finds starting address
        }
    }
    size_t current_addr_tmp = current_addr;
    int    column           = 1;
    for (size_t i = 0; i < data->bytes_read; i += 2) { // second loop finds jumps
        if (i + 1 < data->bytes_read) {
            uint16_t instruction = (data->buffer[i] << 8) | data->buffer[i + 1];
            switch (instruction >> 12) {
            case JMP_OP:
            case JO_OP:
            case JZ_OP:
            case RET_OP:
                if (instruction >> 12 == RET_OP && (instruction & 0b111111111111) == 0)
                    break;
                Jump jump_data;
                jump_data.id          = column;
                jump_data.source      = current_addr;
                jump_data.destination = instruction & 0b11111111111;
                jump_data.column      = column++;
                jump_data.reverse     = jump_data.destination < jump_data.source;
                jump_data.color       = get_color(jump_data.column);
                jump_map_insert(jump_map_global, current_addr, jump_data);
                break;
            default:
                break;
            }
            Instruction ins = parse_instruction(instruction);
            if (!is_directive(&ins)) {
                current_addr++;
            }
        }
    }

    print_header();
    current_addr = current_addr_tmp;
    for (size_t i = 0; i < data->bytes_read; i += 2) {
        if (i + 1 < data->bytes_read) { // third loop adjusts columns and prints
            uint16_t    instruction = (data->buffer[i] << 8) | data->buffer[i + 1];
            Instruction ins         = parse_instruction(instruction);
            JumpVector *jumpsHere   = find_jumps_at_address(jump_map_global, current_addr);
            adjust_jump_vector(jumpsHere);
            len = 0;
            print_instruction(&ins, jumpsHere);
        }
    }

    if (args.only_code != 1) {
        if (args.binary != 1) {
            if (args.print_hex == 1) {
                printf("╰─────────────┴───────╯\n");
            } else {
                printf("╰────────┴───────╯\n");
            }
        } else {
            if (args.print_hex == 1) {
                printf("╰─────────────┴──────────────────────────╯\n");
            } else {
                printf("╰────────┴──────────────────────────╯\n");
            }
        }
    }
    // jump_map_print(jump_map_global);
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
    // macro programming multithreading tomfoolery - "It works" and "It's not broken, don't fix it"
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

Instruction parse_instruction(uint32_t instruction) {
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
