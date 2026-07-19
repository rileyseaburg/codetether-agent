"""Split workflow configuration assertions from async runtime assertions."""

from pathlib import Path

ROOT = Path("/home/riley/spotlessbinco/tests/playwright/codetether")
ASYNC = ROOT / "pr-review-async.cases.ts"
CONFIG = ROOT / "pr-review-config.cases.ts"
SPEC = ROOT / "pr-review-workflow.spec.ts"
ASYNC_CONTENT = '''import { expect, test } from "@playwright/test";
import { readScripts } from "./pr-review-source";

test("waits for a truthful async terminal state", async () => {
  const script = await readScripts();
  expect(script).toContain("CODETETHER_REVIEW_DIFF_MAX_BYTES:-1500000");
  expect(script).toContain("CODETETHER_REVIEW_CHUNK_MAX_BYTES:-200000");
  expect(script).toContain("CODETETHER_REVIEW_PROMPT_MAX_BYTES:-240000");
  expect(script).toContain("CODETETHER_REVIEW_TASK_MAX_BYTES:-900000");
  expect(script).toContain('load_forgejo_approval_credentials "$EVENT_PATH"');
  expect(script).toContain('CODETETHER_FORGEJO_APPROVAL_ACTOR="$approval_actor"');
  expect(script).toContain("clear_forgejo_approval_credentials");
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
'''
CONFIG_CONTENT = '''import { expect, test } from "@playwright/test";
import { readWorkflow } from "./pr-review-source";

test("configures the trusted Forgejo review workflow", async () => {
  const workflow = await readWorkflow();
  expect(workflow).toContain("CODETETHER_TASK_POLL_ATTEMPTS: '360'");
  expect(workflow).toContain("CODETETHER_REVIEW_TARGET_AGENT: pr-review-worker");
  expect(workflow).toContain("pull_request_target:");
  expect(workflow).toContain("CODETETHER_REVIEW_MODEL: openai-codex/gpt-5.6-sol-fast:high");
  expect(workflow).toContain("CODETETHER_REVIEW_AUTO_APPROVE: 'true'");
  expect(workflow).toContain("CODETETHER_REVIEW_DIFF_MAX_BYTES: '1500000'");
  expect(workflow).toContain("CODETETHER_REVIEW_CHUNK_MAX_BYTES: '200000'");
  expect(workflow).toContain("CODETETHER_REVIEW_PROMPT_MAX_BYTES: '240000'");
  expect(workflow).toContain("CODETETHER_REVIEW_TASK_MAX_BYTES: '900000'");
  expect(workflow).toContain("FORGEJO_TOKEN: ${{ github.token }}");
  expect(workflow).not.toContain("${{ secrets.FORGEJO_TOKEN }}");
  expect(workflow).toContain("bash tests/codetether-review-approval.test.sh");
  expect(workflow).toContain("bash tests/codetether-review-author-protocol.test.sh");
  expect(workflow).toContain("bash tests/codetether-review-task-create.test.sh");
});
'''


def main() -> None:
    """Write focused cases and register the new configuration module."""
    ASYNC.write_text(ASYNC_CONTENT)
    CONFIG.write_text(CONFIG_CONTENT)
    text = SPEC.read_text()
    addition = 'import "./pr-review-config.cases";\n'
    if addition not in text:
        anchor = 'import "./pr-review-async.cases";\n'
        if text.count(anchor) != 1:
            raise RuntimeError("Playwright async import anchor is missing")
        SPEC.write_text(text.replace(anchor, anchor + addition, 1))


if __name__ == "__main__":
    main()