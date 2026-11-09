#!/usr/bin/env bash
set -euo pipefail

printf '%s\n' "$*" >>"$CARGO_CALLS"
attempt=0
if [[ -f "$CARGO_STATE" ]]; then
  read -r attempt <"$CARGO_STATE"
fi
((attempt += 1))
printf '%s\n' "$attempt" >"$CARGO_STATE"
if [[ $attempt -eq 1 ]]; then
  printf 'error: synthetic compile failure\n' >&2
  exit 1
fi
