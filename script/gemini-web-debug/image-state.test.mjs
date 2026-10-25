import assert from "node:assert/strict";
import test from "node:test";
import { mkdtempSync, mkdirSync, writeFileSync, statSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { saveImageState, loadImageState, resolveImageState } from "./image-state.mjs";

const state = {conversationId: "c_fixture", responseId: "r_fixture", choiceId: "rc_fixture"};
test("mocked local: explicit and latest successful image versions load private state", () => {
  const root = mkdtempSync(join(tmpdir(), "gemini-state-")), first = join(root, "first"), failed = join(root, "failed");
  mkdirSync(first); mkdirSync(failed);
  saveImageState(first, state); writeFileSync(join(first, "summary.json"), "{}");
  saveImageState(failed, {...state, responseId: "r_failed"});
  assert.deepEqual(loadImageState(first), state);
  assert.deepEqual(loadImageState("latest", root), state);
  assert.equal(resolveImageState("latest", root), join(first, "conversation.json"));
  assert.throws(() => saveImageState(first, state));
  if (process.platform !== "win32") assert.equal(statSync(join(first, "conversation.json")).mode & 0o777, 0o600);
});
test("mocked local: old output and invalid state cannot silently create a new image", () => {
  const root = mkdtempSync(join(tmpdir(), "gemini-state-"));
  assert.throws(() => loadImageState(root), /image_state_invalid|image_state_missing/);
  assert.throws(() => loadImageState("latest", root), /image_state_missing/);
  writeFileSync(join(root, "conversation.json"), JSON.stringify({version: 1, model: "wrong", conversation: state}));
  assert.throws(() => loadImageState(root), /image_state_invalid/);
});