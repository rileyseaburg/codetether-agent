#!/usr/bin/env bash
set -euo pipefail

root=$(cd "$(dirname "$0")/../.." && pwd)
source_helper="$root/script/bump-dev-version.sh"
fixture=$(mktemp -d)
trap 'rm -rf "$fixture"' EXIT
mkdir -p "$fixture/script"
cp "$source_helper" "$fixture/script/bump-dev-version.sh"
cp "$root/script/set-dev-version.sh" "$fixture/script/set-dev-version.sh"
chmod +x "$fixture/script/bump-dev-version.sh"
chmod +x "$fixture/script/set-dev-version.sh"

cat >"$fixture/Cargo.toml" <<'EOF'
[workspace.package]
version = "99.0.0"

[package]
name = "codetether-agent"
version = "4.7.5-dev.9"

[package.metadata.test]
version = "unchanged"
EOF
cat >"$fixture/Cargo.lock" <<'EOF'
[[package]]
name = "another-package"
version = "1.0.0"

[[package]]
name = "codetether-agent"
version = "4.7.5-dev.9"
EOF

actual=$(cd "$fixture" && ./script/bump-dev-version.sh)
[[ $actual == 4.7.5-dev.10 ]]
grep -q '^version = "4.7.5-dev.10"$' "$fixture/Cargo.toml"
grep -q '^version = "4.7.5-dev.10"$' "$fixture/Cargo.lock"
grep -q '^version = "99.0.0"$' "$fixture/Cargo.toml"
grep -q '^version = "unchanged"$' "$fixture/Cargo.toml"
grep -q '^version = "1.0.0"$' "$fixture/Cargo.lock"

sed -i 's/4\.7\.5-dev\.10/4.7.5/' "$fixture/Cargo.toml"
if (cd "$fixture" && ./script/bump-dev-version.sh >/dev/null 2>&1); then
  echo 'stable versions must be rejected' >&2
  exit 1
fi
grep -q '^version = "4.7.5-dev.10"$' "$fixture/Cargo.lock"
