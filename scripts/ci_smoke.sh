#!/bin/bash
# CI smoke test with detailed logging
# Used by GitHub Actions smoke.yml

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
LOG_DIR="${PROJECT_DIR}/smoke-logs"
TIMEOUT=60

run_with_timeout() {
    local seconds="$1"
    shift

    if command -v timeout >/dev/null 2>&1; then
        timeout "$seconds" "$@"
    elif command -v gtimeout >/dev/null 2>&1; then
        gtimeout "$seconds" "$@"
    else
        "$@"
    fi
}

mkdir -p "$LOG_DIR"

run_smoke_with_log() {
    local lang="$1"
    local lang_name="$2"
    local log_file="$LOG_DIR/smoke_${lang}.log"
    
    echo "=== Testing $lang_name ===" | tee -a "$LOG_DIR/smoke-output.txt"
    
    cd "$PROJECT_DIR"
    
    if [ ! -f target/release/zork-termux ]; then
        cargo build --release >> "$log_file" 2>&1
    fi
    
    local input=$(cat <<EOF
$lang
1
look
inventory
quit
EOF
)
    
    local cmd_exit=0
    {
        echo "=== $lang_name smoke test ==="
        echo "Date: $(date -Iseconds)"
        echo "Input: $input"
        echo "---"
        echo "$input" | run_with_timeout "$TIMEOUT" ./target/release/zork-termux 2>&1
        cmd_exit=$?
        echo "---"
        echo "Exit code: $cmd_exit"
    } > "$log_file"
    
    cat "$log_file" >> "$LOG_DIR/smoke-output.txt"
    
    if [ $cmd_exit -eq 0 ]; then
        echo "PASS: $lang_name" | tee -a "$LOG_DIR/smoke-output.txt"
        return 0
    else
        echo "FAIL: $lang_name" | tee -a "$LOG_DIR/smoke-output.txt"
        return 1
    fi
}

# Run tests
FAILED=0

run_smoke_with_log "1" "English" || ((FAILED++))
run_smoke_with_log "2" "Italian" || ((FAILED++))
run_smoke_with_log "3" "Spanish" || ((FAILED++))

echo ""
echo "=== Summary ===" | tee -a "$LOG_DIR/smoke-output.txt"
echo "Failed: $FAILED" | tee -a "$LOG_DIR/smoke-output.txt"

exit $FAILED
