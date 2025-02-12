#include "bdump.h"
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

