#include "print_helpers.c"
CLI parse_arguments(int argc, char *argv[]) {
    CLI opts        = {0};
    opts.input_file = NULL;
    bool seen_color = false;
    for (int i = 1; i < argc; i++) {
        if (strcmp(argv[i], "--help") == 0 || strcmp(argv[i], "-h") == 0) {
            print_help(argv[0]);
            exit(EXIT_SUCCESS);
        } else if (argv[i][0] == '-') {
            if (argv[i][1] == '-') {
                if (strcmp(argv[i], "--colorless") == 0) {
                    seen_color  = true;
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
                } else if (strcmp(argv[i], "--help") == 0) {
                    print_help(argv[0]);
                    exit(EXIT_SUCCESS);
                } else {
                    fputs("Error: Unknown option ", stderr);
                    fputs(argv[i], stderr);
                    fputc('\n', stderr);
                    print_help(argv[0]);
                    exit(EXIT_FAILURE);
                }
            } else {
                for (int j = 1; argv[i][j] != '\0'; j++) {
                    switch (argv[i][j]) {
                    case 'c':
                        seen_color  = true;
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
                        print_help(argv[0]);
                        exit(EXIT_FAILURE);
                    }
                }
            }
        } else {
            if (opts.input_file == NULL) {
                opts.input_file = argv[i];
            } else {
                fputs("Error: Unexpected argument: ", stderr);
                fputs(argv[i], stderr);
                fputc('\n', stderr);
                print_help(argv[0]);
                exit(EXIT_FAILURE);
            }
        }
    }
    if (!seen_color) {
        opts.colors = 1;
    }
    return opts;
}
