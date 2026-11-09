#!/usr/bin/env sh
# Hit Official Installer
# https://hit-scm.com
#
# Usage:
#   curl -fsSL https://hit-scm.com/install.sh | sh

set -e

# Terminal colors
BOLD="$(tput bold 2>/dev/null || printf '')"
GREEN="$(tput setaf 2 2>/dev/null || printf '')"
CYAN="$(tput setaf 6 2>/dev/null || printf '')"
YELLOW="$(tput setaf 3 2>/dev/null || printf '')"
RED="$(tput setaf 1 2>/dev/null || printf '')"
DIM="$(tput setaf 8 2>/dev/null || printf '')"
RESET="$(tput sgr0 2>/dev/null || printf '')"

printf "%b" "${BOLD}${CYAN}
   __    __   __  .___________.
  |  |  |  | |  | |           |
  |  |__|  | |  | \`---|  |----\`
  |   __   | |  |     |  |     
  |  |  |  | |  |     |  |     
  |__|  |__| |__|     |__|     
${RESET}"
printf "%b\n" "${BOLD}Hit - High-Performance Source Control System${RESET}"
printf "%b\n\n" "${DIM}https://hit-scm.com${RESET}"

# Detect OS and architecture
OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS" in
  Linux)
    PLATFORM="linux"
    ;;
  Darwin)
    PLATFORM="macos"
    ;;
  *)
    printf "%b\n" "${RED}✖ Unsupported operating system: $OS${RESET}"
    exit 1
    ;;
esac

case "$ARCH" in
  x86_64|amd64)
    TARGET_ARCH="x86_64"
    ;;
  arm64|aarch64)
    TARGET_ARCH="aarch64"
    ;;
  *)
    printf "%b\n" "${RED}✖ Unsupported architecture: $ARCH${RESET}"
    exit 1
    ;;
esac

TARGET="${TARGET_ARCH}-${PLATFORM}"
printf "%b\n" "Detected platform: ${BOLD}${TARGET}${RESET}"

# Determine installation directory
if [ -z "$HIT_INSTALL_DIR" ]; then
  if [ -w "/usr/local/bin" ]; then
    INSTALL_DIR="/usr/local/bin"
  else
    INSTALL_DIR="$HOME/.local/bin"
    mkdir -p "$INSTALL_DIR"
  fi
else
  INSTALL_DIR="$HIT_INSTALL_DIR"
fi

printf "%b\n" "Installing to: ${BOLD}${INSTALL_DIR}${RESET}"

# Release download URL
RELEASE_TAG="latest"
DOWNLOAD_URL="https://github.com/hit-scm/hit/releases/${RELEASE_TAG}/download/hit-${TARGET}.tar.gz"

TMP_DIR="$(mktemp -d 2>/dev/null || mktemp -d -t 'hit-install')"
trap 'rm -rf "$TMP_DIR"' EXIT

printf "%b\n" "Downloading Hit binary from ${CYAN}${DOWNLOAD_URL}${RESET}..."

# If running locally from git repository build, support local install
if [ -f "./target/release/hit" ]; then
  printf "%b\n" "${DIM}Found local release binary, installing directly...${RESET}"
  cp "./target/release/hit" "$INSTALL_DIR/hit"
  chmod +x "$INSTALL_DIR/hit"
else
  if command -v curl >/dev/null 2>&1; then
    curl -fsSL "$DOWNLOAD_URL" -o "$TMP_DIR/hit.tar.gz" || {
      # Fallback to cargo if release artifact is not yet online
      printf "%b\n" "${YELLOW}Warning: Prebuilt binary not reachable, falling back to Cargo build...${RESET}"
      cargo install hit --root "${INSTALL_DIR}/.."
      exit 0
    }
  elif command -v wget >/dev/null 2>&1; then
    wget -qO "$TMP_DIR/hit.tar.gz" "$DOWNLOAD_URL"
  else
    printf "%b\n" "${RED}✖ Neither curl nor wget was found. Please install one to continue.${RESET}"
    exit 1
  fi

  tar -xzf "$TMP_DIR/hit.tar.gz" -C "$TMP_DIR"
  cp "$TMP_DIR/hit" "$INSTALL_DIR/hit"
  chmod +x "$INSTALL_DIR/hit"
fi

printf "\n%b\n" "${GREEN}${BOLD}✔ Hit installed successfully!${RESET}"

# Check if INSTALL_DIR is in PATH
case ":$PATH:" in
  *":$INSTALL_DIR:"*) ;;
  *)
    printf "\n%b\n" "${YELLOW}Note: ${INSTALL_DIR} is not currently in your \$PATH.${RESET}"
    printf "%b\n" "Add it to your shell configuration by running:"
    printf "%b\n\n" "  ${BOLD}export PATH=\"\$PATH:${INSTALL_DIR}\"${RESET}"
    ;;
esac

printf "%b\n" "Get started:"
printf "%b\n" "  ${BOLD}hit init${RESET}"
printf "%b\n" "  ${BOLD}hit --help${RESET}"
printf "%b\n\n" "Documentation & guides: ${CYAN}https://hit-scm.com${RESET}"

