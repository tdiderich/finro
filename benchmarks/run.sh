#!/usr/bin/env bash
set -euo pipefail

# Kazam benchmark runner
# Usage: ./benchmarks/run.sh <test-name>
# Runs the same prompt against two worktrees: one with kazam, one without.
# Outputs JSON results to benchmarks/results/<test-name>-{kazam,vanilla}.json

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
RESULTS_DIR="$SCRIPT_DIR/results"
TESTS_DIR="$SCRIPT_DIR/tests"
KAZAM_BIN="${KAZAM_BIN:-$(which kazam 2>/dev/null || echo kazam)}"

mkdir -p "$RESULTS_DIR"

if [ $# -lt 1 ]; then
  echo "Usage: $0 <test-name>"
  echo ""
  echo "Available tests:"
  for f in "$TESTS_DIR"/*.sh; do
    name=$(basename "$f" .sh)
    desc=$(head -3 "$f" | grep "^# DESC:" | sed 's/^# DESC: //')
    printf "  %-30s %s\n" "$name" "$desc"
  done
  exit 1
fi

TEST_NAME="$1"
TEST_FILE="$TESTS_DIR/$TEST_NAME.sh"

if [ ! -f "$TEST_FILE" ]; then
  echo "Error: test '$TEST_NAME' not found at $TEST_FILE"
  exit 1
fi

# Source the test to get REPO, PROMPT, MODEL
source "$TEST_FILE"

# Validate required vars
: "${REPO:?TEST must set REPO}"
: "${PROMPT:?TEST must set PROMPT}"
MODEL="${MODEL:-claude-sonnet-4-6}"

echo "=== Benchmark: $TEST_NAME ==="
echo "Repo:   $REPO"
echo "Model:  $MODEL"
echo "Prompt: ${PROMPT:0:80}..."
echo ""

# Create two worktrees
WORK_BASE=$(mktemp -d)
VANILLA_DIR="$WORK_BASE/vanilla"
KAZAM_DIR="$WORK_BASE/kazam"

cleanup() {
  echo "Cleaning up worktrees..."
  cd "$REPO"
  git worktree remove "$VANILLA_DIR" --force 2>/dev/null || true
  git worktree remove "$KAZAM_DIR" --force 2>/dev/null || true
  rm -rf "$WORK_BASE" 2>/dev/null || true
}
trap cleanup EXIT

BRANCH_VANILLA="bench-vanilla-$$"
BRANCH_KAZAM="bench-kazam-$$"

cd "$REPO"
BASE_SHA=$(git rev-parse HEAD)
git worktree add "$VANILLA_DIR" -b "$BRANCH_VANILLA" "$BASE_SHA" --quiet
git worktree add "$KAZAM_DIR" -b "$BRANCH_KAZAM" "$BASE_SHA" --quiet

# Set up kazam workspace in the kazam worktree
echo "Setting up kazam workspace..."
(cd "$KAZAM_DIR" && "$KAZAM_BIN" workspace init --agent claude 2>&1 | head -3)
echo ""

# Strip any existing kazam workspace from vanilla
rm -rf "$VANILLA_DIR/.kazam" "$VANILLA_DIR/.claude/rules/kazam-workspace.md" 2>/dev/null || true

# Run vanilla
echo "--- Running VANILLA ---"
VANILLA_OUT="$RESULTS_DIR/${TEST_NAME}-vanilla.json"
(cd "$VANILLA_DIR" && claude -p "$PROMPT" \
  --model "$MODEL" \
  --output-format json \
  --max-turns 50 \
  --permission-mode default 2>/dev/null) > "$VANILLA_OUT" || true
echo "  Saved: $VANILLA_OUT"

# Run kazam
echo "--- Running KAZAM ---"
KAZAM_OUT="$RESULTS_DIR/${TEST_NAME}-kazam.json"
(cd "$KAZAM_DIR" && claude -p "$PROMPT" \
  --model "$MODEL" \
  --output-format json \
  --max-turns 50 \
  --permission-mode default 2>/dev/null) > "$KAZAM_OUT" || true
echo "  Saved: $KAZAM_OUT"

# Compare
echo ""
echo "=== Results ==="
for variant in vanilla kazam; do
  file="$RESULTS_DIR/${TEST_NAME}-${variant}.json"
  if [ -f "$file" ] && [ -s "$file" ]; then
    cost_usd=$(jq -r '.total_cost_usd // "N/A"' "$file" 2>/dev/null || echo "N/A")
    duration=$(jq -r '.duration_ms // "N/A"' "$file" 2>/dev/null || echo "N/A")
    input_tokens=$(jq -r '.usage.input_tokens // "N/A"' "$file" 2>/dev/null || echo "N/A")
    output_tokens=$(jq -r '.usage.output_tokens // "N/A"' "$file" 2>/dev/null || echo "N/A")
    printf "  %-10s cost=%-10s duration=%-10s in=%-10s out=%-10s\n" \
      "$variant" "$cost_usd" "${duration}ms" "$input_tokens" "$output_tokens"
  else
    echo "  $variant: no output"
  fi
done

# Clean up temp branches
cd "$REPO"
git branch -D "$BRANCH_VANILLA" 2>/dev/null || true
git branch -D "$BRANCH_KAZAM" 2>/dev/null || true

echo ""
echo "Done. Raw JSON in $RESULTS_DIR/"
