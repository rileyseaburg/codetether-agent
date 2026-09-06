#!/usr/bin/env bash
# Mocked local cache ownership and output-copy regression for the Docker wrapper.
set -euo pipefail
root=$(cd "$(dirname "$0")/../.." && pwd)
fixture=$(mktemp -d)
trap 'rm -rf "$fixture"' EXIT
mkdir -p "$fixture/script" "$fixture/bin" "$fixture/.docker-cache/windows" \
  "$fixture/.docker-cache/windows-new"
cp "$root/script/build-windows-docker.sh" "$fixture/script/"
cp "$root/script/package-windows-bundle.py" "$fixture/script/"
cp "$root/install.ps1" "$fixture/"
mkdir -p "$fixture/script/windows-install"
cp "$root/script/windows-install/"*.ps1 "$root/script/windows-install/README.md" "$fixture/script/windows-install/"
cp "$root/script/windows-install/Install-CodeTether.cmd" "$fixture/script/windows-install/"
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
test -s "$fixture/dist/codetether-windows.zip"
test -f "$fixture/dist/windows/script/windows-install/probe-ocr.ps1"
test -f "$fixture/dist/windows/Install-CodeTether.cmd"
MOCK_DRIVER=docker-container bash "$fixture/script/build-windows-docker.sh"
test "$(cat "$fixture/.docker-cache/windows/fresh")" = fresh
test ! -e "$fixture/.docker-cache/windows/old"
test ! -d "$fixture/.docker-cache/windows-new"
printf '%s\n' 'mocked local: Docker cache ownership and artifact-copy checks passed'