#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define DEFAULT_MAX_INDENTATION 4
#define MAX_LINE_LENGTH 1024

#define BOLD "\033[1m"
#define UNDERLINE "\033[4m"
#define RESET "\033[0m"

void print_help() {
    printf(BOLD "BELLE - The Big Endian, Low Level Emulator\n" RESET);
    printf("\n" BOLD "Usage: " RESET "belle [OPTIONS] <ROM>\n\n");
    printf(BOLD "Arguments:\n" RESET);
    printf("  <ROM>  Path to ROM\n\n");
    printf(BOLD "Options:\n" RESET);
    printf("  " BOLD "-v, --verbose" RESET "                  Verbose output\n");
    printf("  " BOLD "-d, --debug" RESET "                    Enter debugger\n");
    printf("  " BOLD "-q, --quiet" RESET "                    Do not print errors\n");
    printf("  " BOLD "-t, --time-delay <TIME_DELAY>" RESET "  Clock delay (milliseconds)\n");
    printf("  " BOLD "-p, --pretty" RESET
           "                   Print the state of the CPU when it halts\n");
    printf("  " BOLD "-f, --fuzz" RESET "                     Fuzzing mode\n");
    printf("  " BOLD "-w, --write" RESET "                    Write crash to file\n");
    printf("  " BOLD "-h, --help" RESET "                     Print help\n");
    printf("  " BOLD "-V, --version" RESET "                  Print version\n");
}

void trim_and_format_line(const char *line, char *formatted_line, size_t max_indentation,
                          int use_tabs) {
    size_t leading_spaces = 0;
    while (line[leading_spaces] == ' ')
        leading_spaces++;

    const char *cut = (leading_spaces > max_indentation) ? line + leading_spaces : line;
    while (*cut == ' ')
        cut++;

    char *semicolon = strchr(cut, ';');
    if (semicolon)
        *semicolon = '\0';

    if (*cut == '\0') {
        formatted_line[0] = '\0';
        return;
    }

    char *last_colon = strrchr(cut, ':');
    int should_not_trim =
        (cut[0] == '.' && (strncmp(cut, ".asciiz", 7) != 0 && strncmp(cut, ".word", 5) != 0)) ||
        (last_colon != NULL || cut[0] == ';');

    if (should_not_trim) {
        strcpy(formatted_line, line);
    } else {
        const char *indent = use_tabs ? "\t" : "    ";
        sprintf(formatted_line, "%s%s", indent, cut);
    }
}

void process_file(const char *filename, size_t max_indentation, int use_tabs) {
    char temp_filename[256];
    sprintf(temp_filename, "%s.tmp", filename);
    FILE *input_file = fopen(filename, "r");
    FILE *output_file = fopen(temp_filename, "w");

    if (!input_file || !output_file) {
        perror("Error opening file");
        exit(EXIT_FAILURE);
    }

    char line[MAX_LINE_LENGTH];
    while (fgets(line, sizeof(line), input_file)) {
        char formatted_line[MAX_LINE_LENGTH];
        trim_and_format_line(line, formatted_line, max_indentation, use_tabs);
        if (formatted_line[0] != '\0') {
            fprintf(output_file, "%s\n", formatted_line);
        }
    }

    fclose(input_file);
    fclose(output_file);
    rename(temp_filename, filename);
}

int main(int argc, char *argv[]) {
    if (argc < 2) {
        print_help();
        return EXIT_FAILURE;
    }
    size_t max_indentation = DEFAULT_MAX_INDENTATION;
    int use_tabs = 0;

    for (int i = 1; i < argc; i++) {
        if (strcmp(argv[i], "--help") == 0 || strcmp(argv[i], "-h") == 0) {
            print_help();
            return EXIT_SUCCESS;
        }
        if (strcmp(argv[i], "--tabs") == 0 || strcmp(argv[i], "-t") == 0) {
            use_tabs = 1;
        } else if (strncmp(argv[i], "--max-indent=", 13) == 0) {
            max_indentation = atoi(argv[i] + 13);
        } else {
            process_file(argv[i], max_indentation, use_tabs);
        }
    }

    return EXIT_SUCCESS;
}
