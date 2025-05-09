#include "bfmt_utils.c"
bool starts_with_semicolon(const char *str) {
    while (isspace((unsigned char) *str)) {
        str++;
    }

    return *str == ';';
}

void trim_and_format_line(char *line, char *formatted_line, size_t max_indentation, int use_tabs) {
    char *lineclone = clone_string(line);
    if (strcmp(line, "\n") == 0) {
        formatted_line = "\n\n";
        return;
    }
    if (lineclone == NULL) {
        perror("Memory allocation failed\n");
        return;
    }

    size_t leading_spaces = 0;
    while (lineclone[leading_spaces] == ' ') {
        leading_spaces++;
    }

    char *cut = (leading_spaces > max_indentation) ? &lineclone[leading_spaces] : lineclone;
    cut = cut + strspn(cut, " ");

    char *comment_pos = strchr(cut, ';');
    if (comment_pos != NULL) {
        *comment_pos = '\0';
    }

    cut = cut + strspn(cut, " ");

    if (*cut == '\0') {
        formatted_line[0] = '\0';
        free(lineclone);
        return;
    }
    bool should_not_trim = false;
    char *last_colon = strrchr(cut, ':');

    if (*cut == '.') {
        should_not_trim = !(strstr(cut, ".asciiz") == cut || strstr(cut, ".word") == cut);
    } else {
        should_not_trim = (last_colon != NULL || *cut == ';');
    }

    char *trimmed_line = trim(line);
    if (trimmed_line == NULL) {
        perror("Memory allocation failed\n");
        free(lineclone);
        return;
    }

    if (should_not_trim) {
        safe_strcpy(formatted_line, trimmed_line, MAX_LINE_LENGTH);
    } else {
        if (use_tabs) {
            snprintf(formatted_line, MAX_LINE_LENGTH, "\t%s", trimmed_line);
        } else {
            snprintf(formatted_line, MAX_LINE_LENGTH, "%*s%s", (int) max_indentation, "",
                     trimmed_line);
        }
    }

    free(trimmed_line);
    free(lineclone);
}
void process_file(FILE *input_file, FILE *output_file, size_t max_indentation, int use_tabs) {
    char line[MAX_LINE_LENGTH];
    while (fgets(line, sizeof(line), input_file)) {
        if (strcmp(line, "\n") == 0) {
            fprintf(output_file, "%s", "\n");
            continue;
        }
        if (starts_with_semicolon(line)) {
            fprintf(output_file, "%s", line);
            continue;
        }
        char formatted_line[MAX_LINE_LENGTH] = {0};
        trim_and_format_line(line, formatted_line, max_indentation, use_tabs);
        fprintf(output_file, "%s", formatted_line);
    }
}

int main(int argc, char *argv[]) {
    size_t max_indentation = DEFAULT_MAX_INDENTATION;
    int use_tabs = 0;
    char *files[MAX_FILES];
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
        } else if ((strcmp(argv[i], "-V") == 0) || (strncmp(argv[i], "--version", 9) == 0)) {
            printf("bfmt 0.2.0\n");
            exit(0);
        } else {
            if (file_count < MAX_FILES) {
                files[file_count++] = argv[i];
            } else {
                fprintf(stderr, "Too many files specified.\n");
                return EXIT_FAILURE;
            }
        }
    }

    if (file_count == 0 && isatty(STDIN_FILENO)) {
        print_help(argv[0]);
        return EXIT_SUCCESS;
    } else if (file_count == 0) {
        process_file(stdin, stdout, max_indentation, use_tabs);
    } else {
        for (int i = 0; i < file_count; i++) {
            char temp_filename[256];
            snprintf(temp_filename, sizeof(temp_filename), "%s.tmp", files[i]);
            FILE *input_file = fopen(files[i], "r");
            FILE *output_file = fopen(temp_filename, "w");

            if (!input_file || !output_file) {
                perror("Error opening file");
                remove(temp_filename);
                exit(EXIT_FAILURE);
            }

            process_file(input_file, output_file, max_indentation, use_tabs);

            fclose(input_file);
            fclose(output_file);
            if (rename(temp_filename, files[i]) != 0) {
                perror("Error renaming file");
                remove(temp_filename);
                exit(EXIT_FAILURE);
            }
        }
    }

    return EXIT_SUCCESS;
}
