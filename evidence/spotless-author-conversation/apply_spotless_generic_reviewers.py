"""Replace repository-specific reviewer selection with configured principals."""

from pathlib import Path

SCRIPT = Path("/home/riley/spotlessbinco/scripts/codetether-review/approval_credentials.sh")
WORKFLOW = Path("/home/riley/spotlessbinco/.forgejo/workflows/codetether-pr-review.yml")
TEST = Path("/home/riley/spotlessbinco/tests/codetether-review-approval-credentials.test.sh")
CONTENT = '''#!/usr/bin/env bash

load_forgejo_approval_credentials() {
  local event_path="$1" author candidates entry vault_path
  if [[ -z "$event_path" || ! -f "$event_path" ]]; then
    echo "Cannot select Forgejo reviewer: pull-request event is missing." >&2
    return 1
  fi
  author="$(jq -er '.pull_request.user.login |
    select(type == "string" and length > 0)' "$event_path")" || {
    echo "Cannot select Forgejo reviewer: PR author is missing." >&2
    return 1
  }
  candidates="${CODETETHER_REVIEWER_CANDIDATES:-}"
  approval_actor=""
  vault_path=""
  IFS=',' read -ra entries <<< "$candidates"
  for entry in "${entries[@]}"; do
    [[ "$entry" == *=* ]] || continue
    local actor="${entry%%=*}" path="${entry#*=}"
    [[ "$actor" =~ ^[A-Za-z0-9._-]{1,128}$ ]] || continue
    [[ "$path" =~ ^[A-Za-z0-9_./-]{1,256}$ ]] || continue
    if [[ "${actor,,}" != "${author,,}" ]]; then
      approval_actor="$actor"
      vault_path="$path"
      break
    fi
  done
  if [[ -z "$approval_actor" ]]; then
    echo "No configured Forgejo reviewer is independent of $author." >&2
    return 1
  fi
  approval_token="$(vault kv get -field=token "$vault_path")" || {
    echo "Cannot load Forgejo reviewer credential for $approval_actor." >&2
    return 1
  }
  unset VAULT_TOKEN
  [[ -n "$approval_token" ]] || {
    echo "Forgejo reviewer credential for $approval_actor is empty." >&2
    return 1
  }
}

clear_forgejo_approval_credentials() {
  approval_token=""
  approval_actor=""
  unset approval_token approval_actor
}
'''


def insert_once(path: Path, anchor: str, addition: str) -> None:
    text = path.read_text()
    if addition.strip() in text:
        return
    if text.count(anchor) != 1:
        raise RuntimeError(f"anchor missing or ambiguous: {path}")
    path.write_text(text.replace(anchor, anchor + addition, 1))


def main() -> None:
    """Write the generic selector and configure this repository's candidates."""
    SCRIPT.write_text(CONTENT)
    insert_once(
        WORKFLOW,
        "      CODETETHER_REVIEW_AUTO_APPROVE: 'true'\n",
        "      CODETETHER_REVIEWER_CANDIDATES: 'codetether-bot=kv/forgejo/codetether-bot,spotless-agent=kv/forgejo/spotlessbinco-agent'\n",
    )
    insert_once(
        TEST,
        'source "$repo_root/scripts/codetether-review/approval_credentials.sh"\n',
        "export CODETETHER_REVIEWER_CANDIDATES='codetether-bot=kv/forgejo/codetether-bot,spotless-agent=kv/forgejo/spotlessbinco-agent'\n",
    )


if __name__ == "__main__":
    main()