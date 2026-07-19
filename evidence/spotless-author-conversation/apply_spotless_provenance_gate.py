"""Require complete CodeTether provenance before workflow handoff."""

from pathlib import Path

ROOT = Path('/home/riley/spotlessbinco')
PROVENANCE = ROOT / 'scripts/codetether-review/author_provenance.sh'
STATE = ROOT / 'scripts/codetether-review/author_state.sh'
IDENTITY = ROOT / 'scripts/codetether-review/author_identity.sh'
MODULES = ROOT / 'scripts/codetether-review/modules.sh'
TEST = ROOT / 'tests/codetether-review-author-protocol.test.sh'
CONTENT = '''#!/usr/bin/env bash

load_author_provenance() {
  local head="$1"
  AUTHOR_KEY_ID="$(commit_trailer "$head" CodeTether-Key-ID)"
  AUTHOR_TENANT_ID="$(commit_trailer "$head" CodeTether-Tenant-ID)"
  AUTHOR_CODETETHER_SIGNATURE="$(commit_trailer "$head" CodeTether-Signature)"
  [[ "$AUTHOR_KEY_ID" =~ ^[A-Za-z0-9][A-Za-z0-9._-]{0,127}$ ]] || return 1
  [[ "$AUTHOR_TENANT_ID" =~ ^[A-Za-z0-9][A-Za-z0-9._-]{0,127}$ ]] || return 1
  [[ "$AUTHOR_CODETETHER_SIGNATURE" =~ ^[0-9a-fA-F]{64}$ ]]
}
'''


def main() -> None:
    """Write provenance preflight and integrate it into author discovery."""
    PROVENANCE.write_text(CONTENT)
    state = STATE.read_text()
    anchor = '  AUTHOR_PROVENANCE_ID=""\n'
    fields = '''  AUTHOR_PROVENANCE_ID=""
  AUTHOR_KEY_ID=""
  AUTHOR_TENANT_ID=""
  AUTHOR_CODETETHER_SIGNATURE=""
'''
    if '  AUTHOR_KEY_ID=""\n' not in state:
        state = state.replace(anchor, fields, 1)
    STATE.write_text(state)
    identity = IDENTITY.read_text()
    anchor = '''  if [[ ! "$AUTHOR_SESSION_ID" =~ ^[A-Za-z0-9_-]{1,128}$ ||
        ! "$AUTHOR_PROVENANCE_ID" =~ ^ctprov_[A-Za-z0-9_-]{16,80}$ ]]; then
'''
    provenance_gate = '''  if ! load_author_provenance "$head"; then
    log "Author delivery disabled: CodeTether provenance proof is incomplete."
    clear_author_identity
    return 0
  fi
'''
    if provenance_gate not in identity:
        if identity.count(anchor) != 1:
            raise RuntimeError('author provenance workflow anchor is missing')
        identity = identity.replace(anchor, provenance_gate + anchor, 1)
    IDENTITY.write_text(identity)
    modules = MODULES.read_text()
    source = 'source "$review_modules_dir/author_provenance.sh"\n'
    if source not in modules:
        anchor = 'source "$review_modules_dir/author_signature.sh"\n'
        modules = modules.replace(anchor, anchor + source, 1)
    MODULES.write_text(modules)
    test = TEST.read_text()
    source = 'source "$repo_root/scripts/codetether-review/author_provenance.sh"\n'
    if source not in test:
        anchor = 'source "$repo_root/scripts/codetether-review/author_signature.sh"\n'
        test = test.replace(anchor, anchor + source, 1)
    trailers = '''CodeTether-Agent-Slot: default
CodeTether-Agent-Name: author
CodeTether-Origin: worker
CodeTether-Tenant-ID: tenant
CodeTether-Key-ID: author-key
CodeTether-Signature: aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
'''
    test = test.replace('CodeTether-Agent-Slot: default\n', trailers)
    TEST.write_text(test)


if __name__ == '__main__':
    main()