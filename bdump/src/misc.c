#include "bdump.h"
bool likely_label = false;
Color get_color(int index) {
    Color color_codes[] = {
        COLOR_RED, COLOR_GREEN, COLOR_YELLOW,
        COLOR_BLUE, COLOR_MAGENTA, COLOR_CYAN, COLOR_WHITE,
        COLOR_GRAY};

    int num_codes = sizeof(color_codes) / sizeof(color_codes[0]);

    index = index % num_codes;

    return color_codes[index];
}

char *color_to_ansi(Color color) {
    switch (color) {
    case COLOR_RED:
        return ANSI_RED;
    case COLOR_GREEN:
        return ANSI_GREEN;
    case COLOR_YELLOW:
        return ANSI_YELLOW;
    case COLOR_BLUE:
        return ANSI_BLUE;
    case COLOR_MAGENTA:
        return ANSI_MAGENTA;
    case COLOR_CYAN:
        return ANSI_CYAN;
    case COLOR_WHITE:
        return ANSI_WHITE;
    case COLOR_GRAY:
        return ANSI_GRAY;
    case COLOR_LIGHT_GRAY:
        return ANSI_LIGHT_GRAY;
    case COLOR_BG_BLACK:
        return ANSI_BG_BLACK;
    case COLOR_BG_RED:
        return ANSI_BG_RED;
    case COLOR_BG_GREEN:
        return ANSI_BG_GREEN;
    case COLOR_BG_YELLOW:
        return ANSI_BG_YELLOW;
    case COLOR_BG_BLUE:
        return ANSI_BG_BLUE;
    case COLOR_BG_MAGENTA:
        return ANSI_BG_MAGENTA;
    case COLOR_BG_CYAN:
        return ANSI_BG_CYAN;
    case COLOR_BG_WHITE:
        return ANSI_BG_WHITE;
    default:
        return ANSI_CYAN;
    }
}
