#ifndef MACROS_H
#define MACROS_H
#define PRINT_COLOR_AND_VALUE(color, format, value) \
    do {                                            \
        PRINTF("%s", color);                        \
        PRINTF(format, value);                      \
        PRINTF(ANSI_RESET);                         \
    } while (0)

#ifdef _WIN32
#define PRINTF(msg, ...)                                                \
    {                                                                   \
        HANDLE hConsole = GetStdHandle(STD_OUTPUT_HANDLE);              \
        snprintf(buffer, sizeof(buffer), msg, ##__VA_ARGS__);           \
        DWORD written;                                                  \
        WriteConsole(hConsole, buffer, strlen(buffer), &written, NULL); \
    }
#else
#define PRINTF(msg, ...) printf(msg, ##__VA_ARGS__)
#endif

#define PRINT_LINE_AND_FILE printf(" on line %d in file %s\n", __LINE__, __FILE__)
#define FORMAT_STRING_MEMPTR "&0x%X"
#define FORMAT_STRING_MEMPTR_COLORED "&%s0x%X%s"
#define FORMAT_STRING_MEM_COLORED "[%s0x%X%s]"
#define FORMAT_STRING_MEM "[0x%X]"
#define FORMAT_STRING_COLORED (args.hex_operands ? "%s0x%X%s" : "%s%d%s")
#define FORMAT_STRING (args.hex_operands ? "0x%X" : "%d")
#define FORMAT_STRING_SIGNED (args.hex_operands ? "0x%X" : "-%d")

#define FORMAT_STRING_WORD_COLORED (args.hex_operands ? "%s.word%s %s0x%X%s" : "%s.word%s %s%d%s")
#define FORMAT_STRING_WORD (args.hex_operands ? ".word 0x%X" : ".word %d")

#define FORMAT_STRING_START ".start [0x%X]"
#define FORMAT_STRING_START_COLORED "%s.start%s [%s0x%X%s]"
#define FORMAT_STRING_SSP ".ssp [0x%X]"
#define FORMAT_STRING_SBP ".sbp [0x%X]"
#define FORMAT_STRING_SSP_COLORED "%s.ssp%s [%s0x%X%s]"
#define FORMAT_STRING_SBP_COLORED "%s.sbp%s [%s0x%X%s]"

#define FORMAT_STRING_ASCII_COLORED (args.hex_operands ? "%s%s%s (%s0x%X%s)" : "%s%s%s (%s%d%s)")
#define FORMAT_STRING_ASCII (args.hex_operands ? "%s (0x%X)" : "%s (%d)")

#define FORMAT_STRING_ST_COLORED "[%s0x%X%s], %sr%d%s"
#define FORMAT_STRING_ST "[0x%X], r%d"
#define ANSI_VARIED (args.hex_operands ? ANSI_CYAN : ANSI_GREEN)
#define FORMAT_STRING_COLORED_SIGNED (args.hex_operands ? "%s0x%X%s" : "-%s%d%s")
#define POSSIBLE_ANSI_BOLD (args.colors ? ANSI_BOLD : "")
#define FMTS (sign ? FORMAT_STRING_SIGNED : FORMAT_STRING)
#define FMTSC (sign ? FORMAT_STRING_COLORED_SIGNED : FORMAT_STRING_COLORED)

#endif
