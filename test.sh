#!/bin/bash
set -e
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
cd "$SCRIPT_DIR"

print_message() {
    local message="$1"
    local color="$2"

    local color_supported=$(tput colors 2>/dev/null)

    if [[ -t 1 && (${color_supported:-0} -ge 8) ]]; then
        case "$color" in
            green) tput setaf 2 ;;
            red) tput setaf 1 ;;
            yellow) tput setaf 3 ;;
            blue) tput setaf 4 ;;
            *) tput sgr0 ;;
        esac
        echo "$message"
        tput sgr0
    else
        echo "$message" # no color
    fi
}

print_message "Running tests on BELLE..." blue
cd belle
cargo test
cd ..
print_message "Running test on BELLE-asm..." blue
cd basm
cargo test
cd ..
print_message "Tests complete!" green
