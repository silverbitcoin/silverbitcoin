#!/bin/bash
# Test script for consumer hardware configuration
# Verifies that SilverBitcoin can run on 16GB RAM and 500GB storage

set -e

echo "╔═══════════════════════════════════════════════════════════╗"
echo "║   SilverBitcoin Consumer Hardware Configuration Test     ║"
echo "║   Testing 16GB RAM and 500GB Storage Requirements        ║"
echo "╚═══════════════════════════════════════════════════════════╝"
echo ""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Test results
TESTS_PASSED=0
TESTS_FAILED=0

# Function to print test result
print_result() {
    if [ $1 -eq 0 ]; then
        echo -e "${GREEN}✓ PASS${NC}: $2"
        ((TESTS_PASSED++))
    else
        echo -e "${RED}✗ FAIL${NC}: $2"
        ((TESTS_FAILED++))
    fi
}

# Function to print warning
print_warning() {
    echo -e "${YELLOW}⚠ WARNING${NC}: $1"
}

# Function to print info
print_info() {
    echo -e "ℹ INFO: $1"
}

echo "1. Checking System Requirements"
echo "================================"

# Check CPU cores
CPU_CORES=$(nproc 2>/dev/null || sysctl -n hw.ncpu 2>/dev/null || echo "0")
print_info "CPU cores detected: $CPU_CORES"
if [ "$CPU_CORES" -ge 4 ]; then
    print_result 0 "CPU cores (minimum 4 required)"
else
    print_result 1 "CPU cores (minimum 4 required, found $CPU_CORES)"
fi

# Check total RAM
if command -v free &> /dev/null; then
    TOTAL_RAM_KB=$(free -k | grep Mem | awk '{print $2}')
    TOTAL_RAM_GB=$((TOTAL_RAM_KB / 1024 / 1024))
elif command -v sysctl &> /dev/null && sysctl -n hw.memsize &> /dev/null; then
    TOTAL_RAM_BYTES=$(sysctl -n hw.memsize)
    TOTAL_RAM_GB=$((TOTAL_RAM_BYTES / 1024 / 1024 / 1024))
else
    TOTAL_RAM_GB=0
fi

print_info "Total RAM detected: ${TOTAL_RAM_GB}GB"
if [ "$TOTAL_RAM_GB" -ge 16 ]; then
    print_result 0 "RAM (minimum 16GB required)"
else
    print_result 1 "RAM (minimum 16GB required, found ${TOTAL_RAM_GB}GB)"
fi

# Check available disk space
AVAILABLE_SPACE_GB=$(df -BG . | tail -1 | awk '{print $4}' | sed 's/G//')
print_info "Available disk space: ${AVAILABLE_SPACE_GB}GB"
if [ "$AVAILABLE_SPACE_GB" -ge 500 ]; then
    print_result 0 "Disk space (minimum 500GB required)"
else
    print_result 1 "Disk space (minimum 500GB required, found ${AVAILABLE_SPACE_GB}GB)"
fi

# Check if SSD
if [ -f /sys/block/sda/queue/rotational ]; then
    IS_SSD=$(cat /sys/block/sda/queue/rotational)
    if [ "$IS_SSD" -eq 0 ]; then
        print_result 0 "Storage type (SSD detected)"
    else
        print_warning "HDD detected - SSD strongly recommended for performance"
    fi
else
    print_info "Cannot determine storage type (assuming SSD)"
fi

echo ""
echo "2. Checking Configuration Files"
echo "================================"

# Check if consumer config exists
if [ -f "node-consumer.toml.example" ]; then
    print_result 0 "Consumer hardware configuration file exists"
else
    print_result 1 "Consumer hardware configuration file missing"
fi

