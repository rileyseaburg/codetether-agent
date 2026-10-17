import assert from "node:assert/strict";
import test from "node:test";
import { summarizeCurl } from "./curl-response.mjs";

/** @param {string} text @param {string} contentType @param {number} exitCode
 * @returns {import("./curl-wire.mjs").CurlResult} */
function fixture(text, contentType = "application/json; charset=utf-8", exitCode = 0) {
  return { exitCode, body: Buffer.from(text), headers: "", chunks: [],
    metrics: { http_code: 200, http_version: "2", content_type: contentType } };
}

/** @param {string} answer @returns {string} */
function frame(answer) {
  const inner = [null, null, null, null, [[null, [answer]]]];
  const batch = JSON.stringify([["wrb.fr", null, JSON.stringify(inner)]]);
  return `${Buffer.byteLength(batch)}\n${batch}\n`;
}

test("mocked local: proprietary Google framing is not SSE", () => {
  const response = fixture(")]}'\n" + frame("draft") + frame("CURL_STREAM_OK 1591"));
  const summary = summarizeCurl(response, [], "CURL_STREAM_OK 1591");
  assert.equal(summary.antiXssiPrefix, true);
  assert.equal(summary.lengthPrefixCount, 2);
  assert.equal(summary.isSse, false);
  assert.equal(summary.smokeAnswerMatches, true);
});

test("mocked local: SSE requires matching content type and data lines", () => {
  assert.equal(summarizeCurl(fixture("data: hello\n\n", "text/event-stream"), []).isSse, true);
  assert.equal(summarizeCurl(fixture("data: hello\n\n"), []).isSse, false);
  assert.equal(summarizeCurl(fixture("", "text/event-stream"), []).isSse, false);
});

test("mocked local: an HTTP 200 error answer does not match the smoke test", () => {
  const summary = summarizeCurl(fixture(frame("Sorry, something went wrong.")), [], "CURL_STREAM_OK 1591");
  assert.equal(summary.status, 200);
  assert.equal(summary.smokeAnswerMatches, false);
});

test("mocked local: timeout and credential redaction remain explicit", () => {
  const timedOut = summarizeCurl(fixture(frame("CURL_STREAM_OK 1591"), "application/json", 28), [], "CURL_STREAM_OK 1591");
  assert.equal(timedOut.curlExitCode, 28);
  assert.equal(timedOut.smokeAnswerMatches, false);
  assert.equal(summarizeCurl(fixture(frame("fixture-secret")), ["fixture-secret"]).answer, "[REDACTED]");
  assert.equal(timedOut.backendIdentity, "Not attested");
});