#!/bin/bash
#
# Piemme Installer
# Usage: curl -fsSL https://raw.githubusercontent.com/cartesia-one/piemme/main/install.sh | bash
#
# Environment variables:
#   PIEMME_INSTALL_DIR - Installation directory (default: ~/.local/bin)
#   PIEMME_VERSION     - Specific version to install (default: latest)
#

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
REPO="cartesia-one/piemme"
BINARY_NAME="piemme"
INSTALL_DIR="${PIEMME_INSTALL_DIR:-$HOME/.local/bin}"

# Logging functions
info() {
    echo -e "${BLUE}==>${NC} $1"
}

success() {
    echo -e "${GREEN}==>${NC} $1"
}

warn() {
    echo -e "${YELLOW}Warning:${NC} $1"
}

error() {
    echo -e "${RED}Error:${NC} $1" >&2
    exit 1
}

# Detect OS and architecture
detect_platform() {
    OS="$(uname -s)"
    ARCH="$(uname -m)"
    
    case "$OS" in
        Linux)
            OS="linux"
            ;;
        Darwin)
            OS="macos"
            ;;
        MINGW*|MSYS*|CYGWIN*)
            OS="windows"
            ;;
        *)
            error "Unsupported operating system: $OS"
            ;;
    esac
    
    case "$ARCH" in
        x86_64|amd64)
            ARCH="x86_64"
            ;;
        aarch64|arm64)
            ARCH="aarch64"
            ;;
        *)
            error "Unsupported architecture: $ARCH"
            ;;
    esac
    
    PLATFORM="${OS}-${ARCH}"
    info "Detected platform: $PLATFORM"
}

# Get the latest release version
get_latest_version() {
    if [ -n "$PIEMME_VERSION" ]; then
        VERSION="$PIEMME_VERSION"
        info "Using specified version: $VERSION"
    else
        info "Fetching latest release..."
        VERSION=$(curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')
        
        if [ -z "$VERSION" ]; then
            error "Failed to fetch latest version. Please check your internet connection or specify PIEMME_VERSION."
        fi
        
        info "Latest version: $VERSION"
    fi
}

# Download and install the binary
download_and_install() {
    ASSET_NAME="${BINARY_NAME}-${PLATFORM}"
    
    if [ "$OS" = "windows" ]; then
        ASSET_NAME="${ASSET_NAME}.exe"
        BINARY_NAME="${BINARY_NAME}.exe"
    fi
    
    DOWNLOAD_URL="https://github.com/${REPO}/releases/download/${VERSION}/${ASSET_NAME}"
    
    info "Downloading from: $DOWNLOAD_URL"
    
    # Create temp directory
    TMP_DIR=$(mktemp -d)
    trap "rm -rf $TMP_DIR" EXIT
    
    # Download binary
    if command -v curl &> /dev/null; then
        curl -fsSL "$DOWNLOAD_URL" -o "$TMP_DIR/$BINARY_NAME"
    elif command -v wget &> /dev/null; then
        wget -q "$DOWNLOAD_URL" -O "$TMP_DIR/$BINARY_NAME"
    else
        error "Neither curl nor wget found. Please install one of them."
    fi
    
    # Create install directory if needed
    mkdir -p "$INSTALL_DIR"
    
    # Install binary
    info "Installing to: $INSTALL_DIR/$BINARY_NAME"
    mv "$TMP_DIR/$BINARY_NAME" "$INSTALL_DIR/$BINARY_NAME"
    chmod +x "$INSTALL_DIR/$BINARY_NAME"
    
    success "Successfully installed piemme!"
}

# Check if install directory is in PATH
check_path() {
    if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
        echo ""
        warn "Installation directory is not in your PATH!"
        echo ""
        echo "Add the following to your shell configuration file:"
        echo ""
        
        SHELL_NAME=$(basename "$SHELL")
        case "$SHELL_NAME" in
            bash)
                echo "  echo 'export PATH=\"\$HOME/.local/bin:\$PATH\"' >> ~/.bashrc"
                echo "  source ~/.bashrc"
                ;;
            zsh)
                echo "  echo 'export PATH=\"\$HOME/.local/bin:\$PATH\"' >> ~/.zshrc"
                echo "  source ~/.zshrc"
                ;;
            fish)
                echo "  fish_add_path $INSTALL_DIR"
                ;;
            *)
                echo "  export PATH=\"$INSTALL_DIR:\$PATH\""
                ;;
        esac
        echo ""
    fi
}

# Print success message
print_success() {
    echo ""
    echo "╔════════════════════════════════════════════════════════════╗"
    echo "║                                                            ║"
    echo "║   🎉 piemme has been installed successfully!               ║"
    echo "║                                                            ║"
    echo "║   Run 'piemme' in any directory to start managing          ║"
    echo "║   your prompts.                                            ║"
    echo "║                                                            ║"
    echo "║   Documentation: https://github.com/${REPO}                ║"
    echo "║                                                            ║"
    echo "╚════════════════════════════════════════════════════════════╝"
    echo ""
}

# Main installation flow
main() {
    echo ""
    echo "  ╔═══════════════════════════════════════╗"
    echo "  ║     Piemme Installer                  ║"
    echo "  ║     TUI Prompt Manager                ║"
    echo "  ╚═══════════════════════════════════════╝"
    echo ""
    
    detect_platform
    get_latest_version
    download_and_install
    check_path
    print_success
}

main "$@"
