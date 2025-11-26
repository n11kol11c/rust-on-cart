#!/usr/bin/env bash
set -e

unameOut="$(uname -s)"

case "${unameOut}" in
    Linux*)   platform=linux;;
    Darwin*)  platform=mac;;
    CYGWIN*|MINGW*|MSYS*) platform=windows;;
    *)       platform=unknown;;
esac

if [ "$platform" = "windows" ]; then
    curl -sSf https://win.rustup.rs/x86_64 -o rustup-init.exe
    ./rustup-init.exe -y
    export PATH="$HOME/.cargo/bin:$PATH"
else
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
fi

rustup update
cargo --version
rustc --version