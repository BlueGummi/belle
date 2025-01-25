#include "bdump.h"
bool in_char = false;
bool next_in_char = false;
bool likely_label = false;
bool printed_addr = false;
char global_str[512] = "";
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
void get_file_size(const char *filename, char *size_str, size_t size_str_len) {
    long file_size = 0;

#ifdef _WIN32
    HANDLE hFile = CreateFileA(filename, GENERIC_READ, FILE_SHARE_READ, NULL, OPEN_EXISTING, FILE_ATTRIBUTE_NORMAL, NULL);
    if (hFile == INVALID_HANDLE_VALUE) {
        snprintf(size_str, size_str_len, "Error getting file size");
        return;
    }
    file_size = GetFileSize(hFile, NULL);
    CloseHandle(hFile);
#else
    struct stat file_stat;
    if (stat(filename, &file_stat) != 0) {
        snprintf(size_str, size_str_len, "Error getting file size");
        return;
    }
    file_size = file_stat.st_size;
#endif

    if (file_size < 1024) {
        snprintf(size_str, size_str_len, "%ld B", file_size);
    } else if (file_size < 1048576) {
        snprintf(size_str, size_str_len, "%.2f KB", file_size / 1024.0);
    } else {
        snprintf(size_str, size_str_len, "%.2f MB", file_size / 1048576.0);
    }
}

void get_last_modified_date(const char *filename, char *date_str, size_t date_str_len) {
#ifdef _WIN32
    HANDLE hFile = CreateFileA(filename, GENERIC_READ, FILE_SHARE_READ, NULL, OPEN_EXISTING, FILE_ATTRIBUTE_NORMAL, NULL);
    if (hFile == INVALID_HANDLE_VALUE) {
        snprintf(date_str, date_str_len, "Error getting date");
        return;
    }

    FILETIME ftLastWrite;
    GetFileTime(hFile, NULL, NULL, &ftLastWrite);
    CloseHandle(hFile);

    SYSTEMTIME st;
    FileTimeToSystemTime(&ftLastWrite, &st);
    snprintf(date_str, date_str_len, "%02d/%02d/%04d %02d:%02d:%02d",
             st.wMonth, st.wDay, st.wYear, st.wHour, st.wMinute, st.wSecond);
#else
    struct stat file_stat;
    if (stat(filename, &file_stat) != 0) {
        snprintf(date_str, date_str_len, "Error getting date");
        return;
    }

    struct tm *tm_info = localtime(&file_stat.st_mtime);
    strftime(date_str, date_str_len, "%Y-%m-%d %H:%M:%S", tm_info);
#endif
}
