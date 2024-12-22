#!/usr/bin/env python3
"""
 " Copyright (c) 2024 BlueGummi
 " All rights reserved.
 "
 " This code is licensed under the BSD 3-Clause License.
"""
import sys
import os

MAX_INDENTATION = 4

def trim_and_format_line(line):
    leading_spaces = len(line) - len(line.lstrip(' '))

    if leading_spaces > MAX_INDENTATION:
        line = line[leading_spaces:]
        leading_spaces = MAX_INDENTATION

    stripped_line = line.lstrip()

    if not stripped_line:
        return ''

    should_trim = False

    last_index = stripped_line.rfind(':')

    if stripped_line[0] == '.' or (last_index != -1 and stripped_line[last_index] == ':'):
        should_trim = True
    elif stripped_line[-2] != ';':
        should_trim = False

    if should_trim:
        return stripped_line
    else:
        return ' ' * MAX_INDENTATION + stripped_line

def process_file(filename):
    temp_filename = f"{filename}.tmp"

    try:
        with open(filename, 'r') as input_file, open(temp_filename, 'w') as output_file:
            for line in input_file:
                formatted_line = trim_and_format_line(line)
                if formatted_line:
                    output_file.write(formatted_line)

        os.replace(temp_filename, filename)

    except Exception as e:
        print(f"Error processing {filename}: {e}")
        if os.path.exists(temp_filename):
            os.remove(temp_filename)

def main():
    if len(sys.argv) < 2:
        print(f"Usage: {sys.argv[0]} <file1> [file2 ...]")
        sys.exit(1)

    for filename in sys.argv[1:]:
        process_file(filename)

if __name__ == "__main__":
    main()
