#!/usr/bin/env bash
# Mocked local contract regression: tags use target, releases use target_commitish.
set -euo pipefail
root=$(cd "$(dirname "$0")/../.." && pwd)
mkdir -p "$root/artifacts/release-verification"
fixture=$(mktemp -d "$root/artifacts/release-verification/tag-target-XXXXXX")
mkdir -p "$fixture/bin" "$fixture/dist"
printf fixture > "$fixture/dist/artifact.txt"
printf notes > "$fixture/notes.md"
cat > "$fixture/bin/curl" <<'MOCK'
#!/usr/bin/env bash
set -euo pipefail
method=GET body='' url=''
while [ "$#" -gt 0 ]; do
  case "$1" in
    -X) method=$2; shift 2 ;;
    -d) body=$2; shift 2 ;;
    -H|-F) shift 2 ;;
    --fail|-sS) shift ;;
    https://*) url=$1; shift ;;
    *) exit 99 ;;
  esac
done
path=${url#https://forgejo.invalid/api/v1/repos/test/project/}
printf '%s %s\n' "$method" "$path" >> "$MOCK_LOG"
case "$method $path" in
  DELETE\ *) ;;
  'POST tags')
    jq -e --arg sha "$SHA" '.target == $sha and (has("target_commitish") | not)' <<< "$body" >/dev/null
    ;;
  GET\ tags/*) jq -n --arg sha "${MOCK_TAG_SHA:-$SHA}" '{commit:{sha:$sha}}' ;;
  'POST releases')
    grep -Fq "GET tags/$TAG" "$MOCK_LOG"
    jq -e --arg sha "$SHA" '.target_commitish == $sha and .draft == true' <<< "$body" >/dev/null
    echo '{"id":123}'
    ;;
  POST\ releases/123/assets\?*) echo '{"id":456}' ;;
  'PATCH releases/123') jq -e '.draft == false' <<< "$body" >/dev/null ;;
  *) echo "Unexpected request: $method $path" >&2; exit 99 ;;
esac
MOCK
chmod +x "$fixture/bin/curl"
export PATH="$fixture/bin:$PATH" MOCK_LOG="$fixture/success-requests.log"
export RELEASE_URL=https://forgejo.invalid TOKEN=mock-token REPOSITORY=test/project
export TAG=vtest SHA=aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa VERSION=test
export NOTES_FILE="$fixture/notes.md" DIST_DIR="$fixture/dist"
cd "$fixture"
bash "$root/scripts/forgejo/publish-release-assets.sh" > "$fixture/success.log"
grep -Fxq 'PATCH releases/123' "$MOCK_LOG"
export MOCK_LOG="$fixture/mismatch-requests.log"
status=0
MOCK_TAG_SHA=bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb \
  bash "$root/scripts/forgejo/publish-release-assets.sh" > "$fixture/mismatch.log" 2>&1 || status=$?
test "$status" -eq 1
grep -Fq 'sha mismatch' "$fixture/mismatch.log"
if grep -Eq '^POST releases|^PATCH releases' "$MOCK_LOG"; then
  echo 'Mismatched tag must stop before release creation or upload' >&2
  exit 1
fi
printf 'mocked local: tag target contract and mismatch guard passed; evidence: %s\n' "$fixture"
