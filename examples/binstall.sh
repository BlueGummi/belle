#!/usr/bin/env bash

URL="https://github.com/BlueGummi/belle/releases/download/nightly/belle-nightly-linux-x86_64.tar.gz"
TAR_FILE="belle-nightly-linux-x86_64.tar.gz"
INSTALL_DIR="$HOME/.local/bin"

if [ ! -d "$INSTALL_DIR" ]; then
    mkdir -p "$INSTALL_DIR"
fi

curl -L -o "$TAR_FILE" "$URL"

tar -xzf "$TAR_FILE"

mv bin/basm "$INSTALL_DIR/basm"
mv bin/bdump "$INSTALL_DIR/bdump"
mv bin/belle "$INSTALL_DIR/belle"
mv bin/bfmt "$INSTALL_DIR/bfmt"

rm -rf belle-nightly
rm "$TAR_FILE"

if ! echo "$PATH" | grep -q "$INSTALL_DIR"; then
    if [[ "$SHELL" == *"bash"* ]]; then
        echo "export PATH=\"$INSTALL_DIR:\$PATH\"" >> ~/.bashrc
    elif [[ "$SHELL" == *"zsh"* ]]; then
        echo "export PATH=\"$INSTALL_DIR:\$PATH\"" >> ~/.zshrc
    elif [[ "$SHELL" == *"fish"* ]]; then
        echo "set -gx PATH \"$INSTALL_DIR\" \$PATH" >> ~/.config/fish/config.fish
    fi
    echo "Please restart your terminal or run 'source ~/.bashrc' or 'source ~/.zshrc' or 'source ~/.config/fish/config.fish' to refresh your PATH."
fi

mv -r bin/examples .
rm -rf bin
cd examples
ls
echo "Installed."
echo "Run 'make' in this directory to compile examples programs and execute them"
