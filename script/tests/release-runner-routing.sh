#!/usr/bin/env bash
# Keep Linux-side release work off the shared Kubernetes Docker pool.
set -euo pipefail
root=$(cd "$(dirname "$0")/../.." && pwd)
workflows="$root/.forgejo/workflows"
grep -Fxq '      runner: codetether-release-proxmox' "$workflows/release.yml"
for name in verify windows publish; do
  grep -Fxq '    runs-on: codetether-release-proxmox' "$workflows/release-$name.yml"
done
if grep -q 'spotlessbinco-k8s' "$workflows"/release*.yml; then
  echo 'Release workflows must not select shared Kubernetes runners' >&2
  exit 1
fi
grep -Fxq '      DOCKER_HOST: unix:///var/run/docker.sock' "$workflows/release-windows.yml"
grep -Fxq '      runner: macOS' "$workflows/release.yml"
grep -Fxq '    runs-on: macOS' "$workflows/release-meta.yml"
awk '/^  linux:/,/^  windows:/' "$workflows/release.yml" \
  | grep -Fxq '    needs: [meta, verify, windows]'
awk '/^  windows:/,/^  macos:/' "$workflows/release.yml" \
  | grep -Fxq '    needs: [meta, verify]'
awk '/^  macos:/,/^  verify:/' "$workflows/release.yml" \
  | grep -Fxq '    needs: [meta, verify]'
awk '/^  verify:/,/^  publish:/' "$workflows/release.yml" \
  | grep -Fxq '    needs: meta'
grep -Fxq '    needs: [meta, verify, linux, windows, macos]' "$workflows/release.yml"
grep -Fq 'cargo +1.95.0 test --locked --lib --tests --no-fail-fast' "$workflows/release-verify.yml"
grep -Fq 'bash script/tests/release-runner-routing.sh' "$workflows/release-verify.yml"
grep -Fq 'perl -MFindBin -MIPC::Cmd -e 1 && make --version' "$root/docker/release/windows.Dockerfile"
printf '%s\n' 'static/local: release runner routing and publication gates passed'