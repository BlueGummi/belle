#include <ctype.h>
#include <dirent.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <stdarg.h>

#define ANSI_RESET "\033[0m"
#define ANSI_BOLD "\033[1m"
#define ANSI_UNDERLINE "\033[4m"
#define ANSI_BLACK "\033[30m"
#define ANSI_RED "\033[31m"
#define ANSI_GREEN "\033[32m"
#define ANSI_YELLOW "\033[33m"
#define ANSI_BLUE "\033[34m"
#define ANSI_MAGENTA "\033[35m"
#define ANSI_CYAN "\033[36m"
#define ANSI_WHITE "\033[37m"
#define ANSI_GRAY "\033[90m"
#define ANSI_LIGHT_GRAY "\033[37m"
#define ANSI_BG_BLACK "\033[40m"
#define ANSI_BG_RED "\033[41m"
#define ANSI_BG_GREEN "\033[42m"
#define ANSI_BG_YELLOW "\033[43m"
#define ANSI_BG_BLUE "\033[44m"
#define ANSI_BG_MAGENTA "\033[45m"
#define ANSI_BG_CYAN "\033[46m"
#define ANSI_BG_WHITE "\033[47m"

#define DEFAULT_MAX_INDENTATION 4
#define MAX_LINE_LENGTH 4096
#define MAX_FILES 100

void print_help(const char *program_name) {
    printf("BELLE-fmt - Format code written for the BELLE-assembler\n");
    printf("\n%s%sUsage:%s %s%s%s [OPTIONS] <FILES>\n\n", ANSI_BOLD, ANSI_UNDERLINE, ANSI_RESET, ANSI_BOLD, program_name, ANSI_RESET);
    printf("%sArguments:%s\n", ANSI_UNDERLINE, ANSI_RESET);
    printf(" <FILES> The files to format\n\n");
    printf("%s%sOptions:%s\n", ANSI_BOLD, ANSI_UNDERLINE, ANSI_RESET);
    printf("  %s-I%s, %s--max-indent%s <INDENTATION> Set the maximum indentation level "
           "(default: 4)\n", ANSI_BOLD, ANSI_RESET, ANSI_BOLD, ANSI_RESET);
    printf("  %s-t%s, %s--tabs%s Use tabs for indentation\n", ANSI_BOLD, ANSI_RESET, ANSI_BOLD, ANSI_RESET);
    printf("  %s-h%s, %s--help%s Print help\n", ANSI_BOLD, ANSI_RESET, ANSI_BOLD, ANSI_RESET);
}
char *trim(const char *str) {
    while (isspace((unsigned char) *str)) {
        str++;
    }

    char *trimmed = (char *) malloc(strlen(str) + 1);
    if (trimmed == NULL) {
        return NULL;
    }

    strlcpy(trimmed, str, sizeof(trimmed));
    return trimmed;
}

char *clone_string(const char *original) {
    char *clone = (char *) malloc(strlen(original) + 1);

    if (clone == NULL) {
        return NULL;
    }

    strlcpy(clone, original, sizeof(clone));

    return clone;
}


