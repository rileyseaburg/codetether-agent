import assert from "node:assert/strict";
import test from "node:test";
import { imageFrame, imageResponse } from "./image-fixture.mjs";
import { parseImageResponse } from "./image-protocol.mjs";
import { imageForm, imageModelHeader, IMAGE_MODE } from "./image-payload.mjs";
import { form } from "./payload.mjs";

const a = "https://lh3.googleusercontent.com/gg-dl/fixture-a";
const b = "https://lh3.googleusercontent.com/gg-dl/fixture-b";
test("mocked local: image payload uses fresh context, text payload unchanged", () => {
  const request = imageForm("draw a lake", "test-token");
  const inner = JSON.parse(JSON.parse(request.get("f.req") ?? "[]")[1]);
  assert.equal(inner.length, 99); assert.deepEqual(inner[17], [[1]]);
  assert.equal(inner[0][0], "draw a lake"); assert.equal(request.get("at"), "test-token");
  assert.equal(inner[3], null); assert.equal(inner[4], null);
  assert.notEqual(inner[59], JSON.parse(JSON.parse(imageForm("draw", "x").get("f.req") ?? "[]")[1])[59]);
  assert.equal(JSON.parse(JSON.parse(form("text", "x").get("f.req") ?? "[]")[1]).length, 69);
  assert.throws(() => imageForm(" ", "x"));
  const header = JSON.parse(imageModelHeader("test-build"));
  assert.equal(header[4], IMAGE_MODE); assert.equal(header[11], 2);
  assert.deepEqual(header[17], ["test-build"]);
});
test("mocked local: image-creation unavailability is not success or a model-identity claim", () => {
  for (const answer of ["I can't create it right now.", "I can't create any for you at the moment."]) {
    assert.throws(() => parseImageResponse(imageResponse([imageFrame([], answer)])), /generation_image_creation_unavailable/);
  }
});

test("mocked local: attachments deduplicate and newer candidates replace drafts", () => {
  const result = parseImageResponse(imageResponse([imageFrame([a]), imageFrame([b, b]), ["wrb.fr", null, "[null,null,null]"], ["e", 9]]));
  assert.deepEqual(result.urls, [`${b}=s1024-rj?alr=yes`]); assert.equal(result.batches, 1);
  assert.throws(() => parseImageResponse(imageResponse([imageFrame([a]), imageFrame([])])), /generation_no_images/);
});
test("mocked local: prose, refusals, unsafe attachments and errors cannot succeed", () => {
  assert.throws(() => parseImageResponse(imageResponse([imageFrame([], `Here is ${a}`)])), /generation_no_images/);
  assert.throws(() => parseImageResponse(imageResponse([imageFrame([a], "Sorry, something went wrong.")])), /generation_answer_error/);
  assert.throws(() => parseImageResponse(imageResponse([imageFrame(["https://evil.invalid/secret"])])), /image_url_disallowed/);
  assert.throws(() => parseImageResponse(imageResponse([["er", 400]])), /generation_rpc_error/);
  assert.throws(() => parseImageResponse(imageResponse([["wrb.fr", null, "broken-json"]])), /generation_payload_error/);
  const broken = imageResponse([imageFrame([a])]); broken.body = Buffer.from("not-framed");
  assert.throws(() => parseImageResponse(broken), /generation_framing_error/);
  broken.exitCode = 28; assert.throws(() => parseImageResponse(broken), /generation_transport_error/);
  broken.exitCode = 0; broken.metrics.http_code = 403;
  assert.throws(() => parseImageResponse(broken), /generation_http_error/);
});
test("mocked local: attachment count is bounded", () => {
  assert.throws(() => parseImageResponse(imageResponse([imageFrame(Array.from({length: 9}, (_, i) => `${a}${i}`))])), /generation_image_limit/);
});