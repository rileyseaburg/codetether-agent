"""Add Forgejo principal trailers to the installed commit-msg hook."""

from pathlib import Path

PATH = Path("/home/riley/A2A-Server-MCP/codetether-agent/src/provenance/hook_script.rs")
HOST_ANCHOR = '''git_cfg() {
  git config --local --get \"$1\" 2>/dev/null || true
}
'''
HOST_ADDITION = '''forgejo_host=\"$CODETETHER_FORGEJO_HOST\"
forgejo_host=\"${forgejo_host#https://}\"
forgejo_host=\"${forgejo_host#http://}\"
forgejo_host=\"${forgejo_host%%/*}\"
'''
IDENTITY_ANCHOR = '''add_trailer \"CodeTether-Agent-Identity\" \"$CODETETHER_AGENT_IDENTITY_ID\"
'''
IDENTITY_ADDITION = '''add_trailer \"CodeTether-Forgejo-Host\" \"$forgejo_host\"
add_trailer \"CodeTether-Forgejo-Login\" \"$CODETETHER_FORGEJO_LOGIN\"
add_trailer \"CodeTether-Agent-Slot\" \"${CODETETHER_AGENT_SLOT:-default}\"
'''

HOST_ANCHOR = HOST_ANCHOR.replace('"', '\\"')
HOST_ADDITION = HOST_ADDITION.replace('"', '\\"')
IDENTITY_ANCHOR = IDENTITY_ANCHOR.replace('"', '\\"')
IDENTITY_ADDITION = IDENTITY_ADDITION.replace('"', '\\"')


def insert(anchor: str, addition: str) -> None:
    text = PATH.read_text()
    if addition in text:
        return
    if text.count(anchor) != 1:
        raise RuntimeError("hook trailer anchor missing or ambiguous")
    PATH.write_text(text.replace(anchor, anchor + addition, 1))


def main() -> None:
    """Install canonical host normalization and all principal labels."""
    insert(HOST_ANCHOR, HOST_ADDITION)
    insert(IDENTITY_ANCHOR, IDENTITY_ADDITION)


if __name__ == "__main__":
    main()