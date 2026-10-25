import assert from "node:assert/strict";
import test from "node:test";
import { sessionTokens } from "./session-tokens.mjs";

test("mocked local: authenticated bootstrap token wins regardless of document order", () => {
  for (const html of ['{"thykhd":"anonymous","SNlM0e":"signed-in"}', '{"SNlM0e":"signed-in","thykhd":"anonymous"}']) {
    const tokens = sessionTokens(html);
    assert.equal(tokens.at, "signed-in"); assert.equal(tokens.authenticatedToken, true);
  }
});
test("mocked local: anonymous bootstrap remains usable for text, not authenticated", () => {
  const tokens = sessionTokens('{"thykhd":"anonymous","cfb2h":"build","FdrFJe":"sid"}');
  assert.deepEqual(tokens, {at: "anonymous", bl: "build", sid: "sid", authenticatedToken: false});
  assert.equal(sessionTokens("empty").at, "");
});
test("mocked local: escaped JSON values and whitespace are decoded", () => {
  const tokens = sessionTokens('{"SNlM0e" : "token\\u003asuffix"}');
  assert.equal(tokens.at, "token:suffix"); assert.equal(tokens.authenticatedToken, true);
});