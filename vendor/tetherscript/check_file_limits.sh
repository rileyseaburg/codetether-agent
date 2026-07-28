#!/usr/bin/env bash
set -euo pipefail

limit=50
base="${1:-}"

if [[ -z "$base" && -n "${GITHUB_BASE_REF:-}" ]]; then
  git fetch --no-tags origin "${GITHUB_BASE_REF}" >/dev/null 2>&1 || true
  base="$(git merge-base "origin/${GITHUB_BASE_REF}" HEAD 2>/dev/null || true)"
fi

if [[ -z "$base" ]] && git rev-parse --verify HEAD~1 >/dev/null 2>&1; then
  base="HEAD~1"
fi

if [[ -z "$base" ]]; then
  base="HEAD"
fi

count_effective_lines() {
  awk '
    {
      line=$0
      sub(/^[[:space:]]+/, "", line)
      if (line == "") next
      if (line ~ /^\/\//) next
      if (line ~ /^\/\*/) next
      if (line ~ /^\*/) next
      count++
    }
    END { print count + 0 }
  ' "$1"
}

count_base_lines() {
  git show "$base:$1" 2>/dev/null | awk '
    {
      line=$0
      sub(/^[[:space:]]+/, "", line)
      if (line == "") next
      if (line ~ /^\/\//) next
      if (line ~ /^\/\*/) next
      if (line ~ /^\*/) next
      count++
    }
    END { print count + 0 }
  ' || echo 0
}

files="$(
  {
    git diff --name-only --diff-filter=ACMR "$base"...HEAD -- 'src/**/*.rs' 2>/dev/null || true
    git diff --name-only --diff-filter=ACMR -- 'src/**/*.rs'
    git diff --cached --name-only --diff-filter=ACMR -- 'src/**/*.rs'
    git ls-files --others --exclude-standard -- 'src/**/*.rs'
  } | sort -u
)"

failed=0
for file in $files; do
  [[ -f "$file" ]] || continue
  current="$(count_effective_lines "$file")"
  previous="$(count_base_lines "$file")"
  if (( current <= limit )); then
    continue
  fi
  if (( previous > limit && current <= previous )); then
    continue
  fi
  echo "$file has $current effective lines; limit is $limit, previous was $previous"
  failed=1
done

exit "$failed"