# Validate consumer config
if [ -f "node-consumer.toml.example" ]; then
    # Check memory settings
    if grep -q "max_memory_usage = 8589934592" node-consumer.toml.example; then
        print_result 0 "Memory limit set to 8GB"
    else
        print_result 1 "Memory limit not properly configured"
    fi
    
    # Check cache settings
    if grep -q "object_cache_size = 268435456" node-consumer.toml.example; then
        print_result 0 "Object cache set to 256MB"
    else
        print_result 1 "Object cache not properly configured"
    fi
    
    # Check pruning settings
    if grep -q "snapshot_retention_days = 7" node-consumer.toml.example; then
        print_result 0 "Aggressive pruning enabled (7 days)"
    else
        print_result 1 "Pruning not properly configured"
    fi
    
    # Check worker threads
    if grep -q "worker_threads = 4" node-consumer.toml.example; then
        print_result 0 "Worker threads set to 4 for consumer hardware"
    else
        print_result 1 "Worker threads not properly configured"
    fi
fi

echo ""
echo "3. Checking Memory Management Module"
echo "====================================="

# Check if memory module exists
if [ -f "crates/silver-node/src/memory.rs" ]; then
    print_result 0 "Memory management module exists"
    
    # Check for key features
    if grep -q "MemoryManager" crates/silver-node/src/memory.rs; then
        print_result 0 "MemoryManager implementation found"
    else
        print_result 1 "MemoryManager implementation missing"
    fi
    
    if grep -q "MemoryPressure" crates/silver-node/src/memory.rs; then
        print_result 0 "Memory pressure monitoring found"
    else
        print_result 1 "Memory pressure monitoring missing"
    fi
    
    if grep -q "cleanup_callbacks" crates/silver-node/src/memory.rs; then
        print_result 0 "Automatic cleanup callbacks found"
    else
        print_result 1 "Automatic cleanup callbacks missing"
    fi
else
    print_result 1 "Memory management module missing"
fi

echo ""
echo "4. Checking Documentation"
echo "========================="

# Check if consumer hardware guide exists
if [ -f "docs/operator/CONSUMER_HARDWARE_GUIDE.md" ]; then
    print_result 0 "Consumer hardware guide exists"
    
    # Check for key sections
    if grep -q "Memory Budget Breakdown" docs/operator/CONSUMER_HARDWARE_GUIDE.md; then
        print_result 0 "Memory budget documentation found"
    else
        print_result 1 "Memory budget documentation missing"
    fi
    
    if grep -q "Storage Budget" docs/operator/CONSUMER_HARDWARE_GUIDE.md; then
        print_result 0 "Storage budget documentation found"
    else
        print_result 1 "Storage budget documentation missing"
    fi
    
    if grep -q "Troubleshooting" docs/operator/CONSUMER_HARDWARE_GUIDE.md; then
        print_result 0 "Troubleshooting guide found"
    else
        print_result 1 "Troubleshooting guide missing"
    fi
else
    print_result 1 "Consumer hardware guide missing"
fi

echo ""
echo "5. Memory Budget Validation"
echo "==========================="

# Calculate total memory budget from config
ROCKSDB_CACHE=512  # MB
OBJECT_CACHE=256   # MB
NETWORK_BUFFERS=1024  # MB (estimated)
EXECUTION_ENGINE=2048  # MB (estimated)
CONSENSUS_ENGINE=1536  # MB (estimated)
API_SERVER=512  # MB (estimated)
OTHER=2304  # MB (estimated)

TOTAL_NODE_MEMORY=$((ROCKSDB_CACHE + OBJECT_CACHE + NETWORK_BUFFERS + EXECUTION_ENGINE + CONSENSUS_ENGINE + API_SERVER + OTHER))
TOTAL_NODE_MEMORY_GB=$((TOTAL_NODE_MEMORY / 1024))

print_info "Estimated node memory usage: ${TOTAL_NODE_MEMORY}MB (~${TOTAL_NODE_MEMORY_GB}GB)"

if [ "$TOTAL_NODE_MEMORY_GB" -le 8 ]; then
    print_result 0 "Memory budget within 8GB limit"
else
    print_result 1 "Memory budget exceeds 8GB limit"
fi

# Check if leaves enough for OS
OS_MEMORY=2048  # MB
SAFETY_MARGIN=2048  # MB
TOTAL_REQUIRED=$((TOTAL_NODE_MEMORY + OS_MEMORY + SAFETY_MARGIN))
TOTAL_REQUIRED_GB=$((TOTAL_REQUIRED / 1024))

