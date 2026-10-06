#!/usr/bin/env bash
set -euo pipefail
set +x
out="${1:?Usage: verify-endpoint.sh ARTIFACT_DIRECTORY}"
mkdir -p "$out"
token="$(vault kv get -field=token secret/codetether/endpoints/public-server)"
trap 'unset token' EXIT
date -u +'%Y-%m-%dT%H:%M:%SZ' > "$out/checked-at.txt"
for path in health api/version api/agent; do
  code=$(curl -sS --max-time 30 -o /dev/null -w '%{http_code}' "https://server.codetether.run/$path")
  printf 'anonymous /%s %s\n' "$path" "$code"
  if [[ "$path" == health ]]; then test "$code" = 200; else test "$code" = 401; fi
done
for path in api/version api/agent; do
  code=$(curl -sS --max-time 30 -o "$out/$(basename "$path").json" -w '%{http_code}' \
    -H @/dev/fd/3 "https://server.codetether.run/$path" 3<<<"Authorization: Bearer $token")
  printf 'authenticated /%s %s\n' "$path" "$code"
  test "$code" = 200
done
unset token
