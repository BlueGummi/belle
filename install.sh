#!/usr/bin/env bash
set -e
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
cd "$SCRIPT_DIR"
DIR="bin"
FILE1="basm"
FILE2="bdump"
FILE3="belle"
FILE4="bfmt"
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
        echo "$message"
    fi
}

install() {
    print_message "Installing..." blue
    mkdir -p ~/.local/bin
    cp "$DIR/$FILE1" ~/.local/bin
    cp "$DIR/$FILE2" ~/.local/bin
    cp "$DIR/$FILE3" ~/.local/bin
    cp "$DIR/$FILE4" ~/.local/bin
    print_message "Installation complete." green

    if ! echo "$PATH" | grep -q "$HOME/.local/bin"; then
        print_message "Updating PATH to include ~/.local/bin"

        case "$SHELL" in
            */bash)
                echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
                print_message "Please run 'source ~/.bashrc' or restart your terminal to apply changes." yellow
                ;;
            */zsh)
                echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.zshrc
                print_message "Please run 'source ~/.zshrc' or restart your terminal to apply changes." yellow
                ;;
            */fish)
                echo 'set -gx PATH $HOME/.local/bin $PATH' >> ~/.config/fish/config.fish
                print_message "Please run 'source ~/.config/fish/config.fish' or restart your terminal to apply changes." yellow
                ;;
            *)
                print_message "Please manually add ~/.local/bin to your PATH." yellow
                ;;
        esac
    fi

    if [ "$cleanup" ]; then
        print_message "Deleting '$DIR'..." yellow
        rm -rf "$DIR"
    fi
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

    printf "The install script for the BELLE programs and utilities\n\n"
    printf "${underline}Usage${reset}: $1 [OPTIONS]\n"
    printf "Options:\n"
    printf "  -c, --cleanup        Clean the binary directory\n"
    printf "  -h, --help           Display this help message\n"
    exit 0
}

for arg in "$@"; do
    case $arg in
        --cleanup|-c)
            cleanup=true
            ;;
        --help|-h|help)
            print_help "$0"
            ;;
    esac
done

BUILD=false

if [ ! -d "$DIR" ]; then
    print_message "Directory '$DIR' does not exist." red
    BUILD=true
else
    FILE1_PATH="$DIR/$FILE1"
    FILE2_PATH="$DIR/$FILE2"
    FILE3_PATH="$DIR/$FILE3"
    FILE4_PATH="$DIR/$FILE4"

    if [ ! -f "$FILE1_PATH" ] && [ ! -f "$FILE2_PATH" ] && [ ! -f "$FILE3_PATH" ]; then
        print_message "All binaries '$FILE1', '$FILE2', and '$FILE3' do not exist in '$DIR'." red
        BUILD=true
    elif [ ! -f "$FILE1_PATH" ]; then
        print_message "Binary '$FILE1' does not exist in '$DIR'." red
        BUILD=true
    elif [ ! -f "$FILE2_PATH" ]; then
        print_message "Binary '$FILE2' does not exist in '$DIR'." red
        BUILD=true
    elif [ ! -f "$FILE3_PATH" ]; then
        print_message "Binary '$FILE3' does not exist in '$DIR'." red
        BUILD=true
    elif [ ! -f "$FILE4_PATH" ]; then
        print_message "Binary '$FILE4' does not exist in '$DIR'." red
        BUILD=true
    fi
fi

if [ "$BUILD" = true ]; then
    read -p "Do you want to build BELLE to create the binaries? [Y/n]: " ANSWER
    ANSWER=${ANSWER:-Y}

    if [[ "$ANSWER" =~ ^[Yy]$ ]]; then
        if [ ! -f "./build.sh" ]; then
            print_message "Build script 'build.sh' not found." red
            exit 1
        fi

        print_message "Building..." blue
        ./build.sh
        print_message "Build successful. Proceeding to installation..." green
        install
    else
        print_message "Exiting without installing." yellow
    fi
else
    install
fi
