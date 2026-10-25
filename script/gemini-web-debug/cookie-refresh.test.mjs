import assert from "node:assert/strict";
import test from "node:test";
import { mkdtempSync, readdirSync, statSync, writeFileSync, chmodSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { refreshCookieJar } from "./cookie-refresh.mjs";
import { readRefreshCache } from "./cookie-refresh-cache.mjs";

const base = () => "__Secure-1PSID=synthetic-base; __Secure-1PSIDTS=old";
const headers = ["__Secure-1PSIDTS=new; Domain=.google.com; Path=/; Secure; Max-Age=3600"];
test("mocked local: refresh is cached privately and reused across calls", async () => {
  const directory = mkdtempSync(join(tmpdir(), "gemini-refresh-"));
  let calls = 0;
  const request = async () => { calls++; return {status: 200, headers}; };
  /** @type {Array<Record<string, unknown>>} */ const evidence = [];
  const first = await refreshCookieJar(base, e => evidence.push(e), {directory, request});
  assert.equal(first.cacheHit, false); assert.match(first.cookies("https://gemini.google.com"), /1PSIDTS=new/);
  const second = await refreshCookieJar(base, e => evidence.push(e), {directory, request});
  assert.equal(second.cacheHit, true); assert.equal(calls, 1);
  await refreshCookieJar(base, () => {}, {directory, request, force: true});
  assert.equal(calls, 2); assert.ok(!JSON.stringify(evidence).includes("synthetic-base"));
  const files = readdirSync(directory); assert.equal(files.length, 1);
  assert.ok(!files[0].includes("synthetic-base"));
  if (process.platform !== "win32") assert.equal(statSync(join(directory, files[0])).mode & 0o777, 0o600);
});
test("mocked local: malformed or non-private cookie cache is ignored", () => {
  const directory = mkdtempSync(join(tmpdir(), "gemini-refresh-")), file = join(directory, "cache.json");
  writeFileSync(file, "not-json", {mode: 0o600}); assert.equal(readRefreshCache(file), null);
  writeFileSync(file, JSON.stringify({updated: Date.now(), cookies: [{name: "SID", value: "not-allowed", expires: 1}]}));
  assert.equal(readRefreshCache(file), null);
  if (process.platform !== "win32") {
    writeFileSync(file, JSON.stringify({updated: Date.now(), cookies: []})); chmodSync(file, 0o644);
    assert.equal(readRefreshCache(file), null);
  }
});
test("mocked local: revoked credentials and missing updates fail without leaking data", async () => {
  const directory = mkdtempSync(join(tmpdir(), "gemini-refresh-"));
  await assert.rejects(refreshCookieJar(base, () => {}, {directory, request: async () => ({status: 401, headers: []})}), /cookie_refresh_rejected/);
  await assert.rejects(refreshCookieJar(base, () => {}, {directory, request: async () => ({status: 200, headers: []})}), /cookie_refresh_no_updates/);
  await assert.rejects(refreshCookieJar(() => "", () => {}, {directory}), /cookie_refresh_missing_session/);
});