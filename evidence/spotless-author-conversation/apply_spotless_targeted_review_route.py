"""Route configured reviewers through the targeted task protocol."""

from pathlib import Path

ROOT = Path('/home/riley/spotlessbinco')
BOOTSTRAP = ROOT / 'scripts/codetether-review/review_bootstrap.sh'
WORKFLOW = ROOT / '.forgejo/workflows/codetether-pr-review.yml'
TEST = ROOT / 'tests/codetether-review-bootstrap.test.sh'
BOOTSTRAP_CONTENT = '''#!/usr/bin/env bash

configure_review_environment() {
  CODETETHER_URL="${CODETETHER_JSON_URL:-}"
  if [[ -n "${CODETETHER_REVIEW_TARGET_AGENT:-}" ]]; then
    if [[ -n "${CODETETHER_API_URL:-}" ]]; then
      CODETETHER_URL="${CODETETHER_API_URL%/}/v1/agent/tasks"
    elif [[ "$CODETETHER_URL" != */v1/agent/tasks ]]; then
      echo "Targeted CodeTether reviews require the task API endpoint." >&2
      return 1
    fi
  elif [[ -z "$CODETETHER_URL" && -n "${CODETETHER_API_URL:-}" ]]; then
    CODETETHER_URL="${CODETETHER_API_URL%/}/v1/json"
  fi
  if [[ -z "$CODETETHER_URL" || -z "${CODETETHER_TOKEN:-}" ]]; then
    echo "CodeTether review failed: API URL and token must be configured." >&2
    return 1
  fi
  require_command jq
  require_command curl
  require_command python3
  require_command forgejo-cli
  require_command sha256sum
  require_command timeout
  EVENT_PATH="${GITHUB_EVENT_PATH:-}"
  if [[ -z "$EVENT_PATH" || ! -f "$EVENT_PATH" ]]; then
    echo "GITHUB_EVENT_PATH is missing; cannot determine PR context." >&2
    return 1
  fi
}
'''
TEST_CONTENT = '''#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
source "$ROOT/scripts/codetether-review/review_bootstrap.sh"
require_command() { :; }
event="$(mktemp)"
trap 'rm -f "$event"' EXIT
printf '{}\n' > "$event"
export GITHUB_EVENT_PATH="$event" CODETETHER_TOKEN=test-token

CODETETHER_API_URL=https://api.example CODETETHER_JSON_URL= \
  CODETETHER_REVIEW_TARGET_AGENT=reviewer configure_review_environment
[[ "$CODETETHER_URL" == https://api.example/v1/agent/tasks ]]

CODETETHER_API_URL=https://api.example CODETETHER_JSON_URL= \
  CODETETHER_REVIEW_TARGET_AGENT= configure_review_environment
[[ "$CODETETHER_URL" == https://api.example/v1/json ]]

if CODETETHER_API_URL= CODETETHER_JSON_URL=https://api.example/v1/json \
  CODETETHER_REVIEW_TARGET_AGENT=reviewer configure_review_environment; then
  echo 'Targeted review unexpectedly accepted the JSON endpoint.' >&2
  exit 1
fi
echo 'targeted reviewer routing tests passed'
'''


def main() -> None:
    """Install targeted routing and its workflow-time validation."""
    BOOTSTRAP.write_text(BOOTSTRAP_CONTENT)
    TEST.write_text(TEST_CONTENT)
    text = WORKFLOW.read_text()
    entry = '          bash tests/codetether-review-bootstrap.test.sh\n'
    if entry not in text:
        anchor = '          bash tests/codetether-review-response.test.sh\n'
        text = text.replace(anchor, anchor + entry, 1)
    WORKFLOW.write_text(text)


if __name__ == '__main__':
    main()