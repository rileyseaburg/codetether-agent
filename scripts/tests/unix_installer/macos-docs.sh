#!/usr/bin/env bash
# Run the documented manual install with mocked downloads and isolated HOME.
set -euo pipefail
root=$(cd "$(dirname "$0")/../../.." && pwd)
evidence=${1:?Provide a retained evidence directory}
mkdir -p "$evidence"
evidence=$(cd "$evidence" && pwd)
awk '
  /^### Manual, pinned-version install without sudo$/ { section=1 }
  section && /^```sh$/ { block=1; next }
  block && /^```$/ { exit }
  block { print }
' "$root/docs/install_unix.md" > "$evidence/manual.sh"
test -s "$evidence/manual.sh"
sh -n "$evidence/manual.sh"
version=$(sed -n 's/^  version=//p' "$evidence/manual.sh")
test -n "$version"
for mode in arm64 intel rosetta corrupt; do
  case_dir="$evidence/$mode"
  mkdir -p "$case_dir"/{bin,home,tmp,archive}
  target=aarch64-apple-darwin
  [[ $mode != intel ]] || target=x86_64-apple-darwin
  printf '#!/bin/sh\nprintf "codetether %s\\n"\n' "${version#v}" > "$case_dir/archive/codetether"
  chmod +x "$case_dir/archive/codetether"
  tar -czf "$case_dir/package.tar.gz" -C "$case_dir/archive" codetether
  hash=$(shasum -a 256 "$case_dir/package.tar.gz"); hash=${hash%% *}
  [[ $mode != corrupt ]] || hash=$(printf '%064d' 0)
  printf '%s  codetether-%s-%s.tar.gz\n' "$hash" "$version" "$target" > "$case_dir/manifest"
  cat > "$case_dir/bin/uname" <<'MOCK'
#!/bin/sh
case "$MODE" in arm64|corrupt) echo arm64 ;; *) echo x86_64 ;; esac
MOCK
  cat > "$case_dir/bin/sysctl" <<'MOCK'
#!/bin/sh
[ "$MODE" = rosetta ] && echo 1
exit 0
MOCK
  cat > "$case_dir/bin/curl" <<'MOCK'
#!/bin/sh
set -eu
test "$1" = -fSL && test "$3" = -o
base="https://forgejo.quantum-forge.io/riley/codetether-agent/releases/download/$VERSION"
case "$2" in
  "$base/codetether-$VERSION-$TARGET.tar.gz") cp "$CASE_DIR/package.tar.gz" "$4" ;;
  "$base/SHA256SUMS-$VERSION.txt") cp "$CASE_DIR/manifest" "$4" ;;
  *) echo "Unexpected download: $2" >&2; exit 1 ;;
esac
MOCK
  chmod +x "$case_dir/bin/"*
  status=0
  MODE="$mode" CASE_DIR="$case_dir" VERSION="$version" TARGET="$target" \
    HOME="$case_dir/home" TMPDIR="$case_dir/tmp" PATH="$case_dir/bin:$PATH" \
    sh "$evidence/manual.sh" > "$case_dir/output.log" 2>&1 || status=$?
  if [[ $mode == corrupt ]]; then
    [[ $status != 0 ]]
    test ! -e "$case_dir/home/.local/bin/codetether"
  else
    [[ $status == 0 ]] || { cat "$case_dir/output.log"; exit 1; }
    [[ $("$case_dir/home/.local/bin/codetether" --version) == "codetether ${version#v}" ]]
    test ! -e "$case_dir/home/.zshrc"
  fi
done
printf 'mocked local: documented ARM64, Intel, Rosetta installs and corrupt-download rejection; evidence: %s\n' "$evidence"
