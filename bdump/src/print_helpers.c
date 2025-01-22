#include "print_utils.c"
void print_binary(int16_t num) {

    char hex[5];
    hex[4] = '\0';
    int numclone = num;
    for (int i = 0; i < 4; i++) {
        hex[3 - i] = "0123456789ABCDEF"[num & 0xF];
        num >>= 4;
    }

    for (int i = 0; i < 4; i += 2) {
        if (args.colors) {
            printf("%s%c%c%s", ANSI_CYAN, hex[i], hex[i + 1], ANSI_RESET);
        } else {
            printf("%c%c", hex[i], hex[i + 1]);
        }
        if (i != 2) {
            printf(" ");
        }
    }
    if (args.binary == 1) {

        if (args.colors) {
            printf(" %s0b%s", ANSI_MAGENTA, ANSI_RESET);
        } else {
            printf(" 0b");
        }

        for (int i = 15; i >= 0; i--) {
            if (args.colors) {
                printf("%s%d%s", ANSI_MAGENTA, (numclone >> i) & 1, ANSI_RESET);
            } else {
                printf("%d", (numclone >> i) & 1);
            }
        }
    }
    printf(" │ ");
}

void print_help(char *bin) { // bin is the name of the bin program
    printf("The disassembler for the BELLE-ISA\n\n");
    printf("%s%sUsage:%s %s%s%s [OPTIONS] <BINARY>\n\n", ANSI_UNDERLINE, ANSI_BOLD, ANSI_RESET,
           ANSI_BOLD, bin, ANSI_RESET);
    printf("%s%sArguments:%s\n", ANSI_BOLD, ANSI_UNDERLINE, ANSI_RESET);
    printf("  <BINARY>  Path to binary\n\n");
    printf("%s%sOptions:%s\n", ANSI_BOLD, ANSI_UNDERLINE, ANSI_RESET);
    printf("  %s-h%s, %s--help%s       Show this help message and exit\n", ANSI_BOLD, ANSI_RESET,
           ANSI_BOLD, ANSI_RESET);
    printf("  %s-b%s, %s--binary%s     Print instruction binary\n", ANSI_BOLD, ANSI_RESET, ANSI_BOLD, ANSI_RESET);
    printf("  %s-c%s, %s--colorless%s  Disable colored output\n", ANSI_BOLD, ANSI_RESET, ANSI_BOLD,
           ANSI_RESET);
    printf("  %s-o%s, %s--only-code%s  Print only disassembled code\n", ANSI_BOLD, ANSI_RESET, ANSI_BOLD,
           ANSI_RESET);
    printf("  %s-x%s, %s--hex-mem%s    Print memory addresses in hexadecimal\n", ANSI_BOLD, ANSI_RESET, ANSI_BOLD,
           ANSI_RESET);
    printf("  %s-X%s, %s--hex%s        Print instruction operands in hexadecimal\n", ANSI_BOLD, ANSI_RESET, ANSI_BOLD,
           ANSI_RESET);
    printf("  %s-v%s, %s--verbose%s    Increase verbosity level (use multiple for more)\n", ANSI_BOLD,
           ANSI_RESET, ANSI_BOLD, ANSI_RESET);
    printf("  %s-V%s, %s--version%s    Print version\n", ANSI_BOLD,
           ANSI_RESET, ANSI_BOLD, ANSI_RESET);
    exit(0);
}

#define PRINT_COLOR_AND_VALUE(color, format, value) \
    do {                                            \
        if (colors) {                               \
            printf("%s", color);                    \
        }                                           \
        printf(format, value);                      \
        if (colors) {                               \
            printf(ANSI_RESET);                     \
        }                                           \
    } while (0)

