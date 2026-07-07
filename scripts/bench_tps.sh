#!/usr/bin/env bash
# Measure tokens-per-second for openai-codex/gpt-5-mini vs openai-codex/gpt-5.5
# Uses `codetether run` with a fixed prompt and counts output tokens via word proxy.
# Output: model, elapsed_s, approx_tokens, tps

set -euo pipefail

PROMPT="Write a detailed 500-word technical explanation of how transformer attention mechanisms work, including the math behind scaled dot-product attention, multi-head attention, and why positional encodings are needed."

RUNS=${BENCH_RUNS:-3}

bench_model() {
    local model="$1"
    local total_elapsed=0
    local total_tokens=0

    for i in $(seq 1 "$RUNS"); do
        local start end elapsed output word_count token_est
        start=$(date +%s%3N)
        output=$(CODETETHER_MODEL="openai-codex/$model" \
            codetether run "$PROMPT" 2>/dev/null)
        end=$(date +%s%3N)
        elapsed=$(( end - start ))

        # Approximate tokens: words * 1.33 (rough token/word ratio for English prose)
        word_count=$(echo "$output" | wc -w)
        token_est=$(echo "$word_count 1.33" | awk '{printf "%d", $1 * $2}')

        total_elapsed=$(( total_elapsed + elapsed ))
        total_tokens=$(( total_tokens + token_est ))

        printf "  run %d: %d ms, ~%d tokens\n" "$i" "$elapsed" "$token_est"
    done

    local avg_elapsed avg_tokens tps
    avg_elapsed=$(( total_elapsed / RUNS ))
    avg_tokens=$(( total_tokens / RUNS ))
    tps=$(echo "$avg_tokens $avg_elapsed" | awk '{printf "%.1f", $1 / ($2 / 1000.0)}')

    printf "  %-20s avg: %d ms, ~%d tokens, TPS = %s\n\n" \
        "openai-codex/$model" "$avg_elapsed" "$avg_tokens" "$tps"
}

echo "=== TPS Benchmark: gpt-5-mini vs gpt-5.5 (${RUNS} runs each) ==="
echo ""
echo "--- gpt-5-mini ---"
bench_model "gpt-5-mini"

echo "--- gpt-5.5 ---"
bench_model "gpt-5.5"
