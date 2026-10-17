import assert from "node:assert/strict";
import test from "node:test";
import { rpcResult } from "./rpc-result.mjs";
import { fixture, framed } from "./rpc-fixture.mjs";

const expected = "RPC_CLIENT_OK 1591";
test("mocked local: valid answer and opaque e trailers are not RPC errors", () => {
  const result = rpcResult(fixture(expected, [["e", 9, 11629]]), [], expected);
  assert.equal(result.outcome, "ok");
  assert.equal(result.explicitErrorFrames, 0);
  assert.equal(result.evidence.framingValidated, true);
  assert.equal(result.evidence.smokeAnswerMatches, true);
  assert.equal(result.evidence.backendIdentity, "Not attested");
});
test("mocked local: transfer, HTTP, framing and explicit RPC failures differ", () => {
  const transport = fixture(); transport.exitCode = 28;
  assert.equal(rpcResult(transport, [], expected).outcome, "transport_error");
  const http = fixture(); http.metrics.http_code = 400;
  assert.equal(rpcResult(http, [], expected).outcome, "http_error");
  const invalid = fixture(); invalid.body = Buffer.from("fixture-secret");
  const result = rpcResult(invalid, ["fixture-secret"], expected);
  assert.equal(result.outcome, "framing_error");
  assert.equal(result.framingError, "missing_anti_xssi_prefix");
  assert.equal(JSON.stringify(result).includes("fixture-secret"), false);
  assert.equal(rpcResult(fixture(expected, [["er", 400]]), [], expected).outcome, "rpc_error");
});
test("mocked local: HTTP 200 generic errors, missing answers and mismatches fail", () => {
  for (const answer of ["Sorry, something went wrong.", "I encountered an error doing what you asked.",
    "I seem to be encountering an error.", "I'm having a hard time fulfilling your request."]) {
    const result = rpcResult(fixture(answer), [], expected);
    assert.equal(result.outcome, "answer_error");
    assert.equal(result.evidence.smokeAnswerMatches, false);
  }
  assert.equal(rpcResult(fixture(""), [], expected).outcome, "no_answer");
  assert.equal(rpcResult(fixture("another answer"), [], expected).outcome, "answer_mismatch");
  assert.equal(rpcResult(fixture("fixture-secret"), ["fixture-secret"], expected).answer, "[REDACTED]");
});
test("mocked local: malformed inner JSON and latest cumulative answers", () => {
  const invalid = fixture(); invalid.body = framed([["wrb.fr", null, "fixture-secret"]]);
  assert.equal(rpcResult(invalid, [], expected).framingError, "invalid_inner_json");
  invalid.body = framed([["wrb.fr", null, 42]]);
  assert.equal(rpcResult(invalid, [], expected).framingError, "invalid_inner_payload");
  const latest = fixture("draft");
  const inner = [null, null, null, null, [[null, [expected]]]];
  latest.body = framed([["wrb.fr", null, JSON.stringify([null, null, null, null, [[null, ["draft"]]]])],
    ["wrb.fr", null, JSON.stringify(inner)]]);
  assert.equal(rpcResult(latest, [], expected).outcome, "ok");
});