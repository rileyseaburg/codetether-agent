"""Correct shared workflow state and response rejection semantics."""

from pathlib import Path

ROOT = Path("/home/riley/spotlessbinco/tests/playwright/codetether")
SOURCE = ROOT / "pr-review-source.ts"
RUNNER = ROOT / "pr-review-response-runner.ts"
CONSTANT = 'const workflowPath = ".forgejo/workflows/codetether-pr-review.yml";\n'
RUNNER_CONTENT = '''import { execFile } from "node:child_process";
import { promisify } from "node:util";

const execFileAsync = promisify(execFile);
const responsePath = "scripts/codetether-review/response.sh";
const setup = `source "$1"; input="$(mktemp)"; output="$(mktemp)";
  trap 'rm -f "$input" "$output"' EXIT; printf '%s' "$2" > "$input";
  normalize_review_response "$input" "$output";`;
const normalizeCommand = `set -euo pipefail; ${setup}
  printf '%s:%s' "$(extract_review_markdown "$output")" "$(extract_blocking_findings "$output")"`;
const rejectCommand = `${setup} extract_blocking_findings "$output"`;

async function execute(command: string, body: string): Promise<string> {
  return (await execFileAsync("bash", ["-c", command, "bash", responsePath, body])).stdout;
}

export async function normalize(body: string): Promise<string> {
  return execute(normalizeCommand, body);
}

export async function rejects(body: string): Promise<boolean> {
  return execute(rejectCommand, body).then(
    () => false,
    () => true,
  );
}
'''


def main() -> None:
    """Restore the workflow constant and focused runner commands."""
    text = SOURCE.read_text()
    if CONSTANT not in text:
        anchor = 'export { scriptPaths } from "./pr-review-script-paths";\n'
        if text.count(anchor) != 1:
            raise RuntimeError('Playwright workflow constant anchor is missing')
        text = text.replace(anchor, anchor + '\n' + CONSTANT, 1)
        SOURCE.write_text(text)
    RUNNER.write_text(RUNNER_CONTENT)


if __name__ == "__main__":
    main()