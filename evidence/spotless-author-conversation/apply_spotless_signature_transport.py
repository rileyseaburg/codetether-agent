"""Use the existing portable Forgejo HTTP transport for signature proof."""

from pathlib import Path

SIGNATURE = Path("/home/riley/spotlessbinco/scripts/codetether-review/author_signature.sh")
BOOTSTRAP = Path("/home/riley/spotlessbinco/scripts/codetether-review/review_bootstrap.sh")
TEST = Path("/home/riley/spotlessbinco/tests/codetether-review-author-protocol.test.sh")


def replace_once(path: Path, old: str, new: str) -> None:
    text = path.read_text()
    if new in text:
        return
    if text.count(old) != 1:
        raise RuntimeError(f"signature transport anchor missing: {path}")
    path.write_text(text.replace(old, new, 1))


def main() -> None:
    """Replace the extra CLI dependency and its fixture."""
    replace_once(
        SIGNATURE,
        '''  timeout --foreground "${CODETETHER_HTTP_TIMEOUT_SECONDS:-60}s" \\
    forgejo-cli --base "$api" --token "$FORGEJO_TOKEN" get \\
    "/repos/$REPO_FULL/git/commits/$head" > "$output" || return 1
''',
        '''  forgejo_request GET "$api/repos/$REPO_FULL/git/commits/$head" \\
    "$FORGEJO_TOKEN" "" "$output" || return 1
''',
    )
    replace_once(
        BOOTSTRAP,
        '''  require_command forgejo-cli
  require_command sha256sum
  require_command timeout
''',
        '''  require_command sha256sum
''',
    )
    replace_once(
        TEST,
        '''mkdir -p "$tmp/bin"
cat > "$tmp/bin/forgejo-cli" <<'SCRIPT'
#!/usr/bin/env bash
jq -n --arg sha "$FORGEJO_TEST_SHA" '{sha:$sha,
  commit:{verification:{verified:true,signer:{username:"Alice"}}}}'
SCRIPT
chmod +x "$tmp/bin/forgejo-cli"
export PATH="$tmp/bin:$PATH" FORGEJO_TEST_SHA="$HEAD_SHA"
''',
        '''forgejo_request() {
  jq -n --arg sha "$FORGEJO_TEST_SHA" '{sha:$sha,
    commit:{verification:{verified:true,signer:{username:"Alice"}}}}' > "$5"
}
export FORGEJO_TEST_SHA="$HEAD_SHA"
''',
    )


if __name__ == "__main__":
    main()