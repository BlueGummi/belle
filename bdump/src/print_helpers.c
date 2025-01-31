#include "print_utils.c"

#define MAX_LINES 100
#define MAX_LENGTH 256
void print_binary(int16_t num) {
    if (in_char && !next_in_char && args.concat_chars)
        goto print_str;
    if (printed_addr && in_char && args.concat_chars)
        return;

    char hex[5];
    hex[4] = '\0';
    int numclone = num;
    for (int i = 0; i < 4; i++) {
        hex[3 - i] = "0123456789ABCDEF"[num & 0xF];
        num >>= 4;
    }

    for (int i = 0; i < 4; i += 2) {
        if (in_char && args.concat_chars) {
            PRINTF("     ");
            break;
        }
        PRINTF("%s%c%c%s", ANSI_CYAN, hex[i], hex[i + 1], ANSI_RESET);
        if (i != 2) {
            PRINTF(" ");
        }
    }
    if (args.binary) {
        if (in_char && args.concat_chars) {
            PRINTF("                   ");
            goto end;
        }
        PRINTF(" %s0b%s", ANSI_MAGENTA, ANSI_RESET);

        for (int i = 15; i >= 0; i--) {
            PRINTF("%s%d%s", ANSI_MAGENTA, (numclone >> i) & 1, ANSI_RESET);
        }
    }
end:
    if (in_char)
        printed_addr = true;
    PRINTF(" │ ");
    return;
print_str:
    PRINTF("%s%s%s\n", ANSI_BRIGHT_GREEN, global_str, ANSI_RESET);
}

#define PRINT_COLOR_AND_VALUE(color, format, value) \
    do {                                            \
        PRINTF("%s", color);                        \
        PRINTF(format, value);                      \
        PRINTF(ANSI_RESET);                         \
    } while (0)

void print_instruction_header(size_t line, bool is_directive) {
    if (printed_addr && in_char && args.concat_chars)
        return;
    PRINTF("│ ");
    char hex[5];
    hex[4] = '\0';
    for (int i = 0; i < 4; i++) {
        hex[3 - i] = "0123456789ABCDEF"[line & 0xF];
        line >>= 4;
    }

    for (int i = 0; i < 4; i += 2) {
        if (in_char && args.concat_chars) {
            PRINT_COLOR_AND_VALUE(ANSI_RED, "%s", "     ");
            break;
        }
        if (is_directive) {
            PRINT_COLOR_AND_VALUE(ANSI_RED, "%s", "XX XX  ");
            break;
        }
        char tmpstr[5];
        snprintf(tmpstr, sizeof(tmpstr), "%c%c", hex[i], hex[i + 1]);
        PRINT_COLOR_AND_VALUE(ANSI_CYAN, "%s", tmpstr);
        if (i != 2) {
            PRINTF(" ");
        }
    }

    if (!is_directive) {
        if (likely_label) {
            PRINTF("%s ●", ANSI_RESET);
        } else {
            PRINTF("  ");
        }
        if (in_char && args.concat_chars) {
            PRINTF(" │ ");
            return;
        }
    }

    PRINTF(" │ ");
}

#define PRINT_HEADER(colors, format, ...) \
    PRINTF(format, __VA_ARGS__);

void print_header(const char *metadata, char *filename) {
    char fsize[15];
    get_file_size(filename, fsize, sizeof(fsize));
    char fdate[30];
    char *fversion = "unknown";
    switch (bin_version) {
    case 1:
        fversion = "0.1";
        break;
    case 2:
        fversion = "0.2";
        break;
    case 3:
        fversion = "0.3";
        break;
    case 4:
        fversion = "0.4";
        break;
    case 5:
        fversion = "0.5";
        break;
    }
    get_last_modified_date(filename, fdate, sizeof(fdate));
    if (!args.only_code) {
        PRINTF("╭──────────────────────────────────────────────────╮\n"
               "│ %sfile%s: %s%-42s%s │\n"
               "├───────────────────────────────┬──────────────────┤\n"
               "│ %smodified%s: %s%-19s%s │ %ssize%s: %s%-10s%s │\n"
               "│ %sbinary version%s: %s%-13s%s ╰──────────────────┤\n",
               ANSI_BOLD, ANSI_RESET, ANSI_GREEN, filename, ANSI_RESET, ANSI_BOLD, ANSI_RESET, ANSI_BRIGHT_CYAN, fdate, ANSI_RESET, ANSI_BOLD, ANSI_RESET, ANSI_RED, fsize, ANSI_RESET, ANSI_MAGENTA, ANSI_RESET, ANSI_GREEN, fversion, ANSI_RESET);
        if (*metadata != '\0') {
            char metadata_buffer[1024];
            strncpy(metadata_buffer, metadata, sizeof(metadata_buffer));
            metadata_buffer[sizeof(metadata_buffer) - 1] = '\0';
            char lines[MAX_LINES][MAX_LENGTH];
            int lineCount = 0;
            int max_length = 0;

            char *line = strtok((char *) metadata, "\n");
            while (line != NULL) {
                strncpy(lines[lineCount], line, MAX_LENGTH);
                lines[lineCount][MAX_LENGTH - 1] = '\0';
                lineCount++;

                int length = strlen(line);
                if (length > max_length) {
                    max_length = length;
                }

                line = strtok(NULL, "\n");
            }
            PRINTF("╞═══════════════════════╡ %sMETADATA%s ╞═══════════════╧", ANSI_BRIGHT_GREEN, ANSI_RESET);
            for (int i = 0; i < max_length - 49; i++) {
                PRINTF("═");
            }
            PRINTF("╕\n");
            for (char *p = metadata_buffer; *p != '\0'; p++) {
                if (*p == '\n') {
                    PRINTF("│ %*s │\n", max_length, " ");
                } else {
                    char *start = p; // Start of the line
                    while (*p != '\n' && *p != '\0') {
                        p++;
                    }
                    PRINTF("│ %.*s", (int) (p - start), start);
                    for (int i = 0; i < max_length - (int) (p - start) + 1; i++) {
                        PRINTF(" ");
                    }
                    PRINTF("│\n");
                }
            }
            PRINTF("╞════════════════════╡ %sEND METADATA%s ╞══════════════╤", ANSI_BRIGHT_RED, ANSI_RESET);
            for (int i = 0; i < max_length - 49; i++) {
                PRINTF("═");
            }
            PRINTF("╛\n");
        }
        if (!args.binary) {
            PRINT_HEADER(args.colors,
                         "├─────────┬───────┬─────────────┬──────────────────╯\n"
                         "│ %saddress%s │  %sbin%s  │ %sinstruction%s │\n"
                         "├─────────┼───────┼─────────────╯\n",
                         ANSI_CYAN, ANSI_RESET, ANSI_MAGENTA, ANSI_RESET, ANSI_BLUE, ANSI_RESET);
        } else {
            PRINT_HEADER(args.colors,
                         "├─────────┬──────────────────────────┬─────────────┤\n"
                         "│ %saddress%s │          %sbinary%s          │ %sinstruction%s │\n"
                         "├─────────┼──────────────────────────┼─────────────╯\n",
                         ANSI_CYAN, ANSI_RESET, ANSI_MAGENTA, ANSI_RESET, ANSI_BLUE, ANSI_RESET);
        }
    }
}
void print_footer(void) {
    if (!args.only_code) {
        if (!args.binary) {
            PRINTF("╰─────────┴───────╯\n");
        } else {
            PRINTF("╰─────────┴──────────────────────────╯\n");
        }
    }
}
