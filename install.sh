#!/bin/sh
#
# ohcrab installer script for macOS and Linux
#
# This script is designed to be run via curl:
#   sh -c "$(curl -fsSL https://raw.githubusercontent.com/luizvbo/oh-crab/main/install.sh)"

set -e

# Define repository and binary name
REPO="luizvbo/ohcrab"
BINARY_NAME="ohcrab"

# Determine the Operating System and Architecture
get_os_arch() {
    OS_TYPE=$(uname -s | tr '[:upper:]' '[:lower:]')
    MACHINE_ARCH=$(uname -m)

    TARGET=""

    case "$OS_TYPE" in
        linux)
            case "$MACHINE_ARCH" in
                x86_64) TARGET="x86_64-unknown-linux-gnu" ;;
                aarch64) TARGET="aarch64-unknown-linux-gnu" ;;
                *) echo "Error: Unsupported architecture ($MACHINE_ARCH) for Linux." >&2; exit 1 ;;
            esac
            ;;
        darwin)
            case "$MACHINE_ARCH" in
                x86_64) TARGET="x86_64-apple-darwin" ;;
                arm64) TARGET="aarch64-apple-darwin" ;;
                *) echo "Error: Unsupported architecture ($MACHINE_ARCH) for macOS." >&2; exit 1 ;;
            esac
            ;;
        *)
            echo "Error: This installer script supports macOS and Linux only." >&2
            echo "For Windows, please download the binary from the GitHub Releases page." >&2
            exit 1
            ;;
    esac
    echo "$TARGET"
}

# Find a writable directory in PATH, prioritizing user-local bins
find_install_dir() {
    # Prefer user-local bin directories
    if [ -d "$HOME/.local/bin" ] && [ -w "$HOME/.local/bin" ]; then
        echo "$HOME/.local/bin"
        return
    fi
    if [ -d "$HOME/bin" ] && [ -w "$HOME/bin" ]; then
        echo "$HOME/bin"
        return
    fi
    
    # Fallback to system-wide directory (might require sudo)
    if [ -d "/usr/local/bin" ] && [ -w "/usr/local/bin" ]; then
        echo "/usr/local/bin"
        return
    fi

    echo ""
}

main() {
    TARGET=$(get_os_arch)
    echo "Detected target: $TARGET"

    LATEST_TAG=$(curl -s "https://api.github.com/repos/$REPO/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')
    if [ -z "$LATEST_TAG" ]; then
        echo "Error: Could not determine the latest release version." >&2
        exit 1
    fi
    echo "Latest version: $LATEST_TAG"

    # The asset name needs to match what the release workflow creates
    DOWNLOAD_URL="https://github.com/$REPO/releases/download/$LATEST_TAG/$BINARY_NAME-$TARGET"
    
    # Create a temporary directory for the download
    TMP_DIR=$(mktemp -d)
    # Ensure the temporary directory is cleaned up on exit
    trap 'rm -rf "$TMP_DIR"' EXIT

    echo "Downloading from: $DOWNLOAD_URL"
    curl -L --progress-bar "$DOWNLOAD_URL" -o "$TMP_DIR/$BINARY_NAME"
    chmod +x "$TMP_DIR/$BINARY_NAME"

    # Determine installation directory
    INSTALL_DIR=$(find_install_dir)

    if [ -n "$INSTALL_DIR" ]; then
        echo "Installing to $INSTALL_DIR..."
        mv "$TMP_DIR/$BINARY_NAME" "$INSTALL_DIR/$BINARY_NAME"
    else
        echo "Could not find a writable installation directory in your PATH." >&2
        echo "Attempting to install with sudo to /usr/local/bin..." >&2
        if [ ! -d "/usr/local/bin" ]; then
            sudo mkdir -p "/usr/local/bin"
        fi
        sudo mv "$TMP_DIR/$BINARY_NAME" "/usr/local/bin/$BINARY_NAME"
        INSTALL_DIR="/usr/local/bin"
    fi

    # Check if the installation directory is in the user's PATH
    case ":$PATH:" in
        *":$INSTALL_DIR:"*)
            # In PATH
            ;;
        *)
            # Not in PATH, print a warning
            echo ""
            echo "⚠️  Warning: The directory $INSTALL_DIR is not in your PATH."
            echo "You will need to add it to your shell's configuration file to run '$BINARY_NAME' directly."
            echo "For example, add the following line to your ~/.bashrc or ~/.zshrc:"
            echo "  export PATH=\"\$PATH:$INSTALL_DIR\""
            echo ""
            ;;
    esac

    echo "✅ $BINARY_NAME has been installed successfully to $INSTALL_DIR"
    echo "You can now run '$BINARY_NAME --help'"
}

main
