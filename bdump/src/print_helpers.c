#include "print_utils.c"
void print_binary(int num, int leading) {

    char hex[9];
    hex[8] = '\0';

    for (int i = 0; i < 8; i++) {
        hex[7 - i] = "0123456789ABCDEF"[num & 0xF];
        num >>= 4;
    }

    for (int i = 0; i < 8; i += 2) {
        if (args.colors) {
            printf("%s%c%c%s", ANSI_CYAN, hex[i], hex[i + 1], ANSI_RESET);
        } else {
            printf("%c%c", hex[i], hex[i + 1]);
        }
        if (i != 6) {
            printf(" ");
        }
    }

    if (args.binary == 1) {
        if (args.colors) {
            printf(" %s0b%s", ANSI_MAGENTA, ANSI_RESET);
        } else {
            printf(" 0b");
        }
        for (int i = (leading * 4) - 1; i >= 0; i--) {
            if (args.colors) {
                printf("%s%d%s", ANSI_MAGENTA, (num >> i) & 1, ANSI_RESET);
            } else {
                printf("%d", (num >> i) & 1);
            }
        }
    }
    printf(" │ ");
}

void print_help(char *bin) { // bin is the name of the bin
    printf("The disassembler for BELLE-ISA\n\n");
    printf("%s%sUsage:%s %s%s%s [OPTIONS] <BINARY>\n\n", ANSI_UNDERLINE, ANSI_BOLD, ANSI_RESET,
           ANSI_BOLD, bin, ANSI_RESET);
    printf("%s%sArguments:%s\n", ANSI_BOLD, ANSI_UNDERLINE, ANSI_RESET);
    printf("  <BINARY> Path to binary\n\n");
    printf("%s%sOptions:%s\n", ANSI_BOLD, ANSI_UNDERLINE, ANSI_RESET);
    printf("  %s-h%s, %s--help%s       Show this help message and exit\n", ANSI_BOLD, ANSI_RESET,
           ANSI_BOLD, ANSI_RESET);
    printf("  %s-b%s, %s--binary%s     Print binary\n", ANSI_BOLD, ANSI_RESET, ANSI_BOLD, ANSI_RESET);
    printf("  %s-c%s, %s--colors%s     Enable colored output\n", ANSI_BOLD, ANSI_RESET, ANSI_BOLD,
           ANSI_RESET);
    printf("  %s-d%s, %s--debug%s      Print debug messages\n", ANSI_BOLD, ANSI_RESET, ANSI_BOLD,
           ANSI_RESET);
    printf("  %s-o%s, %s--only-code%s  Print only code\n", ANSI_BOLD, ANSI_RESET, ANSI_BOLD,
           ANSI_RESET);
    printf("  %s-v%s, %s--verbose%s    Increase verbosity level (use multiple for more)\n", ANSI_BOLD,
           ANSI_RESET, ANSI_BOLD, ANSI_RESET);
    exit(0);
}

void print_instruction_header(size_t line, bool colors, bool is_directive) {
    if (colors) {
        if (!is_directive) {
            printf("%s%*lu │ %s", ANSI_RED, 4, line, ANSI_RESET);
        } else {
            printf("%sXXXX │ %s", ANSI_RED, ANSI_RESET);
        }
    } else {
        if (!is_directive) {
            printf("%*lu │ ", 4, line);
        } else {
            printf("XXXX │ ");
        }
    }
}
