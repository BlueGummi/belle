#include <ctype.h>
#include <dirent.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

#define DEFAULT_MAX_INDENTATION 4
#define MAX_LINE_LENGTH 4096
#define MAX_FILES 100

void print_help(const char *program_name) {
    printf("bfmt - Format code written for the BELLE-assembler\n");
    printf("\nUsage: %s [OPTIONS] <FILES>\n\n", program_name);
    printf("Arguments:\n");
    printf(" <FILES> The files to format\n\n");
    printf("Options:\n");
    printf("  -I, --max-indent <INDENTATION> Set the maximum indentation level "
           "(default: 4)\n");
    printf("  -t, --tabs Use tabs for indentation\n");
    printf("  -h, --help Print help\n");
}
char *trim(const char *str) {
    while (isspace((unsigned char) *str)) {
        str++;
    }

    char *trimmed = (char *) malloc(strlen(str) + 1);
    if (trimmed == NULL) {
        return NULL;
    }

    strcpy(trimmed, str);
    return trimmed;
}
char *clone_string(const char *original) {
    char *clone = (char *) malloc(strlen(original) + 1);

    if (clone == NULL) {
        return NULL;
    }

    strcpy(clone, original);

    return clone;
}

void trim_and_format_line(char *line, char *formatted_line, size_t max_indentation, int use_tabs) {
    char *lineclone = clone_string(line);
    size_t leading_spaces = 0;
    while (lineclone[leading_spaces] == ' ') {
        leading_spaces++;
    }

    char *cut_line = &lineclone[leading_spaces];
    cut_line = cut_line + strspn(cut_line, " ");

    char *cut = (leading_spaces > max_indentation) ? &lineclone[leading_spaces] : lineclone;
    cut = cut + strspn(cut, " ");

    char *comment_pos = strchr(cut, ';');
    if (comment_pos != NULL) {
        *comment_pos = '\0';
    }

    cut = cut + strspn(cut, " ");

    if (*cut == '\0') {
        formatted_line[0] = '\0';
        return;
    }

    bool should_not_trim = false;
    char *last_colon = strrchr(cut, ':');

    if (*cut == '.') {
        should_not_trim = !(strstr(cut, ".asciiz") == cut || strstr(cut, ".word") == cut);
    } else {
        should_not_trim = (last_colon != NULL || *cut == ';');
    }

    if (should_not_trim) {
        strcpy(formatted_line, trim(line));
    } else {
        if (use_tabs) {
            sprintf(formatted_line, "\t%s", trim(line));
        } else {
            sprintf(formatted_line, "%*s%s", (int) max_indentation, "", trim(line));
        }
    }
    free(lineclone);
}

void process_file(const char *filename, size_t max_indentation, int use_tabs) {
    char temp_filename[256];
    snprintf(temp_filename, sizeof(temp_filename), "%s.tmp", filename);
    FILE *input_file = fopen(filename, "r");
    FILE *output_file = fopen(temp_filename, "w");

    if (!input_file || !output_file) {
        perror("Error opening file");
	remove(temp_filename);
        exit(EXIT_FAILURE);
    }

    char line[MAX_LINE_LENGTH];
    while (fgets(line, sizeof(line), input_file)) {
        char formatted_line[MAX_LINE_LENGTH] = {0};
        trim_and_format_line(line, formatted_line, max_indentation, use_tabs);
        if (formatted_line[0] != '\0') {
            fprintf(output_file, "%s", formatted_line);
        }
    }

    fclose(input_file);
    fclose(output_file);
    if (rename(temp_filename, filename) != 0) {
        perror("Error renaming file");
        exit(EXIT_FAILURE);
    }
}

int main(int argc, char *argv[]) {
    if (argc < 2) {
        print_help(argv[0]);
        return EXIT_SUCCESS;
    }

    size_t max_indentation = DEFAULT_MAX_INDENTATION;
    int use_tabs = 0;
    char* files[MAX_FILES];
    int file_count = 0;

    for (int i = 1; i < argc; i++) {
        if (strcmp(argv[i], "--help") == 0 || strcmp(argv[i], "-h") == 0) {
            print_help(argv[0]);
            return EXIT_SUCCESS;
        } else if (strcmp(argv[i], "--tabs") == 0 || strcmp(argv[i], "-t") == 0) {
            use_tabs = 1;
        } else if (strncmp(argv[i], "--max-indent=", 13) == 0) {
            const char *value = argv[i] + 13;
            max_indentation = atoi(value);
        } else if (strcmp(argv[i], "-I") == 0 && i + 1 < argc) {
            max_indentation = strtoul(argv[++i], NULL, 10);
        } else {
            if (file_count < MAX_FILES) {
                files[file_count++] = argv[i];
            } else {
                fprintf(stderr, "Too many files specified.\n");
                return EXIT_FAILURE;
            }
        }
    }

    for (int i = 0; i < file_count; i++) {
        process_file(files[i], max_indentation, use_tabs);
    }

    return EXIT_SUCCESS;
}