void print_instruction_header(size_t line, bool colors, bool is_directive) {
    size_t lineclone = line;

    printf("│ ");
    if (args.print_hex == 1) {
        char hex[5];
        hex[4] = '\0';
        for (int i = 0; i < 4; i++) {
            hex[3 - i] = "0123456789ABCDEF"[line & 0xF];
            line >>= 4;
        }

        for (int i = 0; i < 4; i += 2) {
            if (is_directive) {
                PRINT_COLOR_AND_VALUE(ANSI_RED, "%s", "XX XX");
                break;
            }
            char tmpstr[5];
            snprintf(tmpstr, sizeof(tmpstr), "%c%c", hex[i], hex[i + 1]);
            PRINT_COLOR_AND_VALUE(ANSI_CYAN, "%s", tmpstr);
            if (i != 2) {
                printf(" ");
            }
        }
    }

    if (!is_directive) {
        char tmpstr[20];
        if (likely_label) {
            printf("%s●", ANSI_RESET);
        } else {
            printf(" ");
        }
#ifdef _WIN32
        snprintf(tmpstr, sizeof(tmpstr), "%*llu", 5, lineclone);
#else
        snprintf(tmpstr, sizeof(tmpstr), "%*lu", 5, lineclone);
#endif
        PRINT_COLOR_AND_VALUE(ANSI_GREEN, "%s", tmpstr);
        printf("%s ", ANSI_RESET);
    } else {
        PRINT_COLOR_AND_VALUE(ANSI_RED, "%s", " XXXXX ");
    }

    printf("│ ");
}

void print_header() {
    if (args.only_code != 1) {
        if (args.binary != 1) {
            if (args.print_hex == 0) {
                if (args.colors) {
                    printf("╭────────┬───────┬─────────────╮\n");
                    printf("│  %saddr%s  │  %sbin%s  │ %sinstruction%s │\n", ANSI_CYAN, ANSI_RESET, ANSI_MAGENTA, ANSI_RESET, ANSI_BLUE, ANSI_RESET);
                    printf("├────────┼───────┼─────────────╯\n");
                } else {
                    printf("╭────────┬───────┬─────────────╮\n");
                    printf("│  addr  │  bin  │ instruction │\n");
                    printf("├────────┼───────┼─────────────╯\n");
                }
            } else {
                if (args.colors) {
                    printf("╭─────────────┬───────┬─────────────╮\n");
                    printf("│   %saddress%s   │  %sbin%s  │ %sinstruction%s │\n", ANSI_CYAN, ANSI_RESET, ANSI_MAGENTA, ANSI_RESET, ANSI_BLUE, ANSI_RESET);
                    printf("├─────────────┼───────┼─────────────╯\n");
                } else {
                    printf("╭─────────────┬───────┬─────────────╮\n");
                    printf("│   address   │  bin  │ instruction │\n");
                    printf("├─────────────┼───────┼─────────────╯\n");
                }
            }
        } else {
            if (args.print_hex == 0) {
                if (args.colors) {
                    printf("╭────────┬──────────────────────────┬─────────────╮\n");
                    printf("│  %saddr%s  │          %sbinary%s          │ %sinstruction%s │\n", ANSI_CYAN, ANSI_RESET, ANSI_MAGENTA, ANSI_RESET, ANSI_BLUE, ANSI_RESET);
                    printf("├────────┼──────────────────────────┼─────────────╯\n");
                } else {
                    printf("╭────────┬──────────────────────────┬─────────────╮\n");
                    printf("│  addr  │         binary           │ instruction │\n");
                    printf("├────────┼──────────────────────────┼─────────────╯\n");
                }
            } else {
                if (args.colors) {
                    printf("╭─────────────┬──────────────────────────┬─────────────╮\n");
                    printf("│   %saddress%s   │          %sbinary%s          │ %sinstruction%s │\n", ANSI_CYAN, ANSI_RESET, ANSI_MAGENTA, ANSI_RESET, ANSI_BLUE, ANSI_RESET);
                    printf("├─────────────┼──────────────────────────┼─────────────╯\n");
                } else {
                    printf("╭─────────────┬──────────────────────────┬─────────────╮\n");
                    printf("│   address   │         binary           │ instruction │\n");
                    printf("├─────────────┼──────────────────────────┼─────────────╯\n");
                }
            }
        }
    }
}
