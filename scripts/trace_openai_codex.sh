#!/usr/bin/env bash
set -uo pipefail

if [[ ${1:-} == "--help" || $# -eq 0 ]]; then
    echo "usage: scripts/trace_openai_codex.sh '<prompt>'"
    exit $(( $# == 0 ))
fi

script_dir=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
repo_root=$(cd -- "$script_dir/.." && pwd)
stamp=$(date -u +%Y%m%dT%H%M%SZ)
trace_dir=${CODETETHER_TRACE_DIR:-"$repo_root/artifacts/openai-codex-trace/$stamp"}
prompt=$*

mkdir -p -- "$trace_dir"
chmod 700 -- "$trace_dir"
export CODETETHER_OPENAI_CODEX_ALLOW_CHATGPT_BACKEND=1
export RUST_BACKTRACE=full
export RUST_LOG=${RUST_LOG:-"codetether_agent::provider::openai_codex=trace,info"}

{
    date -u +"started_at=%Y-%m-%dT%H:%M:%SZ"
    echo "model=openai-codex/gpt-5.6-sol"
    echo "rust_log=$RUST_LOG"
    codetether --version
    strace --version | head -n 1
} >"$trace_dir/metadata.txt"
printf '%s\n' "$prompt" >"$trace_dir/prompt.txt"

strace -ff -ttt -T -yy -xx -s 0 -e trace=%network \
    -o "$trace_dir/network.strace" \
    codetether run --print-logs -m openai-codex/gpt-5.6-sol "$prompt" \
    > >(tee "$trace_dir/stdout.log") \
    2> >(tee "$trace_dir/stderr.log" >&2)
status=$?

parse_status=0
parse_count=0
for trace_file in "$trace_dir"/network.strace.*; do
    [[ $trace_file =~ \.[0-9]+$ ]] || continue
    ((parse_count += 1))
    python3 "$repo_root/scripts/parse_strace.py" \
        "$trace_file" "$trace_file.jsonl" || parse_status=$?
done
if (( parse_count == 0 )); then
    parse_status=1
fi
if (( status == 0 && parse_status != 0 )); then
    status=$parse_status
fi
printf 'exit_status=%s\nparse_status=%s\nparsed_files=%s\n' \
    "$status" "$parse_status" "$parse_count" >"$trace_dir/result.txt"
printf 'Trace artifacts: %s\n' "$trace_dir" >&2
exit "$status"
