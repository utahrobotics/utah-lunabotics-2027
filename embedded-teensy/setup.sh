#!/usr/bin/env bash
# One-time setup for building and flashing embedded-teensy (Teensy 4.1).
# Linux only, and only guaranteed on Ubuntu.
# Safe to re-run: each step skips what's already installed.
#
# Installs:
#   - the Rust toolchain pinned in rust-toolchain.toml, with the thumbv7em-none-eabihf
#     target (the Teensy's Cortex-M7) and llvm-tools (which rust-objcopy uses)
#   - cargo-binutils, for rust-objcopy (turns the build into a HEX file)
#   - defmt-print, which decodes the firmware's logs
#   - teensy_loader_cli, PJRC's command-line loader
#   - PJRC's udev rules, so the loader can reach the board without sudo
# and adds you to the dialout group, so you can read the log adapter without sudo.
set -euo pipefail

if [ "$(uname -s)" != Linux ]; then
    echo "This script only supports Linux (and is only guaranteed on Ubuntu)." >&2
    exit 1
fi
if ! grep -qx 'ID=ubuntu' /etc/os-release 2>/dev/null; then
    echo "Warning: this script is only guaranteed on Ubuntu. Continuing anyway." >&2
fi

# rustup picks its toolchain from the current directory, so run from here:
# that way it uses the version pinned in this folder's rust-toolchain.toml.
cd "$(dirname "$0")"

step() { printf '\n==> %s\n' "$*"; }
have() { command -v "$1" >/dev/null 2>&1; }

if ! have rustup; then
    echo "rustup not found. Install Rust from https://rustup.rs, then re-run this." >&2
    exit 1
fi

step "Rust toolchain, Teensy target and llvm-tools"
rustup toolchain install # with no name, installs the pinned toolchain if it's missing
rustup target add thumbv7em-none-eabihf
rustup component add llvm-tools
echo "Using $(rustup show active-toolchain | cut -d' ' -f1)"

step "cargo-binutils (rust-objcopy)"
if have rust-objcopy; then
    echo "Already installed."
else
    cargo install cargo-binutils
fi

step "defmt-print (decodes the firmware's logs)"
if have defmt-print; then
    echo "Already installed."
else
    cargo install defmt-print
fi

step "teensy_loader_cli"
if have teensy_loader_cli; then
    echo "Already installed."
elif have apt-get; then
    sudo apt-get update
    sudo apt-get install -y teensy-loader-cli
else
    echo "No apt-get here. Install teensy_loader_cli by hand, then re-run this:" >&2
    echo "  https://github.com/PaulStoffregen/teensy_loader_cli" >&2
    exit 1
fi

step "udev rules (let the loader reach the Teensy without sudo)"
rules=/etc/udev/rules.d/00-teensy.rules
url=https://www.pjrc.com/teensy/00-teensy.rules
if [ -f "$rules" ]; then
    echo "Already installed."
else
    if have curl; then
        sudo curl -fsSL -o "$rules" "$url"
    else
        sudo wget -qO "$rules" "$url"
    fi
    sudo udevadm control --reload-rules
    sudo udevadm trigger
    echo "Installed. If the Teensy is plugged in, unplug and replug it."
fi

step "Serial port access (the dialout group)"
if id -nG "$USER" | grep -qw dialout; then
    echo "Already in dialout."
else
    sudo usermod -aG dialout "$USER"
    echo "Added $USER to dialout. Log out and back in for it to take effect."
fi

step "Done"
echo "To build and flash, run this from embedded-teensy/:"
echo "  cargo run --release"
echo "then press the button on the Teensy when the loader says it's waiting."
echo "To read the logs, see the Logs section of README.md."
