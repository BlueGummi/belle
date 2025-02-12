#ifndef BDUMP_H
#define BDUMP_H

#include <errno.h>
#include <inttypes.h>
#include <math.h>
#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#ifdef _WIN32
#include <windows.h>
#else
#include <sys/stat.h>
#include <unistd.h>
#endif

#include "consts.h"
#include "colors.h"
#include "structures.h"
#include "macros.h"
#include "prototypes.h"
CLI args = {0};

bool is_term = true;
int bin_version = 0;
int start_loc = 100;
bool in_char = false;
bool next_in_char = false;
bool likely_label = false;
bool printed_addr = false;
char global_str[65536] = "";

#ifdef _WIN32
char buffer[1024];
#endif

HashMap *jump_map_global;
size_t current_addr = 100;
uint64_t len = 0;
#endif
