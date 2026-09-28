#!/usr/bin/env bash
# Replace a Forgejo release body with the generated per-platform install notes.
# Usage: script/forgejo/update-release-notes.sh <version-without-v>...
# Uses git's existing credential helper for forgejo.quantum-forge.io; the
# token is never printed or written to disk.
set -euo pipefail
root=$(cd "$(dirname "$0")/../.." && pwd)
api=https://forgejo.quantum-forge.io/api/v1/repos/riley/codetether-agent
token=$(printf 'protocol=https\nhost=forgejo.quantum-forge.io\n\n' | git credential fill | sed -n 's/^password=//p')
test -n "$token"
for version in "$@"; do
  id=$(curl -fsS "$api/releases/tags/v$version" | jq -r .id)
  body=$(bash "$root/script/forgejo/release-notes.sh" "$version")
  jq -n --arg body "$body" '{body: $body}' |
    curl -fsS -X PATCH -H 'Content-Type: application/json' -H "Authorization: token $token" \
      --data-binary @- "$api/releases/$id" | jq -r '"updated \(.tag_name) body_lines=\(.body|split("\n")|length)"'
done
