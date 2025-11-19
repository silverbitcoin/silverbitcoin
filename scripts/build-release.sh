#!/bin/bash
# SilverBitcoin Release Build Script
# Builds optimized release binaries for multiple platforms

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Configuration
VERSION=${VERSION:-"0.1.0"}
BUILD_DIR="target/release"
DIST_DIR="dist"
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

echo -e "${GREEN}SilverBitcoin Release Builder${NC}"
echo -e "${GREEN}Version: ${VERSION}${NC}"
echo ""

# Detect platform
detect_platform() {
    local os=$(uname -s)
    local arch=$(uname -m)
    
    case "$os" in
        Linux*)
            case "$arch" in
                x86_64) echo "linux-x86_64" ;;
                aarch64|arm64) echo "linux-arm64" ;;
                *) echo "unknown" ;;
            esac
            ;;
        Darwin*)
            case "$arch" in
                x86_64) echo "macos-intel" ;;
                arm64) echo "macos-apple-silicon" ;;
                *) echo "unknown" ;;
            esac
            ;;
        *)
            echo "unknown"
            ;;
    esac
}

PLATFORM=$(detect_platform)
echo -e "${YELLOW}Building for platform: ${PLATFORM}${NC}"

if [ "$PLATFORM" = "unknown" ]; then
    echo -e "${RED}Error: Unsupported platform${NC}"
    exit 1
fi

# Clean previous builds
echo -e "${YELLOW}Cleaning previous builds...${NC}"
rm -rf "$DIST_DIR"
mkdir -p "$DIST_DIR"

# Build release binaries
echo -e "${YELLOW}Building release binaries...${NC}"
cd "$PROJECT_ROOT"

# Set optimization flags
export RUSTFLAGS="-C target-cpu=native -C opt-level=3"

# Build all binaries
cargo build --release --bins

# Check if build succeeded
if [ $? -ne 0 ]; then
    echo -e "${RED}Build failed!${NC}"
    exit 1
fi

echo -e "${GREEN}Build completed successfully!${NC}"

# Create distribution directory structure
DIST_PLATFORM_DIR="$DIST_DIR/silverbitcoin-$VERSION-$PLATFORM"
mkdir -p "$DIST_PLATFORM_DIR/bin"
mkdir -p "$DIST_PLATFORM_DIR/config"
mkdir -p "$DIST_PLATFORM_DIR/docs"

# Copy binaries
echo -e "${YELLOW}Copying binaries...${NC}"
cp "$BUILD_DIR/silver-node" "$DIST_PLATFORM_DIR/bin/" 2>/dev/null || true
cp "$BUILD_DIR/silver-cli" "$DIST_PLATFORM_DIR/bin/" 2>/dev/null || true
cp "$BUILD_DIR/silver-indexer" "$DIST_PLATFORM_DIR/bin/" 2>/dev/null || true
cp "$BUILD_DIR/quantum-cli" "$DIST_PLATFORM_DIR/bin/" 2>/dev/null || true

# Strip binaries to reduce size
echo -e "${YELLOW}Stripping debug symbols...${NC}"
strip "$DIST_PLATFORM_DIR/bin/"* 2>/dev/null || true

# Copy configuration examples
echo -e "${YELLOW}Copying configuration files...${NC}"
cp node.toml.example "$DIST_PLATFORM_DIR/config/"
cp node-with-metrics.toml.example "$DIST_PLATFORM_DIR/config/"
cp genesis.json.example "$DIST_PLATFORM_DIR/config/"

