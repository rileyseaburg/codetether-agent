"""Split the external Playwright source contract into focused modules."""

from pathlib import Path

ROOT = Path("/home/riley/spotlessbinco/tests/playwright/codetether")
FILES = {
    'pr-review-workflow.spec.ts': '''import "./pr-review-async.cases";
import "./pr-review-response.cases";
import "./pr-review-security.cases";
import "./pr-review-shell.cases";
''',
    'pr-review-source.ts': '''import { readFile } from "node:fs/promises";

export const workflowPath = ".forgejo/workflows/codetether-pr-review.yml";
export const scriptPaths = [
  "scripts/codetether-pr-review.sh",
  "scripts/codetether-review/common.sh",
  "scripts/codetether-review/modules.sh",
  "scripts/codetether-review/review_bootstrap.sh",
  "scripts/codetether-review/review_orchestration.sh",
  "scripts/codetether-review/review_publish.sh",
  "scripts/codetether-review/context.sh",
  "scripts/codetether-review/diff.sh",
  "scripts/codetether-review/chunks.sh",
  "scripts/codetether-review/chunk_pipeline.sh",
  "scripts/codetether-review/payload.sh",
  "scripts/codetether-review/request_async.sh",
  "scripts/codetether-review/request_sync.sh",
  "scripts/codetether-review/response.sh",
  "scripts/codetether-review/forgejo.sh",
  "scripts/codetether-review/approval_state.sh",
  "scripts/codetether-review/approval_input.sh",
  "scripts/codetether-review/approval_create.sh",
  "scripts/codetether-review/approval.sh",
  "scripts/codetether-review/approval_credentials.sh",
  "scripts/codetether-review/author_signature.sh",
  "scripts/codetether-review/author_guard.sh",
  "scripts/codetether-review/author_principal.sh",
  "scripts/codetether-review/author_state.sh",
  "scripts/codetether-review/author_identity.sh",
  "scripts/codetether-review/author_payload.sh",
  "scripts/codetether-review/task_create.sh",
  "scripts/codetether-review/author_delivery.sh",
];

export const readWorkflow = (): Promise<string> => readFile(workflowPath, "utf8");
export async function readScripts(): Promise<string> {
  const sources = await Promise.all(scriptPaths.map((path) => readFile(path, "utf8")));
  return sources.join("\\n");
}
''',
    'pr-review-security.cases.ts': '''import { expect, test } from "@playwright/test";
import { readFile } from "node:fs/promises";
import { readScripts, readWorkflow } from "./pr-review-source";

test.describe("CodeTether PR review security", () => {
  test("fails loudly when review credentials are missing", async () => {
    const [workflow, script] = await Promise.all([readWorkflow(), readScripts()]);
    expect(workflow).not.toContain("continue-on-error: true");
    expect(script).toContain("CodeTether review failed:");
    expect(script).toContain("--rawfile diff");
    expect(script).toContain("/v1/agent/tasks");
    expect(script).toContain("X-Forgejo-Token");
    expect(script).not.toContain('blocking_findings // 0');
  });

  test("requires structured counts before a formal approval", async () => {
    const payload = await readFile("scripts/codetether-review/payload.sh", "utf8");
    const response = await readFile("scripts/codetether-review/response.sh", "utf8");
    const approval = await readScripts();
    expect(payload).toContain("Return exactly one JSON object");
    expect(payload).toContain('"blocking_findings":0');
    expect(payload).toContain('"reviewed_chunk_sha256":"hex"');
    const chunks = await readFile("scripts/codetether-review/chunks.sh", "utf8");
    expect(chunks).toContain(".reviewed_chunk_sha256 == $sha");
    expect(response).toContain('select(type == "number" and . >= 0 and floor == .)');
    expect(approval).toContain("if (( blocking != 0 ))");
    expect(approval).toContain("A Forgejo approval credential is required");
    expect(approval).toContain("unset VAULT_TOKEN");
    expect(approval).toContain("current_head");
  });
});
''',
    'pr-review-async.cases.ts': '''import { expect, test } from "@playwright/test";
import { readScripts, readWorkflow } from "./pr-review-source";

test("waits for a truthful async terminal state", async () => {
  const [workflow, script] = await Promise.all([readWorkflow(), readScripts()]);
  expect(workflow).toContain("CODETETHER_TASK_POLL_ATTEMPTS: '360'");
  expect(workflow).toContain("CODETETHER_REVIEW_TARGET_AGENT: pr-review-worker");
  expect(workflow).toContain("pull_request_target:");
  expect(workflow).toContain("CODETETHER_REVIEW_MODEL: openai-codex/gpt-5.6-sol-fast:high");
  expect(workflow).toContain("CODETETHER_REVIEW_AUTO_APPROVE: 'true'");
  expect(workflow).toContain("CODETETHER_REVIEW_DIFF_MAX_BYTES: '1500000'");
  expect(workflow).toContain("CODETETHER_REVIEW_CHUNK_MAX_BYTES: '200000'");
  expect(workflow).toContain("CODETETHER_REVIEW_PROMPT_MAX_BYTES: '240000'");
  expect(workflow).toContain("CODETETHER_REVIEW_TASK_MAX_BYTES: '900000'");
  expect(script).toContain("CODETETHER_REVIEW_DIFF_MAX_BYTES:-1500000");
  expect(script).toContain("CODETETHER_REVIEW_CHUNK_MAX_BYTES:-200000");
  expect(script).toContain("CODETETHER_REVIEW_PROMPT_MAX_BYTES:-240000");
  expect(script).toContain("CODETETHER_REVIEW_TASK_MAX_BYTES:-900000");
  expect(workflow).toContain("FORGEJO_TOKEN: ${{ github.token }}");
  expect(workflow).not.toContain("${{ secrets.FORGEJO_TOKEN }}");
  expect(script).toContain('load_forgejo_approval_credentials "$EVENT_PATH"');
  expect(script).toContain('CODETETHER_FORGEJO_APPROVAL_ACTOR="$approval_actor"');
  expect(script).toContain("clear_forgejo_approval_credentials");
  expect(workflow).toContain("bash tests/codetether-review-approval.test.sh");
  expect(workflow).toContain("bash tests/codetether-review-author-protocol.test.sh");
  expect(workflow).toContain("bash tests/codetether-review-task-create.test.sh");
  expect(script).toContain('event:"APPROVED"');
  expect(script).toContain("current_head");
  expect(script).toContain("CODETETHER_TASK_POLL_ATTEMPTS:-360");
  expect(script).toContain("target_agent_name:$target_agent");
  expect(script).toContain("task_timeout_seconds:$task_timeout");
  expect(script).toContain("codetether.forgejo-author.v1");
  expect(script).toContain("CodeTether-Forgejo-Login");
  expect(script).toContain("Idempotency-Key:");
  expect(script).toContain("route_author_followup");
  expect(script).not.toContain("git verify-commit");
  expect(script).toContain("{model:$model}");
  expect(script).toContain("CodeTether task failed with status $task_status");
  expect(script).toContain("did not finish after $attempts poll attempts");
});
''',
    'pr-review-response-runner.ts': '''import { execFile } from "node:child_process";
import { promisify } from "node:util";

const execFileAsync = promisify(execFile);
const responsePath = "scripts/codetether-review/response.sh";
const command = `
  set -euo pipefail; source "$1"; input="$(mktemp)"; output="$(mktemp)";
  trap 'rm -f "$input" "$output"' EXIT; printf '%s' "$2" > "$input";
  normalize_review_response "$input" "$output";
  printf '%s:%s' "$(extract_review_markdown "$output")" "$(extract_blocking_findings "$output")"`;

export async function normalize(body: string): Promise<string> {
  return (await execFileAsync("bash", ["-c", command, "bash", responsePath, body])).stdout;
}

export async function rejects(body: string): Promise<boolean> {
  return execFileAsync("bash", ["-c", command, "bash", responsePath, body]).then(
    () => false,
    () => true,
  );
}
''',
    'pr-review-response.cases.ts': '''import { expect, test } from "@playwright/test";
import { normalize, rejects } from "./pr-review-response-runner";

test("normalizes async JSON and rejects a missing blocking count", async () => {
  const valid = '{"result":"{\\"markdown\\":\\"ready\\",\\"blocking_findings\\":2}"}';
  expect(await normalize(valid)).toBe("ready:2");
  const prefixed = '{"result":"analysis before result\\n{\\"markdown\\":\\"ready\\",\\"blocking_findings\\":4}"}';
  expect(await normalize(prefixed)).toBe("ready:4");
  const trailing = '{"result":"{\\"markdown\\":\\"ready\\",\\"blocking_findings\\":0,\\"nonblocking_findings\\":0}\\nCommitted"}';
  expect(await normalize(trailing)).toBe("ready:0");
  const ambiguous = '{"result":"{\\"markdown\\":\\"one\\",\\"blocking_findings\\":0} then {\\"markdown\\":\\"two\\",\\"blocking_findings\\":0}"}';
  expect(await rejects(ambiguous)).toBe(true);
  expect(await rejects('{"markdown":"unsafe"}')).toBe(true);
});
''',
    'pr-review-shell.cases.ts': '''import { expect, test } from "@playwright/test";
import { execFile } from "node:child_process";
import { readFile } from "node:fs/promises";
import { promisify } from "node:util";
import { scriptPaths } from "./pr-review-source";

const execFileAsync = promisify(execFile);

test("keeps every hand-written shell module valid and under 50 lines", async () => {
  for (const path of scriptPaths) {
    const source = await readFile(path, "utf8");
    const codeLines = source.split("\\n").filter((line) => {
      const trimmed = line.trim();
      return trimmed.length > 0 && !trimmed.startsWith("#");
    });
    expect(codeLines.length).toBeLessThanOrEqual(50);
    expect((await execFileAsync("bash", ["-n", path])).stderr).toBe("");
  }
});
''',
}


def main() -> None:
    """Write the focused source-contract modules."""
    for name, content in FILES.items():
        (ROOT / name).write_text(content)


if __name__ == "__main__":
    main()