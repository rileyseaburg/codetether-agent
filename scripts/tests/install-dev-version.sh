#!/usr/bin/env bash
set -euo pipefail

root=$(cd "$(dirname "$0")/../.." && pwd)
fixture=$(mktemp -d)
trap 'rm -rf "$fixture"' EXIT
mkdir -p "$fixture/scripts" "$fixture/bin" "$fixture/home"
cp "$root/scripts/install-dev.sh" "$fixture/scripts/"
cp "$root/scripts/bump-dev-version.sh" "$fixture/scripts/"
cp "$root/scripts/set-dev-version.sh" "$fixture/scripts/"
cp "$root/scripts/tests/fixtures/fake-cargo.sh" "$fixture/bin/cargo"
cp "$root/scripts/tests/fixtures/fake-codetether.sh" "$fixture/bin/codetether"
chmod +x "$fixture/scripts/"*.sh "$fixture/bin/"*

cat >"$fixture/Cargo.toml" <<'EOF'
[package]
name = "codetether-agent"
version = "4.7.5-dev.9"
EOF
cat >"$fixture/Cargo.lock" <<'EOF'
[[package]]
name = "codetether-agent"
version = "4.7.5-dev.9"
EOF

export HOME="$fixture/home"
export PATH="$fixture/bin:/usr/bin:/bin"
export CARGO_CALLS="$fixture/cargo.calls"
export CARGO_STATE="$fixture/cargo.state"
export CODETETHER_CALLS="$fixture/codetether.calls"

"$fixture/scripts/install-dev.sh" >"$fixture/install.out"
grep -q '^version = "4.7.5-dev.10"$' "$fixture/Cargo.toml"
grep -q '^version = "4.7.5-dev.10"$' "$fixture/Cargo.lock"
[[ $(grep -c '^install --path \. --force$' "$CARGO_CALLS") -eq 2 ]]
[[ $(wc -l <"$CODETETHER_CALLS") -eq 1 ]]

"$fixture/scripts/install-dev.sh" test --lib >"$fixture/test.out"
grep -q '^version = "4.7.5-dev.11"$' "$fixture/Cargo.toml"
grep -q '^version = "4.7.5-dev.11"$' "$fixture/Cargo.lock"
[[ $(tail -1 "$CARGO_CALLS") == 'test --lib' ]]
