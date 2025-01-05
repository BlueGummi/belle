#pragma once
int line = 1;
void print_binary(int num, int leading) {
    if (args.binary == 1) {
        printf("\n");
        for (int i = leading - 1; i >= 0; i--) {
            printf("%d", (num >> i) & 1);
        }

        printf("\n");
    }
}

void print_help(char *bin) { // bin is the name of the bin
    printf("The disassembler for %sBELLE-ISA%s\n\n", ANSI_BOLD, ANSI_RESET);
    printf("%s%sUsage:%s %s%s%s [OPTIONS] <FILE>\n\n", ANSI_UNDERLINE, ANSI_BOLD, ANSI_RESET,
           ANSI_BOLD, bin, ANSI_RESET);
    printf("%s%sArguments:%s\n", ANSI_BOLD, ANSI_UNDERLINE, ANSI_RESET);
    printf("  <FILE> Path to input\n\n");
    printf("%s%sOptions:%s\n", ANSI_BOLD, ANSI_UNDERLINE, ANSI_RESET);
    printf("  %s-h%s, %s--help%s       Show this help message and exit\n", ANSI_BOLD, ANSI_RESET,
           ANSI_BOLD, ANSI_RESET);
    printf("  %s-l%s, %s--line-num%s   Enable line numbering\n", ANSI_BOLD, ANSI_RESET, ANSI_BOLD,
           ANSI_RESET);
    printf("  %s-b%s, %s--binary%s     Print binary\n", ANSI_BOLD, ANSI_RESET, ANSI_BOLD,
           ANSI_RESET);
    printf("  %s-c%s, %s--colors%s     Enable colored output\n", ANSI_BOLD, ANSI_RESET, ANSI_BOLD,
           ANSI_RESET);
    printf("  %s-d%s, %s--debug%s      Print debug messages\n", ANSI_BOLD, ANSI_RESET, ANSI_BOLD,
           ANSI_RESET);
    printf("  %s-v%s, %s--verbose%s    Increase verbosity level (use multiple for more)\n",
           ANSI_BOLD, ANSI_RESET, ANSI_BOLD, ANSI_RESET);
    exit(0);
}

void print(const char *format, ...) {
    if (args.debug == 1) {
        printf(ANSI_GREEN "DEBUG: " ANSI_RESET);
        va_list arguments;
        va_start(arguments, format);
        vprintf(format, arguments);
        va_end(arguments);
    }
}
void print_instruction_header(int line, bool colors) {
    if (colors) {
        printf("%sline %*d:%s ", ANSI_RED, 3, line, ANSI_RESET);
    } else {
        printf("line %*d: ", 3, line);
    }
}
