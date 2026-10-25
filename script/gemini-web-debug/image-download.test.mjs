import assert from "node:assert/strict";
import test from "node:test";
import { downloadImage } from "./image-download.mjs";
import { png, response } from "./image-fixture.mjs";

const initial = "https://lh3.googleusercontent.com/gg-dl/fixture";
const middle = "https://work.fife.usercontent.google.com/rd-gg-dl/fixture";
const final = "https://lh3.googleusercontent.com/rd-gg-dl/fixture";
test("mocked local: two text-URL hops retrieve bytes with correctly scoped cookies", async () => {
  /** @type {Array<{url: string, headers: Record<string, string>}>} */ const calls = [];
  /** @type {Array<Record<string, unknown>>} */ const evidence = [];
  const result = await downloadImage(initial, () => "test-only-cookie", v => evidence.push(v), async (url, headers) => {
    calls.push({url, headers});
    return url === initial ? response(middle) : url === middle ? response(final) : response(png, "image/png");
  });
  assert.equal(result.width, 1); assert.deepEqual(result.body, png);
  assert.deepEqual(calls.map(c => c.url), [initial, middle, final]);
  assert.equal(calls[0].headers.Cookie, undefined); assert.equal(calls[1].headers.Cookie, "test-only-cookie");
  assert.equal(calls[2].headers.Cookie, undefined);
  assert.equal(calls[2].headers.Origin, "https://gemini.google.com");
  assert.ok(!JSON.stringify(evidence).includes("fixture")); assert.ok(!JSON.stringify(evidence).includes("test-only-cookie"));
});
test("mocked local: redirects work, foreign hosts fail before cookies or network", async () => {
  const redirect = response("", "text/plain", 302); redirect.headers = `Location: ${final}\r\n`;
  const image = await downloadImage(initial, () => "", () => {}, async url => url === initial ? redirect : response(png, "image/png"));
  assert.equal(image.extension, "png");
  for (const target of ["http://lh3.googleusercontent.com/gg-dl/x", "https://evil.invalid/x", "https://lh3.googleusercontent.com.evil.invalid/gg-dl/x", "https://u:p@lh3.googleusercontent.com/gg-dl/x", "https://lh3.googleusercontent.com:8443/gg-dl/x"]) {
    let calls = 0;
    await assert.rejects(downloadImage(initial, () => { throw new Error("must not resolve cookies"); }, () => {}, async () => {
      calls++; return response(target);
    }), /image_url_disallowed/);
    assert.equal(calls, 1);
  }
});
test("mocked local: loops, hop exhaustion, HTTP and invalid images are failures", async () => {
  await assert.rejects(downloadImage(initial, () => "", () => {}, async () => response(initial)), /image_redirect_loop/);
  let count = 0;
  await assert.rejects(downloadImage(initial, () => "", () => {}, async () => response(`${initial}${++count}`)), /image_hop_limit/);
  assert.equal(count, 6);
  await assert.rejects(downloadImage(initial, () => "", () => {}, async () => response("", "text/plain", 403)), /image_http_error/);
  await assert.rejects(downloadImage(initial, () => "", () => {}, async () => response("not-an-image", "image/jpeg")), /image_content_invalid/);
  const timeout = response(""); timeout.exitCode = 28;
  await assert.rejects(downloadImage(initial, () => "", () => {}, async () => timeout), /image_transport_error/);
});