# Copy documentation
echo -e "${YELLOW}Copying documentation...${NC}"
cp README.md "$DIST_PLATFORM_DIR/"
cp LICENSE "$DIST_PLATFORM_DIR/"
cp CONTRIBUTING.md "$DIST_PLATFORM_DIR/"
cp -r docs/* "$DIST_PLATFORM_DIR/docs/" 2>/dev/null || true

# Create installation script
cat > "$DIST_PLATFORM_DIR/install.sh" << 'EOF'
#!/bin/bash
# SilverBitcoin Installation Script

set -e

PREFIX=${PREFIX:-"/usr/local"}
BIN_DIR="$PREFIX/bin"
CONFIG_DIR="$PREFIX/etc/silverbitcoin"

echo "Installing SilverBitcoin..."
echo "Installation prefix: $PREFIX"

# Check for root privileges
if [ "$EUID" -ne 0 ] && [ ! -w "$BIN_DIR" ]; then
    echo "Error: Installation requires root privileges or write access to $BIN_DIR"
    echo "Try running with sudo or set PREFIX to a writable location:"
    echo "  PREFIX=~/.local ./install.sh"
    exit 1
fi

# Create directories
mkdir -p "$BIN_DIR"
mkdir -p "$CONFIG_DIR"

# Install binaries
echo "Installing binaries to $BIN_DIR..."
cp bin/* "$BIN_DIR/"
chmod +x "$BIN_DIR"/silver-*
chmod +x "$BIN_DIR"/quantum-* 2>/dev/null || true

# Install configuration examples
echo "Installing configuration examples to $CONFIG_DIR..."
cp config/*.example "$CONFIG_DIR/" 2>/dev/null || true

echo ""
echo "Installation complete!"
echo ""
echo "Binaries installed to: $BIN_DIR"
echo "Configuration examples: $CONFIG_DIR"
echo ""
echo "To get started:"
echo "  1. Copy and edit configuration: cp $CONFIG_DIR/node.toml.example ~/.silverbitcoin/node.toml"
echo "  2. Generate keys: silver-cli keygen"
echo "  3. Start node: silver-node --config ~/.silverbitcoin/node.toml"
echo ""
echo "For more information, visit: https://docs.silverbitcoin.org"
EOF

chmod +x "$DIST_PLATFORM_DIR/install.sh"

# Create uninstall script
cat > "$DIST_PLATFORM_DIR/uninstall.sh" << 'EOF'
#!/bin/bash
# SilverBitcoin Uninstallation Script

set -e

PREFIX=${PREFIX:-"/usr/local"}
BIN_DIR="$PREFIX/bin"
CONFIG_DIR="$PREFIX/etc/silverbitcoin"

echo "Uninstalling SilverBitcoin..."

# Check for root privileges
if [ "$EUID" -ne 0 ] && [ ! -w "$BIN_DIR" ]; then
    echo "Error: Uninstallation requires root privileges"
    echo "Try running with sudo"
    exit 1
fi

# Remove binaries
echo "Removing binaries from $BIN_DIR..."
rm -f "$BIN_DIR/silver-node"
rm -f "$BIN_DIR/silver-cli"
rm -f "$BIN_DIR/silver-indexer"
rm -f "$BIN_DIR/quantum-cli"

# Remove configuration directory
echo "Removing configuration directory $CONFIG_DIR..."
rm -rf "$CONFIG_DIR"

echo "Uninstallation complete!"
echo ""
echo "Note: User data in ~/.silverbitcoin was not removed."
echo "To remove user data: rm -rf ~/.silverbitcoin"
EOF

chmod +x "$DIST_PLATFORM_DIR/uninstall.sh"

# Create README for distribution
cat > "$DIST_PLATFORM_DIR/README.txt" << EOF
SilverBitcoin v${VERSION}
Platform: ${PLATFORM}

INSTALLATION
============

Quick Install (requires root/sudo):
  sudo ./install.sh

Custom Install Location:
  PREFIX=~/.local ./install.sh

CONTENTS
========

bin/
  - silver-node: Blockchain node (validator or full node)
  - silver-cli: Command-line interface
  - silver-indexer: Blockchain indexer
  - quantum-cli: Quantum smart contract tools

config/
  - node.toml.example: Node configuration template
  - node-with-metrics.toml.example: Node with metrics enabled
  - genesis.json.example: Genesis configuration template

docs/
  - Complete documentation

QUICK START
===========

1. Install binaries:
   sudo ./install.sh

2. Generate keys:
   silver-cli keygen

3. Create configuration:
   mkdir -p ~/.silverbitcoin
   cp config/node.toml.example ~/.silverbitcoin/node.toml
   # Edit ~/.silverbitcoin/node.toml as needed

4. Start node:
   silver-node --config ~/.silverbitcoin/node.toml

DOCUMENTATION
=============

Full documentation: https://docs.silverbitcoin.org
GitHub: https://github.com/silverbitcoin/silverbitcoin
Discord: https://discord.gg/silverbitcoin

SYSTEM REQUIREMENTS
===================

Validator Node:
  - CPU: 8+ cores
  - RAM: 16GB minimum, 32GB recommended
  - Disk: 500GB SSD (NVMe recommended)
  - Network: 1Gbps

Full Node:
  - CPU: 4+ cores
  - RAM: 8GB minimum, 16GB recommended
  - Disk: 500GB SSD
  - Network: 100Mbps

LICENSE
=======

Apache License 2.0
See LICENSE file for details.

SUPPORT
=======

For issues and questions:
  - GitHub Issues: https://github.com/silverbitcoin/silverbitcoin/issues
  - Discord: https://discord.gg/silverbitcoin
  - Email: support@silverbitcoin.org
EOF

# Create tarball
echo -e "${YELLOW}Creating distribution archive...${NC}"
cd "$DIST_DIR"
tar -czf "silverbitcoin-$VERSION-$PLATFORM.tar.gz" "silverbitcoin-$VERSION-$PLATFORM"

# Create checksum
echo -e "${YELLOW}Generating checksums...${NC}"
if command -v sha256sum &> /dev/null; then
    sha256sum "silverbitcoin-$VERSION-$PLATFORM.tar.gz" > "silverbitcoin-$VERSION-$PLATFORM.tar.gz.sha256"
elif command -v shasum &> /dev/null; then
    shasum -a 256 "silverbitcoin-$VERSION-$PLATFORM.tar.gz" > "silverbitcoin-$VERSION-$PLATFORM.tar.gz.sha256"
fi

cd "$PROJECT_ROOT"

# Print summary
echo ""
echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}Build Summary${NC}"
echo -e "${GREEN}========================================${NC}"
echo -e "Version:     ${VERSION}"
echo -e "Platform:    ${PLATFORM}"
echo -e "Archive:     ${DIST_DIR}/silverbitcoin-${VERSION}-${PLATFORM}.tar.gz"
echo -e "Size:        $(du -h "${DIST_DIR}/silverbitcoin-${VERSION}-${PLATFORM}.tar.gz" | cut -f1)"
echo ""
echo -e "${GREEN}Binaries included:${NC}"
ls -lh "$DIST_PLATFORM_DIR/bin/"
echo ""
echo -e "${GREEN}Build completed successfully!${NC}"
