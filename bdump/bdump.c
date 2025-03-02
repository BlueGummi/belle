/*
 * Copyright (c) 2024 BlueGummi
 * All rights reserved.
 *
 * This code is licensed under the BSD 3-Clause License.
 */
#include "cli.c"
#include "colors.c"
#include "structures.c"
#include "io.c"
#include "print_helpers.c"
#include "instruction_print.c"
void *process_instructions(void *arg, char *filename) {
    char metadata[1024] = "\0";
#ifdef _WIN32
    SetConsoleOutputCP(CP_UTF8);
    setvbuf(stdout, NULL, _IOFBF, 1024);
#endif
    ThreadData *data = (ThreadData *) arg;
    if (4 >= data->bytes_read) {
        printf("%sBinary appears invalid.%s\n", ANSI_RED, ANSI_RESET);
        exit(1);
    }
    bin_version = data->buffer[1];
    current_addr = (data->buffer[2] << 8) | (data->buffer[3]);
    jump_map_global = jump_map_create();
    size_t start_ind = 4;
    for (size_t i = 4; i < data->bytes_read; i += 2) {
        if (i + 1 < data->bytes_read) {
            uint16_t instruction = (data->buffer[i] << 8) | data->buffer[i + 1];
            if (i > 0 && instruction >> 8 == 1) {
                char inschar = (char) instruction & 0xFF;
                strncat(metadata, &inschar, 1);
            } else {
                start_ind = i;
            }
        }
    }
    int counter = 0;
    size_t current_addr_tmp = current_addr;
    for (size_t i = start_ind; i < data->bytes_read; i += 2) { // second loop finds jumps
        if (i + 1 < data->bytes_read) {
            uint16_t instruction = (data->buffer[i] << 8) | data->buffer[i + 1];
            switch (instruction >> 12) {
            case JMP_OP:
            case BO_OP:
            case BZ_OP:
            case RET_OP:
                if (instruction >> 12 == RET_OP && (instruction & 0xfff) == 0)
                    break;
                if (((instruction >> 11) & 1) == 1)
                    break;
                Jump jump_data;
                jump_data.source = current_addr;
                jump_data.destination = instruction & 0x7ff;
                jump_data.reverse = jump_data.destination < jump_data.source;
                jump_data.color = get_color(counter++);
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

    print_header(metadata, filename);
    current_addr = current_addr_tmp;
    for (size_t i = 4; i < data->bytes_read; i += 2) { // start at 2 to ignore version
        if (i + 1 < data->bytes_read) {                // third loop adjusts columns and prints
            uint16_t instruction = (data->buffer[i] << 8) | data->buffer[i + 1];
            if (i > 0 && instruction >> 8 == 1) {
                continue; // metadata
            }
            uint16_t next_instruction = (data->buffer[i + 2] << 8) | data->buffer[i + 3];
            Instruction ins = parse_instruction(instruction);
            Instruction ins2 = parse_instruction(next_instruction);
            JumpVector *jumpsHere = find_jumps_at_address(jump_map_global, current_addr);
            len = 0;
            print_instruction(&ins, &ins2, jumpsHere);
            free_jump_vector(jumpsHere);
        }
    }
    print_footer();
    free_map(jump_map_global);
    // jump_map_print(jump_map_global);
    return NULL;
}
int main(int argc, char *argv[]) {
    is_term = is_terminal();
    args = parse_arguments(argc, argv);
    if (args.num_files == 0) {
        print_help(argv[0]);
        return EXIT_FAILURE;
    }

    ThreadData thread_data[THREAD_COUNT];

    for (uint8_t i = 0; i < args.num_files; i++) {
        FILE *input = fopen(args.input_files[i], "rb");
        if (!input) {
            fputs(ANSI_RED_CONST ANSI_BOLD_CONST "Failed to open file \033[0m", stderr);
            perror(args.input_files[i]);
            return EXIT_FAILURE;
        }

        size_t bytes_read = fread(thread_data[0].buffer, sizeof(uint8_t), BUFFER_SIZE, input);
        if (bytes_read > 0) {
            thread_data[0].bytes_read = bytes_read;
            thread_data[0].input = input;
            process_instructions(&thread_data[0], args.input_files[i]);
        }

        fclose(input);
    }

    return EXIT_SUCCESS;
}
char *match_opcode(Instruction *s) {
    switch (s->opcode) {
    case HLT_OP:
        return "hlt";

    case ADD_OP:
        return "add";

    case BO_OP:
        return "bo";

    case POP_OP:
        return "pop";

    case DIV_OP:
        return "div";

    case RET_OP:
        return "ret";

    case LD_OP:
        return "ld";

    case ST_OP:
        return "st";

    case JMP_OP:
        return "jmp";

    case BZ_OP:
        return "bz";

    case CMP_OP:
        return "cmp";

    case NAND_OP:
        return "nand";

    case PUSH_OP:
        return "push";

    case INT_OP:
        return "int";

    case MOV_OP:
        return "mov";

    case LEA_OP:
        return "lea";

    default:
        puts("OPCODE not recognized.");
        exit(1);
    }
}

Instruction parse_instruction(uint32_t instruction) {
    Instruction parsed_ins;
    parsed_ins.opcode = instruction >> 12;
    parsed_ins.destination = (instruction >> 9) & 0x7;
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
    parsed_ins.full_ins = (int16_t) instruction;
    return parsed_ins;
}
