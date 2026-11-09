import assert from "node:assert/strict";
import test from "node:test";
import { cookieUpdates, withCookieUpdates } from "./cookie-refresh-update.mjs";

test("mocked local: rotation accepts only secure, scoped, allowlisted cookies", () => {
  const suffix = "; Domain=.google.com; Path=/; Secure; Max-Age=3600";
  const updates = cookieUpdates([`__Secure-1PSIDTS=new${suffix}`, `SID=ignored${suffix}`,
    "__Secure-3PSIDTS=evil; Domain=evil.invalid; Path=/; Secure; Max-Age=3600",
    "__Secure-1PSIDCC=insecure; Domain=.google.com; Path=/; Max-Age=3600"]);
  assert.equal(updates.length, 1); assert.equal(updates[0].value, "new");
  const base = (/** @type {string} */ url) => new URL(url).hostname.endsWith(".google.com") ? "__Secure-1PSIDTS=old; other=kept" : "";
  const cookies = withCookieUpdates(base, updates);
  assert.equal(cookies("https://gemini.google.com/"), "__Secure-1PSIDTS=new; other=kept");
  assert.equal(cookies("https://lh3.googleusercontent.com/"), "");
  assert.equal(cookies("https://google.com.evil.invalid/"), "");
  assert.equal(cookies("http://gemini.google.com/"), base("http://gemini.google.com/"));
});
test("mocked local: expired updates remove the old token", () => {
  const updates = cookieUpdates(["__Secure-1PSIDTS=; Domain=.google.com; Path=/; Secure; Max-Age=0"]);
  assert.equal(withCookieUpdates(() => "__Secure-1PSIDTS=old", updates)("https://gemini.google.com/"), "");
});