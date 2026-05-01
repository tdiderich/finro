#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
TESTS_DIR="$SCRIPT_DIR/tests"
RESULTS_DIR="$SCRIPT_DIR/results"

echo "=== Running all benchmarks ==="
echo "Started: $(date)"
echo ""

for test_file in "$TESTS_DIR"/*.sh; do
  name=$(basename "$test_file" .sh)
  echo "────────────────────────────────────────"
  echo "[$name] Starting at $(date '+%H:%M:%S')"
  echo "────────────────────────────────────────"
  "$SCRIPT_DIR/run.sh" "$name" 2>&1
  echo ""
done

echo "========================================"
echo "All benchmarks complete: $(date)"
echo ""

# Print summary table
echo "=== SUMMARY ==="
printf "%-25s %-12s %-12s %-12s %-12s\n" "Test" "V-Cost" "K-Cost" "V-Duration" "K-Duration"
printf "%-25s %-12s %-12s %-12s %-12s\n" "----" "------" "------" "----------" "----------"
for test_file in "$TESTS_DIR"/*.sh; do
  name=$(basename "$test_file" .sh)
  v_file="$RESULTS_DIR/${name}-vanilla.json"
  k_file="$RESULTS_DIR/${name}-kazam.json"

  v_cost="N/A"; k_cost="N/A"; v_dur="N/A"; k_dur="N/A"
  if [ -f "$v_file" ] && [ -s "$v_file" ]; then
    v_cost=$(jq -r '.total_cost_usd // "N/A"' "$v_file" 2>/dev/null || echo "N/A")
    v_dur=$(jq -r '.duration_ms // "N/A"' "$v_file" 2>/dev/null || echo "N/A")
  fi
  if [ -f "$k_file" ] && [ -s "$k_file" ]; then
    k_cost=$(jq -r '.total_cost_usd // "N/A"' "$k_file" 2>/dev/null || echo "N/A")
    k_dur=$(jq -r '.duration_ms // "N/A"' "$k_file" 2>/dev/null || echo "N/A")
  fi
  printf "%-25s %-12s %-12s %-12s %-12s\n" "$name" "$v_cost" "$k_cost" "${v_dur}" "${k_dur}"
done
