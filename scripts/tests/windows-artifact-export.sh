#!/usr/bin/env bash
# Mocked local regression: daemon-side image export and stopped-container copy.
set -euo pipefail
root=$(cd "$(dirname "$0")/../.." && pwd)
mkdir -p "$root/artifacts/release-verification"
fixture=$(mktemp -d "$root/artifacts/release-verification/windows-export-XXXXXX")
mkdir -p "$fixture/bin" "$fixture/docker/release"
cp "$root/docker/release/windows.Dockerfile" "$fixture/docker/release/"
cat > "$fixture/bin/docker" <<'MOCK'
#!/usr/bin/env bash
set -euo pipefail
test "$DOCKER_HOST" = unix:///var/run/docker.sock
test -z "${DOCKER_CONTEXT:-}${DOCKER_TLS_VERIFY:-}${DOCKER_CERT_PATH:-}"
printf '%s\n' "$*" >> "$MOCK_LOG"
case "$1 ${2:-}" in
  'info ') ;;
  'buildx inspect') test "$#" -eq 2; printf 'Driver: %s\n' "${MOCK_DRIVER:-docker}" ;;
  'buildx build')
    [[ " $* " != *' --output '* && " $* " != *' --load '* ]]
    [[ " $* " == *' --progress plain '* && " $* " == *' --tag codetether-windows-artifact:'* ]]
    exit "${MOCK_BUILD_EXIT:-0}"
    ;;
  create\ *)
    test "$3" = /codetether.exe
    echo mock-container
    ;;
  'cp mock-container:/codetether.exe')
    test "$3" = build/windows/codetether.exe
    if [ "${MOCK_COPY_EXIT:-0}" != 0 ]; then exit "$MOCK_COPY_EXIT"; fi
    printf 'MZmock executable' > "$3"
    ;;
  'rm mock-container'|'image rm') ;;
  *) echo "Unexpected Docker call: $*" >&2; exit 99 ;;
esac
MOCK
chmod +x "$fixture/bin/docker"
export PATH="$fixture/bin:$PATH" MOCK_LOG="$fixture/success.log"
export DOCKER_CONTEXT=stale DOCKER_TLS_VERIFY=1 DOCKER_CERT_PATH=/stale
cd "$fixture"
bash "$root/scripts/forgejo/build-windows-artifact.sh"
test "$(cat build/windows/codetether.exe)" = 'MZmock executable'
grep -Fxq 'rm mock-container' "$MOCK_LOG"
grep -q '^image rm codetether-windows-artifact:' "$MOCK_LOG"

expect_failure() {
  local expected=$1
  shift
  local status=0
  "$@" || status=$?
  test "$status" -eq "$expected"
}
export MOCK_LOG="$fixture/copy-failure.log"
expect_failure 23 env MOCK_COPY_EXIT=23 bash "$root/scripts/forgejo/build-windows-artifact.sh"
grep -Fxq 'rm mock-container' "$MOCK_LOG"
grep -q '^image rm codetether-windows-artifact:' "$MOCK_LOG"
export MOCK_LOG="$fixture/build-failure.log"
expect_failure 42 env MOCK_BUILD_EXIT=42 bash "$root/scripts/forgejo/build-windows-artifact.sh"
if grep -q '^create ' "$MOCK_LOG"; then exit 1; fi
grep -q '^image rm codetether-windows-artifact:' "$MOCK_LOG"
export MOCK_LOG="$fixture/driver-failure.log"
expect_failure 1 env MOCK_DRIVER=docker-container bash "$root/scripts/forgejo/build-windows-artifact.sh"
if grep -q '^buildx build' "$MOCK_LOG"; then exit 1; fi
grep -Fq 'run: bash scripts/forgejo/build-windows-artifact.sh' "$root/.forgejo/workflows/release-windows.yml"
printf 'mocked local: export, cleanup, failure propagation, and driver checks passed; evidence: %s\n' "$fixture"