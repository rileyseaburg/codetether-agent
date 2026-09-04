#!/usr/bin/env bash
# Exercise the release gate with a failing fake Cargo; never build or publish.
set -euo pipefail
root=$(cd "$(dirname "$0")/../.." && pwd)
mkdir -p "$root/artifacts/release-verification"
fixture=$(mktemp -d "$root/artifacts/release-verification/run-XXXXXX")
cp "$root/release.sh" "$fixture/release.sh"
cd "$fixture"
git init -q
git config user.name 'Release gate test'
git config user.email 'release-test@example.invalid'
printf '[package]\nname = "fixture"\nversion = "1.2.3"\n' > Cargo.toml
printf '# Fixture lockfile\n' > Cargo.lock
cat > cargo-stub <<'STUB'
#!/usr/bin/env bash
printf '%s\n' "$*" >> "$RELEASE_TEST_COMMANDS"
case "$1" in
    generate-lockfile) exit 0 ;;
    test) exit 1 ;;
    *) echo "Unexpected Cargo command: $*" >&2; exit 2 ;;
esac
STUB
chmod +x cargo-stub
git add Cargo.toml Cargo.lock release.sh cargo-stub
git -c core.hooksPath=/dev/null commit -qm fixture
export RELEASE_TEST_COMMANDS="$fixture/commands.log"
unset CODETETHER_RELEASE_VERIFY_CMD
if CODETETHER_CARGO_CMD="$fixture/cargo-stub" bash ./release.sh patch > release.log 2>&1; then
    echo 'Release unexpectedly accepted a failing verifier' >&2
    exit 1
fi
grep -Fx 'test --quiet --lib --tests -- --test-threads=1' commands.log
grep -F 'release verification failed' release.log
git diff --exit-code -- Cargo.toml Cargo.lock
test -z "$(git tag --list)"
test "$(git rev-list --count HEAD)" = 1
test "$(wc -l < commands.log)" -eq 2
printf 'Release gate evidence: %s\n' "$fixture"