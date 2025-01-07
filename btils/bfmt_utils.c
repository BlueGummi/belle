#include <ctype.h>
#include <dirent.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <stdarg.h>


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


