#!/bin/bash
# SilverBitcoin Cross-Platform Build Script
# Builds release binaries for multiple platforms using cross-compilation

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

VERSION=${VERSION:-"0.1.0"}
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

echo -e "${GREEN}SilverBitcoin Cross-Platform Builder${NC}"
echo -e "${GREEN}Version: ${VERSION}${NC}"
echo ""

# Check if cross is installed
if ! command -v cross &> /dev/null; then
    echo -e "${YELLOW}Installing cross for cross-compilation...${NC}"
    cargo install cross --git https://github.com/cross-rs/cross
fi

# Target platforms
TARGETS=(
    "x86_64-unknown-linux-gnu:linux-x86_64"
    "aarch64-unknown-linux-gnu:linux-arm64"
    "x86_64-apple-darwin:macos-intel"
    "aarch64-apple-darwin:macos-apple-silicon"
)

cd "$PROJECT_ROOT"

# Build for each target
for target_pair in "${TARGETS[@]}"; do
    IFS=':' read -r target platform <<< "$target_pair"
    
    echo ""
    echo -e "${GREEN}========================================${NC}"
    echo -e "${GREEN}Building for ${platform} (${target})${NC}"
    echo -e "${GREEN}========================================${NC}"
    
    # Skip macOS builds on Linux (requires macOS SDK)
    if [[ "$target" == *"apple-darwin"* ]] && [[ "$(uname -s)" != "Darwin" ]]; then
        echo -e "${YELLOW}Skipping ${platform} (requires macOS host or SDK)${NC}"
        continue
    fi
    
    # Build using cross
    if [[ "$target" == *"apple-darwin"* ]]; then
        # Use cargo for native macOS builds
        cargo build --release --target "$target" --bins
    else
        # Use cross for Linux cross-compilation
        cross build --release --target "$target" --bins
    fi
    
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}Build successful for ${platform}${NC}"
        
        # Create distribution
        DIST_DIR="dist/silverbitcoin-$VERSION-$platform"
        mkdir -p "$DIST_DIR/bin"
        mkdir -p "$DIST_DIR/config"
        mkdir -p "$DIST_DIR/docs"
        
        # Copy binaries
        BUILD_DIR="target/$target/release"
        cp "$BUILD_DIR/silver-node" "$DIST_DIR/bin/" 2>/dev/null || true
        cp "$BUILD_DIR/silver-cli" "$DIST_DIR/bin/" 2>/dev/null || true
        cp "$BUILD_DIR/silver-indexer" "$DIST_DIR/bin/" 2>/dev/null || true
        cp "$BUILD_DIR/quantum-cli" "$DIST_DIR/bin/" 2>/dev/null || true
        
        # Strip binaries (if not macOS)
        if [[ "$target" != *"apple-darwin"* ]]; then
            strip "$DIST_DIR/bin/"* 2>/dev/null || true
        fi
        
        # Copy configs and docs
        cp node.toml.example "$DIST_DIR/config/"
        cp node-with-metrics.toml.example "$DIST_DIR/config/"
        cp genesis.json.example "$DIST_DIR/config/"
        cp README.md LICENSE CONTRIBUTING.md "$DIST_DIR/"
        cp -r docs/* "$DIST_DIR/docs/" 2>/dev/null || true
        
        # Create archive
        cd dist
        tar -czf "silverbitcoin-$VERSION-$platform.tar.gz" "silverbitcoin-$VERSION-$platform"
        
        # Create checksum
        if command -v sha256sum &> /dev/null; then
            sha256sum "silverbitcoin-$VERSION-$platform.tar.gz" > "silverbitcoin-$VERSION-$platform.tar.gz.sha256"
        elif command -v shasum &> /dev/null; then
            shasum -a 256 "silverbitcoin-$VERSION-$platform.tar.gz" > "silverbitcoin-$VERSION-$platform.tar.gz.sha256"
        fi
        
        cd "$PROJECT_ROOT"
        
        echo -e "${GREEN}Distribution created: dist/silverbitcoin-$VERSION-$platform.tar.gz${NC}"
    else
        echo -e "${RED}Build failed for ${platform}${NC}"
    fi
done

# Create checksums file for all builds
echo ""
echo -e "${YELLOW}Creating combined checksums file...${NC}"
cd dist
cat *.sha256 > "silverbitcoin-$VERSION-checksums.txt" 2>/dev/null || true
cd "$PROJECT_ROOT"

# Print summary
echo ""
echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}Cross-Platform Build Summary${NC}"
echo -e "${GREEN}========================================${NC}"
echo -e "Version: ${VERSION}"
echo ""
echo -e "${GREEN}Built distributions:${NC}"
ls -lh dist/*.tar.gz 2>/dev/null || echo "No distributions created"
echo ""
echo -e "${GREEN}Total size:${NC}"
du -sh dist/ 2>/dev/null || echo "N/A"
echo ""
echo -e "${GREEN}Build completed!${NC}"
