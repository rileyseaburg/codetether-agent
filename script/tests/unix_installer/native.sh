#!/usr/bin/env bash
# Mocked-local archive/layout tests without Python or machine/profile writes.
set -euo pipefail
root=$(cd "$(dirname "$0")/../../.." && pwd)
evidence=${1:?Provide a retained evidence directory}
run_case() {
  local platform=$1 layout=$2 case_dir="$evidence/$1-$2" filename=codetether
  mkdir -p "$case_dir"/{archive,bin,installed}
  [[ $layout != versioned ]] || filename="codetether-v4.7.5-$platform"
  [[ $layout != missing ]] || filename=wrong-file
  printf '#!/bin/sh\nprintf "codetether 4.7.5\\n"\n' > "$case_dir/archive/$filename"
  chmod +x "$case_dir/archive/$filename"
  tar -czf "$case_dir/release.tar.gz" -C "$case_dir/archive" "$filename"
  for target in "$case_dir/bin/codetether" "$case_dir/installed/codetether"; do
    printf '#!/bin/sh\nprintf "codetether 1.0.0\\n"\n' > "$target"
    chmod +x "$target"
  done
  sed '$d' "$root/install.sh" > "$case_dir/functions.sh"
  local status=0
  CASE_DIR="$case_dir" INSTALL_TEST_ARCHIVE="$case_dir/release.tar.gz" \
    INSTALL_TEST_PLATFORM="$platform" PATH="$case_dir/bin:$PATH" \
    sh "$root/script/tests/unix_installer/harness.sh" > "$case_dir/installer.log" 2>&1 || status=$?
  if [[ $layout == missing ]]; then
    [[ $status != 0 ]]
    [[ $("$case_dir/installed/codetether" --version) == 'codetether 1.0.0' ]]
  else
    [[ $status == 0 ]] || { cat "$case_dir/installer.log"; return 1; }
    grep -q 'PATH resolves' "$case_dir/installer.log"
    [[ $("$case_dir/installed/codetether" --version) == 'codetether 4.7.5' ]]
  fi
}
for platform in x86_64-unknown-linux-gnu aarch64-apple-darwin x86_64-apple-darwin; do
  for layout in bare versioned; do run_case "$platform" "$layout"; done
done
run_case x86_64-unknown-linux-gnu missing
echo 'mocked local: 6 archive/target combinations and missing-binary preservation'
