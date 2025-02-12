#ifndef COLORS_H
#define COLORS_H

typedef enum {
    COLOR_RED,
    COLOR_GREEN,
    COLOR_YELLOW,
    COLOR_BLUE,
    COLOR_MAGENTA,
    COLOR_CYAN,
    COLOR_WHITE,
    COLOR_GRAY,
    COLOR_LIGHT_GRAY,
    COLOR_BG_BLACK,
    COLOR_BG_RED,
    COLOR_BG_GREEN,
    COLOR_BG_YELLOW,
    COLOR_BG_BLUE,
    COLOR_BG_MAGENTA,
    COLOR_BG_CYAN,
    COLOR_BG_WHITE,
} Color;

#define ANSI_RESET (is_term ? "\033[0m" : "")
#define ANSI_BOLD (args.colors && is_term ? "\033[1m" : "")
#define ANSI_UNDERLINE (args.colors && is_term ? "\033[4m" : "")
#define ANSI_BLACK (args.colors && is_term ? "\033[30m" : "")
#define ANSI_RED (args.colors && is_term ? "\033[31m" : "")
#define ANSI_GREEN (args.colors && is_term ? "\033[32m" : "")
#define ANSI_YELLOW (args.colors && is_term ? "\033[33m" : "")
#define ANSI_BLUE (args.colors && is_term ? "\033[34m" : "")
#define ANSI_MAGENTA (args.colors && is_term ? "\033[35m" : "")
#define ANSI_CYAN (args.colors && is_term ? "\033[36m" : "")
#define ANSI_WHITE (args.colors && is_term ? "\033[37m" : "")
#define ANSI_GRAY (args.colors && is_term ? "\033[90m" : "")
#define ANSI_LIGHT_GRAY (args.colors && is_term ? "\033[37m" : "")
#define ANSI_BG_BLACK (args.colors && is_term ? "\033[40m" : "")
#define ANSI_BG_RED (args.colors && is_term ? "\033[41m" : "")
#define ANSI_BG_GREEN (args.colors && is_term ? "\033[42m" : "")
#define ANSI_BG_YELLOW (args.colors && is_term ? "\033[43m" : "")
#define ANSI_BG_BLUE (args.colors && is_term ? "\033[44m" : "")
#define ANSI_BG_MAGENTA (args.colors && is_term ? "\033[45m" : "")
#define ANSI_BG_CYAN (args.colors && is_term ? "\033[46m" : "")
#define ANSI_BG_WHITE (args.colors && is_term ? "\033[47m" : "")
#define ANSI_BRIGHT_BLACK (args.colors && is_term ? "\033[90m" : "")
#define ANSI_BRIGHT_RED (args.colors && is_term ? "\033[91m" : "")
#define ANSI_BRIGHT_GREEN (args.colors && is_term ? "\033[92m" : "")
#define ANSI_BRIGHT_YELLOW (args.colors && is_term ? "\033[93m" : "")
#define ANSI_BRIGHT_BLUE (args.colors && is_term ? "\033[94m" : "")
#define ANSI_BRIGHT_MAGENTA (args.colors && is_term ? "\033[95m" : "")
#define ANSI_BRIGHT_CYAN (args.colors && is_term ? "\033[96m" : "")
#define ANSI_BRIGHT_WHITE (args.colors && is_term ? "\033[97m" : "")
#define ANSI_RED_CONST "\033[31m"
#define ANSI_BOLD_CONST "\033[1m"

#endif
