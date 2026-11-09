#!/usr/bin/env bash
# Fetch a pinned Forgejo action archive, avoiding a full remote action-history clone.
set -euo pipefail
[[ ${CI:-} == true ]] || { echo 'Forgejo CI execution required' >&2; exit 1; }
mkdir -p .ci-actions/upload-artifact
curl -fsSL --retry 3 https://code.forgejo.org/actions/upload-artifact/archive/c24449f33cd45d4826c6702db7e49f7cdb9b551d.tar.gz -o /tmp/bonsai-upload-action.tgz
echo '390c228f0fd08dc3f636eaa84f66af9c62c0ec9931fc15953a8cedc6e4f758f9  /tmp/bonsai-upload-action.tgz' | sha256sum -c -
tar -xzf /tmp/bonsai-upload-action.tgz -C .ci-actions/upload-artifact --strip-components=1
test -f .ci-actions/upload-artifact/action.yml
test -f .ci-actions/upload-artifact/dist/index.js