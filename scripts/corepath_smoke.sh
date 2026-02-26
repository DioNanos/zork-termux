#!/bin/bash
# Core-path smoke test runner for zork-termux
# Tests core gameplay scenarios from CORE_PATH_GAMEPLAY_MATRIX
# Exit code !=0 on failure

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
TIMEOUT=10
LOGS_DIR="$PROJECT_DIR/logs/corepath"

FAILED=0
PASSED=0
RESULTS=()

run_test() {
    local id="$1"
    local name="$2"
    local lang="$3"
    local input="$4"
    local expected="$5"
    
    local full_input="${lang}
1
${input}
quit"
    
    local output
    if ! output=$(echo -e "$full_input" | timeout $TIMEOUT ./target/release/zork-termux 2>&1); then
        echo "    [$id] $name: FAIL (timeout/error)"
        ((FAILED++))
        RESULTS+=("FAIL: $id - $name")
        return 1
    fi
    
    if echo "$output" | grep -qi "$expected"; then
        echo "    [$id] $name: PASS"
        ((PASSED++))
        RESULTS+=("PASS: $id - $name")
        return 0
    else
        echo "    [$id] $name: FAIL (expected '$expected')"
        ((FAILED++))
        RESULTS+=("FAIL: $id - $name")
        return 1
    fi
}

echo "ZORK-TERMUX Core-path Smoke Test"
echo "================================="
echo ""

cd "$PROJECT_DIR"

if [ ! -f target/release/zork-termux ]; then
    echo "Building release..."
    cargo build --release 2>/dev/null || exit 1
fi

mkdir -p "$LOGS_DIR"

echo "Testing English scenarios..."
run_test "CP01" "EN Basic look" "1" "look" "West of House"
run_test "CP02" "EN Open mailbox" "1" "open mailbox" "Opened"
run_test "CP03" "EN Take leaflet" "1" "open mailbox\ntake leaflet" "Taken"
run_test "CP05" "EN Move north" "1" "north" "North of House"
run_test "CP06" "EN Move south" "1" "south" "South of House"
run_test "CP07" "EN Move west" "1" "west" "Forest"
run_test "CP09" "EN Enter window" "1" "north\neast\nenter" "Kitchen"
run_test "CP19" "EN Use lamp (in living room)" "1" "north\neast\nin\nwest\ntake lamp\nuse lamp" "lamp is on"
run_test "CP27" "EN Inventory" "1" "open mailbox\ntake leaflet\ninventory" "carrying"
run_test "CP28" "EN Drop item" "1" "open mailbox\ntake leaflet\ndrop leaflet" "Dropped"
run_test "CP30" "EN Save game" "1" "save" "saved"

echo ""
echo "Testing Italian scenarios..."
run_test "CP02" "IT Apri cassetta" "2" "apri cassetta" "Aperto"
run_test "CP03" "IT Prendi volantino" "2" "apri cassetta\nprendi volantino" "Preso"
run_test "CP05" "IT Vai nord" "2" "nord" "Nord della Casa"
run_test "CP09" "IT Entra finestra" "2" "nord\nest\nentra" "Cucina"
run_test "CP30" "IT Salva partita" "2" "salva" "salvata"

echo ""
echo "Testing Spanish scenarios..."
run_test "CP02" "ES Abrir buzón" "3" "abrir buzón" "Abierto"
run_test "CP03" "ES Tomar folleto" "3" "abrir buzón\ntomar folleto" "Tomado"
run_test "CP05" "ES Ir norte" "3" "norte" "Norte de la Casa"
run_test "CP09" "ES Entrar ventana" "3" "norte\neste\nentrar" "Cocina"
run_test "CP30" "ES Guardar partida" "3" "guardar" "guardado"

echo ""
echo "================================="
echo "Summary:"
for r in "${RESULTS[@]}"; do
    echo "  $r"
done
echo ""
echo "Results: PASSED=$PASSED FAILED=$FAILED"
echo "Logs: $LOGS_DIR"

if [ $FAILED -gt 0 ]; then
    exit 1
fi

exit 0
