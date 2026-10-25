import assert from "node:assert/strict";
import test from "node:test";
import { cookieJar } from "./cookie-jar.mjs";

test("mocked local: cookie scope respects host, domain, path, expiry and TLS", () => {
  const jar = cookieJar([
    "# Netscape HTTP Cookie File",
    "#HttpOnly_.google.com\tTRUE\t/\tTRUE\t0\tsession\ttest-session",
    "gemini.google.com\tFALSE\t/app\tTRUE\t0\thost\ttest-host",
    ".google.com\tTRUE\t/\tTRUE\t1\texpired\tnot-sent",
    "work.fife.usercontent.google.com\tFALSE\t/rd-gg-dl/\tTRUE\t0\twork\ttest-work",
  ].join("\n"));
  assert.equal(jar("https://gemini.google.com/app"), "host=test-host; session=test-session");
  assert.equal(jar("https://gemini.google.com/application"), "session=test-session");
  assert.equal(jar("https://work.fife.usercontent.google.com/rd-gg-dl/image"), "work=test-work; session=test-session");
  for (const url of ["https://google.com.evil.invalid/", "https://lh3.googleusercontent.com/", "http://gemini.google.com/"]) {
    assert.equal(jar(url), "");
  }
});
test("mocked local: malformed or unscoped exports fail without leaking cookies", () => {
  for (const text of ["session\ttest-secret", ".google.com\tTRUE\t/\tTRUE\t0\tx\tbad;value", "# empty"]) {
    assert.throws(() => cookieJar(text), error => {
      assert.ok(error instanceof Error); assert.ok(!error.message.includes(text)); return true;
    });
  }
});
test("mocked local: empty cookie values retain their field boundary", () => {
  const jar = cookieJar(".google.com\tTRUE\t/\tTRUE\t0\tempty\t");
  assert.equal(jar("https://gemini.google.com/"), "empty=");
});