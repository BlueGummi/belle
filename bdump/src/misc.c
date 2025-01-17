#include "bdump.h"

Color get_color(int index) {
    Color color_codes[] = {
        COLOR_RED, COLOR_GREEN, COLOR_YELLOW,
        COLOR_BLUE, COLOR_MAGENTA, COLOR_CYAN, COLOR_WHITE,
        COLOR_GRAY};

    int num_codes = sizeof(color_codes) / sizeof(color_codes[0]);

    index = index % num_codes;

    return color_codes[index];
}

const char *color_to_ansi(Color color) {
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
const char *get_color_name(Color color) {
    char *color_name = malloc(50);

    if (color_name == NULL) {
        return "Memory allocation failed";
    }

    switch (color) {
    case COLOR_RED:
        strcpy(color_name, "Red");
        break;
    case COLOR_GREEN:
        strcpy(color_name, "Green");
        break;
    case COLOR_YELLOW:
        strcpy(color_name, "Yellow");
        break;
    case COLOR_BLUE:
        strcpy(color_name, "Blue");
        break;
    case COLOR_MAGENTA:
        strcpy(color_name, "Magenta");
        break;
    case COLOR_CYAN:
        strcpy(color_name, "Cyan");
        break;
    case COLOR_WHITE:
        strcpy(color_name, "White");
        break;
    case COLOR_GRAY:
        strcpy(color_name, "Gray");
        break;
    case COLOR_LIGHT_GRAY:
        strcpy(color_name, "Light Gray");
        break;
    case COLOR_BG_BLACK:
        strcpy(color_name, "Background Black");
        break;
    case COLOR_BG_RED:
        strcpy(color_name, "Background Red");
        break;
    case COLOR_BG_GREEN:
        strcpy(color_name, "Background Green");
        break;
    case COLOR_BG_YELLOW:
        strcpy(color_name, "Background Yellow");
        break;
    case COLOR_BG_BLUE:
        strcpy(color_name, "Background Blue");
        break;
    case COLOR_BG_MAGENTA:
        strcpy(color_name, "Background Magenta");
        break;
    case COLOR_BG_CYAN:
        strcpy(color_name, "Background Cyan");
        break;
    case COLOR_BG_WHITE:
        strcpy(color_name, "Background White");
        break;
    default:
        strcpy(color_name, "Unknown Color");
        break;
    }

    char *result = malloc(50);
    snprintf(result, 50, "%s%s%s", color_to_ansi(color), color_name, ANSI_RESET);

    free(color_name);

    return result;
}