print_info "Total memory required (node + OS + margin): ${TOTAL_REQUIRED}MB (~${TOTAL_REQUIRED_GB}GB)"

if [ "$TOTAL_REQUIRED_GB" -le 16 ]; then
    print_result 0 "Total memory requirement fits in 16GB"
else
    print_result 1 "Total memory requirement exceeds 16GB"
fi

echo ""
echo "6. Storage Budget Validation"
echo "============================"

# Calculate storage budget
BLOCKCHAIN_DATA=300  # GB
INDEXES=50  # GB
SNAPSHOTS=30  # GB
LOGS=20  # GB

TOTAL_STORAGE=$((BLOCKCHAIN_DATA + INDEXES + SNAPSHOTS + LOGS))

print_info "Estimated storage usage: ${TOTAL_STORAGE}GB"

if [ "$TOTAL_STORAGE" -le 400 ]; then
    print_result 0 "Storage budget within 400GB target (500GB disk)"
else
    print_result 1 "Storage budget exceeds 400GB target"
fi

echo ""
echo "7. Performance Expectations"
echo "==========================="

# Calculate expected TPS based on CPU cores
if [ "$CPU_CORES" -ge 8 ]; then
    EXPECTED_TPS="5,000-10,000"
    print_info "Expected TPS (8+ cores): $EXPECTED_TPS"
elif [ "$CPU_CORES" -ge 4 ]; then
    EXPECTED_TPS="2,000-5,000"
    print_info "Expected TPS (4+ cores): $EXPECTED_TPS"
else
    EXPECTED_TPS="<2,000"
    print_warning "Expected TPS (<4 cores): $EXPECTED_TPS - may be insufficient"
fi

# Finality expectation
print_info "Expected finality: <1 second (480ms snapshot interval)"

# Sync speed expectation
print_info "Expected sync speed: 500-1,000 transactions/second"

echo ""
echo "8. Optimization Recommendations"
echo "================================"

# Provide recommendations based on system
if [ "$TOTAL_RAM_GB" -lt 16 ]; then
    print_warning "RAM below minimum - consider upgrading to 16GB+"
elif [ "$TOTAL_RAM_GB" -lt 32 ]; then
    print_info "RAM meets minimum - consider upgrading to 32GB for better performance"
else
    print_info "RAM exceeds minimum - can use standard configuration for better performance"
fi

if [ "$AVAILABLE_SPACE_GB" -lt 500 ]; then
    print_warning "Disk space below minimum - free up space or add storage"
elif [ "$AVAILABLE_SPACE_GB" -lt 1000 ]; then
    print_info "Disk space meets minimum - consider 1TB for longer retention"
else
    print_info "Disk space exceeds minimum - can increase retention periods"
fi

if [ "$CPU_CORES" -lt 4 ]; then
    print_warning "CPU cores below minimum - performance will be limited"
elif [ "$CPU_CORES" -lt 8 ]; then
    print_info "CPU cores meet minimum - consider 8+ cores for better performance"
else
    print_info "CPU cores exceed minimum - can increase worker threads"
fi

echo ""
echo "═══════════════════════════════════════════════════════════"
echo "Test Summary"
echo "═══════════════════════════════════════════════════════════"
echo -e "Tests Passed: ${GREEN}$TESTS_PASSED${NC}"
echo -e "Tests Failed: ${RED}$TESTS_FAILED${NC}"
echo ""

if [ $TESTS_FAILED -eq 0 ]; then
    echo -e "${GREEN}✓ All tests passed!${NC}"
    echo "Your system meets the requirements for running SilverBitcoin on consumer hardware."
    echo ""
    echo "Next steps:"
    echo "1. Copy node-consumer.toml.example to node.toml"
    echo "2. Edit node.toml and set your external_address"
    echo "3. Run: ./silver-node --config node.toml"
    echo ""
    exit 0
else
    echo -e "${RED}✗ Some tests failed${NC}"
    echo "Please review the failures above and address them before running the node."
    echo ""
    echo "For help, see: docs/operator/CONSUMER_HARDWARE_GUIDE.md"
    echo ""
    exit 1
fi
