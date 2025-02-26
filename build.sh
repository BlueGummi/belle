#!/usr/bin/env bash
set -e
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
cd "$SCRIPT_DIR"
cargo_installed=false
check_deps() {
    if ! command -v cargo &> /dev/null; then
        print_message "Cargo is not installed. Would you like to install it? [y/N]" yellow

        read -r user_input

        if [[ "$user_input" =~ ^[yY]$ ]]; then
            print_message "Installing Cargo..." yellow
            curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
            source "$HOME/.cargo/env"
            print_message "Cargo installed successfully!" green
            cargo_installed=true
        else
            print_message "Cargo installation skipped." red
            exit
        fi
    fi

    if ! command -v make &> /dev/null; then
        print_message "Make is not installed. Please install it." red
        exit
    fi

    if ! command -v gcc &> /dev/null; then
        print_message "GCC is not installed. Please install it." red
        exit
    fi
}
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

clear_line() {
    printf "\r\033[K"
}

clean() {
    print_message "Cleaning up..." blue
    cd basm
    cargo clean --quiet
    cd ..
    cd bdump
    make clean --quiet
    cd ..
    cd belle
    cargo clean --quiet
    cd fuzz
    cargo clean --quiet
    cd ../../
    cd btils
    make clean --quiet
    cd ../
    cd site
    rm -rf node_modules
    cd ..
    print_message "Cleaned up!" green
}

spinner() {
    if [ "$no_spin" ]; then
        return
    fi

    local pid=$1
    local delay=0.1
    local spin='/-\|'
    local msg=$2
    print_message "$msg" yellow
    local i=0
    while ps -p $pid > /dev/null; do
        local temp=${spin:i++%${#spin}:1}
        printf "\r$temp"
        sleep $delay
    done
    clear_line
    print_message "Done!" green
}

print_help() {
    local color_supported=$(tput colors 2>/dev/null)

    if [[ -t 1 && (${color_supported:-0} -ge 8) ]]; then
        underline=$(tput smul)
        reset=$(tput sgr0)
    else
        underline=""
        reset=""
    fi

    printf "The build script for the BELLE programs and utilities\n\n"
    printf "${underline}Usage${reset}: $1 [OPTIONS] [TARGETS]\n"
    printf "Options:\n"
    printf "  -c, --clean        Clean the build directories (doesn't build)\n"
    printf "  -w, --with-cleanup Clean directories after building\n"
    printf "  -q, --quiet        Suppress output\n"
    printf "  -n, --no-spin      Disable the spinner during builds\n"
    printf "  -h, --help         Display this help message\n"
    printf "  -l, --loud         Print build outputs\n"
    printf "  -N, --nodisplay    No display suppott\n"
    printf "\nTargets:\n"
    printf "  bdump, basm, belle, btils (default: all)\n"
    exit 0
}

default_build() {
    if [ ! -d "bin" ]; then
        mkdir bin
    fi
    if [ "$clean" ]; then
        clean
        exit 0
    fi
    for target in "${targets[@]}"; do
        case "$target" in
            basm)
                cd basm
                if ! [ "$loud" ]; then
                        cargo build --release --quiet &
                else
                        cargo build --release -v &
                fi
                pid=$!
                if [ -z "$no_spin" ]; then
                    spinner $pid "Building BELLE-asm..."
                else
                    echo "Building BELLE-asm..."
                    wait $pid
                fi
                cp -f target/release/basm ../bin
                cd ..
                ;;
            bdump)
                cd bdump
                if ! [ "$loud" ]; then
                        make clean --quiet
                        make --quiet &
                else
                        make clean
                        make &
                fi
                pid=$!
                if [ -z "$no_spin" ]; then
                    spinner $pid "Building BELLE-dump..."
                else
                    echo "Building BELLE-dump..."
                    wait $pid
                fi
                cp -f bdump ../bin
                cd ..
                ;;
            belle)
                cd belle
                if ! [ "$loud" ]; then
			if [ "$nodisplay" ]; then
                        	cargo build --release --quiet &
			else
				cargo build -r --quiet --features "window" &
			fi
                else
			if [ "$nodisplay" ]; then
                        	cargo build -rv &
			else
				cargo build -rv --features "window" &
			fi
                fi
                pid=$!
                if [ -z "$no_spin" ]; then
                    spinner $pid "Building BELLE..."
                else
                    echo "Building BELLE..."
                    wait $pid
                fi
                cp -f target/release/belle ../bin
                cd ..
                ;;
            btils)
                cd btils
                if ! [ "$loud" ]; then
                        make --quiet &
                else
                        make &
                fi
                pid=$!
                if [ -z "$no_spin" ]; then
                    spinner $pid "Building BELLE-fmt..."
                else
                    echo "Building BELLE-fmt..."
                    wait $pid
                fi
                cp -f bfmt ../bin
                cd ..
                ;;
        esac
    done

    if [ "$with_cleanup" ]; then
        clean
    fi

    print_message "Build complete" green
    exit 0
}

check_deps

targets=()

for arg in "$@"; do
    case $arg in
        --clean|-c)
            clean=true
            ;;
        --with-cleanup|-w)
            with_cleanup=true
            ;;
        --quiet|-q)
            quiet=true
            ;;
        --no-spin|-n)
            no_spin=true
            ;;
        --help|-h|help)
            print_help "$0"
            ;;
        bdump|basm|belle|btils)
            targets+=("$arg")
            ;;
        --loud|-l)
            loud=true
            ;;
        --nodisplay|-N)
            nodisplay=true
            ;;
    esac
done

if [ ${#targets[@]} -eq 0 ]; then
    targets=(bdump basm belle btils)
fi

default_build
