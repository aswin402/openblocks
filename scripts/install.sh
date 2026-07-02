#!/bin/bash
set -e

# OpenBlocks installer script
# Downloads the pre-compiled binary for your system and installs it.

REPO="aswin402/openblocks"
BINARY_NAME="openblocks"
INSTALL_DIR="/usr/local/bin"
ALT_INSTALL_DIR="$HOME/.local/bin"

# Detect OS
OS="$(uname -s)"
case "${OS}" in
    Linux*)     PLATFORM=linux;;
    Darwin*)    PLATFORM=macos;;
    *)          echo "Unsupported OS: ${OS}"; exit 1;;
esac

# Detect Architecture
ARCH="$(uname -m)"
case "${ARCH}" in
    x86_64*)    TARGET_ARCH=x86_64;;
    arm64*|aarch64*)  TARGET_ARCH=aarch64;;
    *)          echo "Unsupported architecture: ${ARCH}"; exit 1;;
esac

# Determine target triple
if [ "$PLATFORM" = "linux" ]; then
    if [ "$TARGET_ARCH" = "x86_64" ]; then
        TARGET="x86_64-unknown-linux-gnu"
    else
        TARGET="aarch64-unknown-linux-gnu"
    fi
else # macos
    if [ "$TARGET_ARCH" = "x86_64" ]; then
        TARGET="x86_64-apple-darwin"
    else
        TARGET="aarch64-apple-darwin"
    fi
fi

echo "Detected platform: ${OS} (${ARCH})"
echo "Matching target: ${TARGET}"

# Fetch latest release metadata from GitHub
echo "Fetching latest release information..."
RELEASE_JSON=$(curl -s "https://api.github.com/repos/${REPO}/releases/latest")

# Get download URL for the tarball matching the target
DOWNLOAD_URL=$(echo "${RELEASE_JSON}" | grep -o 'https://github.com/.*\.tar\.gz' | grep "${TARGET}" | head -n 1)

if [ -z "$DOWNLOAD_URL" ]; then
    echo "Error: Could not find release asset for target ${TARGET}."
    echo "Check latest releases at https://github.com/${REPO}/releases"
    exit 1
fi

VERSION=$(echo "${RELEASE_JSON}" | grep -o '"tag_name": "[^"]*' | grep -o '[^"]*$')
echo "Downloading OpenBlocks ${VERSION} for ${TARGET}..."

# Create temp dir
TEMP_DIR=$(mktemp -d)
trap 'rm -rf "${TEMP_DIR}"' EXIT

# Download and unpack
curl -L "${DOWNLOAD_URL}" -o "${TEMP_DIR}/openblocks.tar.gz"
tar -xzf "${TEMP_DIR}/openblocks.tar.gz" -C "${TEMP_DIR}"

# Determine installation directory
if [ -w "${INSTALL_DIR}" ]; then
    DEST_DIR="${INSTALL_DIR}"
else
    echo "No write permission for ${INSTALL_DIR}, trying ${ALT_INSTALL_DIR}..."
    mkdir -p "${ALT_INSTALL_DIR}"
    DEST_DIR="${ALT_INSTALL_DIR}"
fi

# Copy binary
mv "${TEMP_DIR}/${BINARY_NAME}" "${DEST_DIR}/${BINARY_NAME}"
chmod +x "${DEST_DIR}/${BINARY_NAME}"

echo "Successfully installed ${BINARY_NAME} to ${DEST_DIR}/${BINARY_NAME}"

# Verify installation
if ! command -v "${BINARY_NAME}" &> /dev/null; then
    if [ "${DEST_DIR}" = "${ALT_INSTALL_DIR}" ]; then
        echo "Warning: ${ALT_INSTALL_DIR} is not in your PATH."
        echo "Please add it to your shell configuration (e.g. ~/.bashrc or ~/.zshrc):"
        echo 'export PATH="$HOME/.local/bin:$PATH"'
    fi
fi

# Run seed command to initialize the database
echo "Initializing database..."
if command -v "${BINARY_NAME}" &> /dev/null; then
    "${BINARY_NAME}" seed
else
    "${DEST_DIR}/${BINARY_NAME}" seed
fi

echo "OpenBlocks has been successfully installed and initialized! 🎉"
