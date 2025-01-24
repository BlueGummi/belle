#include "print_helpers.c"
CLI parse_arguments(int argc, char *argv[]) {
    CLI opts = {0};
    opts.num_files = 0;
    bool seen_color = false;

    opts.colors = 1;
    const char *valid_options[] = {
        "--help", "--colorless", "--verbose", "--hex-mem", "--binary",
        "--only-code", "--hex", "-h", "-c", "-v", "-b", "-o", "-x", "-X"};
    int valid_count = sizeof(valid_options) / sizeof(valid_options[0]);

    for (int i = 1; i < argc; i++) {
        if (strcmp(argv[i], "--help") == 0 || strcmp(argv[i], "-h") == 0) {
            print_help(argv[0]);
            exit(EXIT_SUCCESS);
        } else if (argv[i][0] == '-') {
            if (argv[i][1] == '-') {
                if (strcmp(argv[i], "--colorless") == 0) {
                    seen_color = true;
                    opts.colors = 0;
                } else if (strcmp(argv[i], "--verbose") == 0) {
                    opts.verbosity++;
                } else if (strcmp(argv[i], "--hex-mem") == 0) {
                    opts.print_hex = 1;
                } else if (strcmp(argv[i], "--binary") == 0) {
                    opts.binary = 1;
                } else if (strcmp(argv[i], "--only-code") == 0) {
                    opts.only_code = 1;
                } else if (strcmp(argv[i], "--hex") == 0) {
                    opts.hex_operands = 1;
                } else {
                    fputs("Error: Unknown option ", stderr);
                    fputs(argv[i], stderr);
                    fputc('\n', stderr);
                    suggest_option(argv[i], valid_options, valid_count);
                    exit(EXIT_FAILURE);
                }
            } else {
                for (int j = 1; argv[i][j] != '\0'; j++) {
                    switch (argv[i][j]) {
                    case 'c':
                        seen_color = true;
                        opts.colors = 0;
                        break;
                    case 'v':
                        opts.verbosity++;
                        break;
                    case 'b':
                        opts.binary = 1;
                        break;
                    case 'o':
                        opts.only_code = 1;
                        break;
                    case 'x':
                        opts.print_hex = 1;
                        break;
                    case 'X':
                        opts.hex_operands = 1;
                        break;
                    case 'h':
                        print_help(argv[0]);
                        exit(EXIT_SUCCESS);
                        break;
                    default:
                        fputs("Error: Unknown option -", stderr);
                        fputc(argv[i][j], stderr);
                        fputc('\n', stderr);
                        suggest_option(argv[i], valid_options, valid_count);
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
    if (!seen_color) {
        opts.colors = 1;
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

void suggest_option(const char *invalid_option, const char *valid_options[], int valid_count) {
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
    } else {
        fputs("No similar arguments found\n", stderr);
    }
    for (int i = 0; i < suggestion_count; i++) {
        fputs("  ", stderr);
        fputs(suggestions[i], stderr);
        fputc('\n', stderr);
    }
}
