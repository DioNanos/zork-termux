#!/bin/bash
# Release Preflight Script for zork-termux
# Runs all verification checks before a release
# Exit code !=0 on failure
set -u

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
FAILED=0
PASSED=0

echo "ZORK-TERMUX Release Preflight"
echo "=============================="
echo ""

cd "$PROJECT_DIR"

# 1. Cargo test
echo "[1/6] Running cargo test..."
if cargo test >/dev/null 2>&1; then
    echo "      PASS: cargo test"
    PASSED=$((PASSED+1))
else
    echo "      FAIL: cargo test"
    FAILED=$((FAILED+1))
fi

# 2. Smoke all languages
echo "[2/6] Running smoke_all_languages.sh..."
if bash scripts/smoke_all_languages.sh >/dev/null 2>&1; then
    echo "      PASS: smoke_all_languages"
    PASSED=$((PASSED+1))
else
    echo "      FAIL: smoke_all_languages"
    FAILED=$((FAILED+1))
fi

# 3. Corepath smoke
echo "[3/6] Running corepath_smoke.sh..."
if bash scripts/corepath_smoke.sh >/dev/null 2>&1; then
    echo "      PASS: corepath_smoke"
    PASSED=$((PASSED+1))
else
    echo "      FAIL: corepath_smoke"
    FAILED=$((FAILED+1))
fi

# 4. JSON validation
echo "[4/6] Validating i18n JSON files..."
JSON_OK=true
for f in data/i18n/en.json data/i18n/it.json data/i18n/es.json; do
    if ! jq empty "$f" 2>/dev/null; then
        echo "      FAIL: $f is not valid JSON"
        JSON_OK=false
    fi
done
if $JSON_OK; then
    echo "      PASS: JSON validation"
    PASSED=$((PASSED+1))
else
    FAILED=$((FAILED+1))
fi

# 5. Data alignment gate (errors only, warnings OK)
echo "[5/6] Running data_alignment_gate.py..."
GATE_OUTPUT=$(python3 scripts/data_alignment_gate.py 2>&1)
GATE_ERRORS=$(echo "$GATE_OUTPUT" | grep "^Errors:" | awk '{print $2}' || echo 0)
if [ "$GATE_ERRORS" = "0" ] || [ -z "$GATE_ERRORS" ]; then
    WARNINGS=$(echo "$GATE_OUTPUT" | grep "^Warnings:" | awk '{print $2}')
    echo "      PASS: data_alignment_gate (warnings: ${WARNINGS:-0})"
    PASSED=$((PASSED+1))
else
    echo "      FAIL: data_alignment_gate ($GATE_ERRORS errors)"
    FAILED=$((FAILED+1))
fi

# 6. Git working tree check
echo "[6/6] Checking git working tree..."
if git diff --quiet && git diff --cached --quiet; then
    echo "      PASS: working tree clean"
    PASSED=$((PASSED+1))
else
    if [ "${ALLOW_DIRTY:-0}" = "1" ]; then
        echo "      WARN: uncommitted changes (ALLOW_DIRTY=1)"
        echo "      Files changed:"
        git status --short | head -5
        PASSED=$((PASSED+1))
    else
        echo "      FAIL: working tree not clean"
        echo "      Files changed:"
        git status --short | head -5
        FAILED=$((FAILED+1))
    fi
fi

echo ""
echo "=============================="
echo "Results: PASSED=$PASSED FAILED=$FAILED"
echo ""

if [ $FAILED -gt 0 ]; then
    echo "PREFLIGHT FAILED - Fix issues before release"
    exit 1
else
    echo "PREFLIGHT PASSED - Ready for release"
    exit 0
fi
