import assert from "node:assert/strict";
import test from "node:test";
import { imageContinuation } from "./image-continuation.mjs";
import { imageFrame, imageResponse } from "./image-fixture.mjs";
import { parseImageResponse } from "./image-protocol.mjs";
import { imageForm } from "./image-payload.mjs";

const state = {conversationId: "c_fixture", responseId: "r_fixture", choiceId: "rc_fixture"};
test("mocked local: continuation is taken from image-candidate metadata", () => {
  const frame = imageFrame(["https://lh3.googleusercontent.com/gg-dl/test"]);
  const inner = JSON.parse(String(frame[2])); inner[1] = [state.conversationId, state.responseId]; inner[4][0][0] = state.choiceId;
  frame[2] = JSON.stringify(inner);
  assert.deepEqual(parseImageResponse(imageResponse([frame])).conversation, state);
  inner[4][0][0] = "https://untrusted.invalid/"; frame[2] = JSON.stringify(inner);
  assert.equal(parseImageResponse(imageResponse([frame])).conversation, null);
});
test("mocked local: refinement carries conversation, response and candidate IDs", () => {
  const body = imageForm("Make it blue", "test-token", state);
  const inner = JSON.parse(JSON.parse(body.get("f.req") ?? "[]")[1]);
  assert.deepEqual(inner[2].slice(0, 3), Object.values(state));
  assert.equal(inner[71][0], state.conversationId); assert.equal(inner[0][0], "Make it blue");
  assert.equal(JSON.parse(JSON.parse(imageForm("new", "token").get("f.req") ?? "[]")[1])[2], null);
});
test("mocked local: arbitrary or malformed continuation state is rejected", () => {
  for (const bad of [null, {}, {...state, choiceId: "wrong"}, {...state, responseId: "r_test\nsecret"}]) {
    assert.throws(() => imageContinuation(bad), /image_state_invalid/);
  }
});