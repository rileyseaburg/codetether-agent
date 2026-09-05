#!/usr/bin/env bash
# Mocked local cache ownership and output-copy regression for the Docker wrapper.
set -euo pipefail
root=$(cd "$(dirname "$0")/../.." && pwd)
fixture=$(mktemp -d)
trap 'rm -rf "$fixture"' EXIT
mkdir -p "$fixture/script" "$fixture/bin" "$fixture/.docker-cache/windows" \
  "$fixture/.docker-cache/windows-new"
cp "$root/script/build-windows-docker.sh" "$fixture/script/"
printf old > "$fixture/.docker-cache/windows/old"
printf stale > "$fixture/.docker-cache/windows-new/stale"
cat > "$fixture/bin/docker" <<'MOCK'
#!/usr/bin/env bash
set -euo pipefail
case "$1 $2" in
  'buildx inspect') printf 'Driver: %s\n' "$MOCK_DRIVER" ;;
  'buildx build')
    mkdir -p dist
    printf 'mock executable' > dist/codetether.exe
    if [ "$MOCK_DRIVER" = docker-container ]; then
      printf fresh > .docker-cache/windows-new/fresh
    fi
    ;;
  *) exit 1 ;;
esac
MOCK
chmod +x "$fixture/bin/docker"
export PATH="$fixture/bin:$PATH"
MOCK_DRIVER=docker bash "$fixture/script/build-windows-docker.sh"
test "$(cat "$fixture/.docker-cache/windows/old")" = old
test "$(cat "$fixture/.docker-cache/windows-new/stale")" = stale
cmp "$fixture/dist/codetether.exe" "$fixture/dist/windows/codetether.exe"
cmp "$fixture/dist/codetether.exe" "$fixture/dist/codetether-windows.exe"
MOCK_DRIVER=docker-container bash "$fixture/script/build-windows-docker.sh"
test "$(cat "$fixture/.docker-cache/windows/fresh")" = fresh
test ! -e "$fixture/.docker-cache/windows/old"
test ! -d "$fixture/.docker-cache/windows-new"
printf '%s\n' 'mocked local: Docker cache ownership and artifact-copy checks passed'