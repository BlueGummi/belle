#include "print_helpers.c"
const char *valid_options[] = {
    "-h", "--help",
    "-c", "--colorless",
    "-j", "--no-jump",
    "-o", "--only-code",
    "-C", "--concat-chars",
    "-b", "--binary",
    "-X", "--hex",
    "-V", "--version"};

const char *descriptions[] = {
    "Print help and exit",
    "Disable colors",
    "Disable jump visuals",
    "Print only disassembled code",
    "Concatenate characters",
    "Print instruction binary",
    "Print instruction operands in hexadecimal",
    "Print version"};
CLI parse_arguments(int argc, char *argv[]) {
    CLI opts = {0};
    opts.num_files = 0;
    opts.colors = 1;
    int valid_count = sizeof(valid_options) / sizeof(valid_options[0]);

    for (int i = 1; i < argc; i++) {
        if (strcmp(argv[i], "--help") == 0 || strcmp(argv[i], "-h") == 0) {
            print_help(argv[0]);
            exit(EXIT_SUCCESS);
        } else if (argv[i][0] == '-') {
            if (argv[i][1] == '-') {
                if (strcmp(argv[i], "--colorless") == 0) {
                    opts.colors = 0;
                } else if (strcmp(argv[i], "--binary") == 0) {
                    opts.binary = 1;
                } else if (strcmp(argv[i], "--only-code") == 0) {
                    opts.only_code = 1;
                } else if (strcmp(argv[i], "--hex") == 0) {
                    opts.hex_operands = 1;
                } else if (strcmp(argv[i], "--no-jump") == 0) {
                    opts.no_jump = 1;
                } else if (strcmp(argv[i], "--concat-chars") == 0) {
                    opts.concat_chars = 1;
                } else if (strcmp(argv[i], "--version") == 0) {
                    PRINTF("bdump 0.2.0\n");
                    exit(EXIT_SUCCESS);
                } else {
                    fputs("Error: Unknown option ", stderr);
                    fputs(argv[i], stderr);
                    fputc('\n', stderr);
                    suggest_option(argv[i], valid_count);
                    exit(EXIT_FAILURE);
                }
            } else {
                for (int j = 1; argv[i][j] != '\0'; j++) {
                    switch (argv[i][j]) {
                    case 'c':
                        opts.colors = 0;
                        break;
                    case 'b':
                        opts.binary = 1;
                        break;
                    case 'o':
                        opts.only_code = 1;
                        break;
                    case 'X':
                        opts.hex_operands = 1;
                        break;
                    case 'j':
                        opts.no_jump = 1;
                        break;
                    case 'h':
                        print_help(argv[0]);
                        exit(EXIT_SUCCESS);
                        break;
                    case 'C':
                        opts.concat_chars = 1;
                        break;
                    case 'V':
                        PRINTF("bdump 0.2.0\n");
                        exit(EXIT_SUCCESS);
                        break;
                    default:
                        fputs("Error: Unknown option -", stderr);
                        fputc(argv[i][j], stderr);
                        fputc('\n', stderr);
                        suggest_option(argv[i], valid_count);
                        exit(EXIT_FAILURE);
                    }
                }
            }
        } else {
            if (opts.num_files < MAX_INPUT_FILES) {
                opts.input_files[opts.num_files++] = argv[i];
            } else {
                fputs("Error: Too many input files specified\n", stderr);
                exit(EXIT_FAILURE);
            }
        }
    }
    return opts;
}

int levenshtein_distance(const char *s1, const char *s2) {
    int len1 = strlen(s1);
    int len2 = strlen(s2);
    int dp[len1 + 1][len2 + 1];

    for (int i = 0; i <= len1; i++) {
        for (int j = 0; j <= len2; j++) {
            if (i == 0) {
                dp[i][j] = j;
            } else if (j == 0) {
                dp[i][j] = i;
            } else {
                dp[i][j] = (s1[i - 1] == s2[j - 1]) ? dp[i - 1][j - 1] : 1 + fmin(fmin(dp[i - 1][j], dp[i][j - 1]), dp[i - 1][j - 1]);
            }
        }
    }
    return dp[len1][len2];
}

void print_help(char *bin) { // bin is the name of the bin program
    PRINTF("The disassembler for the BELLE-ISA\n\n");
    PRINTF("%s%sUsage:%s %s%s%s [OPTIONS] <ROMS>\n\n", ANSI_UNDERLINE, ANSI_BOLD, ANSI_RESET,
           ANSI_BOLD, bin, ANSI_RESET);
    PRINTF("%s%sArguments:%s\n", ANSI_BOLD, ANSI_UNDERLINE, ANSI_RESET);
    PRINTF("  <ROMS>  Path to ROMs\n\n");
    PRINTF("%s%sOptions:%s\n", ANSI_BOLD, ANSI_UNDERLINE, ANSI_RESET);
    PRINTF("  %s-h%s, %s--help%s          Show this help message and exit\n", ANSI_BOLD, ANSI_RESET,
           ANSI_BOLD, ANSI_RESET);
    PRINTF("  %s-c%s, %s--colorless%s     Disable colors\n", ANSI_BOLD, ANSI_RESET, ANSI_BOLD,
           ANSI_RESET);
    PRINTF("  %s-j%s, %s--no-jump%s       Disable jump visuals\n", ANSI_BOLD, ANSI_RESET, ANSI_BOLD,
           ANSI_RESET);
    PRINTF("  %s-o%s, %s--only-code%s     Print only disassembled code\n", ANSI_BOLD, ANSI_RESET, ANSI_BOLD,
           ANSI_RESET);
    PRINTF("  %s-C%s, %s--concat-chars%s  Concatenate characters\n", ANSI_BOLD, ANSI_RESET, ANSI_BOLD,
           ANSI_RESET);
    PRINTF("  %s-b%s, %s--binary%s        Print instruction binary\n", ANSI_BOLD, ANSI_RESET, ANSI_BOLD, ANSI_RESET);
    PRINTF("  %s-X%s, %s--hex%s           Print instruction operands in hexadecimal\n", ANSI_BOLD, ANSI_RESET, ANSI_BOLD,
           ANSI_RESET);
    PRINTF("  %s-V%s, %s--version%s       Print version\n", ANSI_BOLD,
           ANSI_RESET, ANSI_BOLD, ANSI_RESET);
    exit(0);
}

void suggest_option(const char *invalid_option, int valid_count) {
    int min_distance = 3;
    char *suggestions[MAX_SUGGESTIONS];
    int suggestion_count = 0;

    for (int i = 0; i < valid_count; i++) {
        int distance = levenshtein_distance(invalid_option, valid_options[i]);
        if (distance < min_distance && suggestion_count < MAX_SUGGESTIONS) {
            suggestions[suggestion_count++] = (char *) valid_options[i];
        }
    }

    if (suggestion_count > 0) {
        fputs("Did you mean one of these options?\n", stderr);
        for (int i = 0; i < suggestion_count; i++) {
            fputs("  ", stderr);
            fputs(suggestions[i], stderr);
            fputs(": ", stderr);
            fputs(ANSI_BOLD, stderr);
            fputs(ANSI_YELLOW, stderr);
            fputs(descriptions[i], stderr);
            fputs(ANSI_RESET, stderr);
            fputc('\n', stderr);
        }
    }
}
