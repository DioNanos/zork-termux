#!/bin/bash
# Smoke test runner for zork-termux
# Tests basic gameplay in EN/IT/ES
# Exit code !=0 on failure

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
TIMEOUT=60

FAILED=0
PASSED=0

run_smoke() {
    local lang="$1"
    local lang_name="$2"
    local expected_welcome="$3"
    local expected_room="$4"
    local expected_goodbye="$5"
    
    echo "=== Testing $lang_name ==="
    
    cd "$PROJECT_DIR"
    
    # Build if needed
    if [ ! -f target/release/zork-termux ]; then
        cargo build --release 2>/dev/null
    fi
    
    # Create input sequence: lang, game (1=Zork1), commands
    local input=$(cat <<EOF
$lang
1
look
inventory
quit
EOF
)
    
    # Run with timeout, capture output
    local output
    if output=$(echo "$input" | timeout $TIMEOUT ./target/release/zork-termux 2>&1); then
        :
    else
        local exit_code=$?
        if [ $exit_code -eq 124 ]; then
            echo "FAIL: $lang_name - timeout after ${TIMEOUT}s"
            ((FAILED++))
            return 1
        fi
    fi
    
    # Check expected strings
    local errors=0
    
    if ! echo "$output" | grep -qi "$expected_welcome"; then
        echo "FAIL: $lang_name - missing welcome message"
        echo "  Expected: '$expected_welcome'"
        ((errors++))
    fi
    
    if ! echo "$output" | grep -qi "$expected_room"; then
        echo "FAIL: $lang_name - missing room description"
        echo "  Expected: '$expected_room'"
        ((errors++))
    fi
    
    if ! echo "$output" | grep -qi "$expected_goodbye"; then
        echo "FAIL: $lang_name - missing goodbye message"
        echo "  Expected: '$expected_goodbye'"
        ((errors++))
    fi
    
    if [ $errors -eq 0 ]; then
        echo "PASS: $lang_name"
        ((PASSED++))
    else
        echo "FAIL: $lang_name - $errors check(s) failed"
        ((FAILED++))
        return 1
    fi
}

echo "ZORK-TERMUX Smoke Test Runner"
echo "=============================="
echo ""

# English
run_smoke "1" "English" \
    "Welcome to ZORK" \
    "West of House" \
    "Thanks for playing"

# Italian
run_smoke "2" "Italian" \
    "Benvenuto in ZORK" \
    "A Ovest della Casa" \
    "Grazie per aver giocato"

# Spanish
run_smoke "3" "Spanish" \
    "Bienvenido a ZORK" \
    "Al Oeste de la Casa" \
    "Gracias por jugar"

echo ""
echo "=============================="
echo "Results: PASSED=$PASSED FAILED=$FAILED"

if [ $FAILED -gt 0 ]; then
    exit 1
fi

exit 0
