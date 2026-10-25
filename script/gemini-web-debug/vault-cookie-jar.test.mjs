import assert from "node:assert/strict";
import test from "node:test";
import { mkdtempSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { loadCookieJar } from "./vault-cookie-jar.mjs";

test("mocked local: explicit UTF-8 cookie files override Vault without exposing values", () => {
  const directory = mkdtempSync(join(tmpdir(), "gemini-cookie-file-"));
  const path = join(directory, "cookies.txt");
  writeFileSync(path, "\uFEFF.google.com\tTRUE\t/\tTRUE\t0\ttest\tsynthetic-value\n", {mode: 0o600});
  const cookies = loadCookieJar(path);
  assert.equal(cookies("https://gemini.google.com/"), "test=synthetic-value");
  assert.equal(cookies("https://unrelated.invalid/"), "");
});
test("mocked local: Windows UTF-16 cookie files are supported", () => {
  const directory = mkdtempSync(join(tmpdir(), "gemini-cookie-file-"));
  const path = join(directory, "cookies.txt");
  writeFileSync(path, Buffer.from("\uFEFF.google.com\tTRUE\t/\tTRUE\t0\ttest\twindows-value\r\n", "utf16le"), {mode: 0o600});
  assert.equal(loadCookieJar(path)("https://gemini.google.com/"), "test=windows-value");
});
test("mocked local: missing and malformed cookie files fail closed", () => {
  const directory = mkdtempSync(join(tmpdir(), "gemini-cookie-file-"));
  const path = join(directory, "cookies.txt");
  assert.throws(() => loadCookieJar(path), /cookie_file_read_failed/);
  writeFileSync(path, "synthetic-secret=value", {mode: 0o600});
  assert.throws(() => loadCookieJar(path), error => {
    assert.ok(error instanceof Error); assert.equal(error.message, "cookie_domain_metadata_required"); return true;
  });
});