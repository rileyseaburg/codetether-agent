#!/usr/bin/env bash
set -euo pipefail
root=$(cd "$(dirname "$0")/../.." && pwd)
fixture=$(mktemp -d)
trap 'printf "Mocked installer evidence retained: %s\n" "$fixture"' EXIT
mkdir -p "$fixture/scripts" "$fixture/home"
cp "$root/scripts/install-dev.sh" "$fixture/scripts/"
cp "$root/scripts/install-dev-cache.sh" "$fixture/scripts/"
printf '#!/usr/bin/env bash\necho 0.0.0-test\n' > "$fixture/scripts/bump-dev-version.sh"
chmod +x "$fixture/scripts/bump-dev-version.sh"
# Replace copy/service guidance with a marker: never touch installed binaries.
printf 'echo POST_INSTALL\n' > "$fixture/scripts/install-dev-post-install.sh"
cargo() { return "${MOCK_CARGO_EXIT:-0}"; }
export -f cargo
export HOME="$fixture/home" TMPDIR="$fixture"
export CODETETHER_INSTALL_AUTO_FIX=0 CODETETHER_INSTALL_SCCACHE=0
export MOCK_CARGO_EXIT=0
bash "$fixture/scripts/install-dev.sh" --foreground > "$fixture/success.log"
grep -Fxq POST_INSTALL "$fixture/success.log"
grep -Fxq 'Build succeeded.' "$fixture/success.log"
bash "$fixture/scripts/install-dev.sh" --foreground test --lib > "$fixture/test.log"
! grep -Fq POST_INSTALL "$fixture/test.log"
grep -Fxq 'Build succeeded.' "$fixture/test.log"
export MOCK_CARGO_EXIT=42
status=0
bash "$fixture/scripts/install-dev.sh" --foreground > "$fixture/failure.log" || status=$?
[[ "$status" == 42 ]]
! grep -Fq POST_INSTALL "$fixture/failure.log"
! grep -Fq 'Build succeeded.' "$fixture/failure.log"
echo 'mocked local: success, test-only, and failed installer paths passed'



