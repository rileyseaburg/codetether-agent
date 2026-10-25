import assert from "node:assert/strict";
import test from "node:test";
import { resolve } from "node:path";
import { spawnSync } from "node:child_process";
import { fileURLToPath } from "node:url";
import { easyOptions } from "./easy-options.mjs";

test("mocked local: easy options use defaults and explicit overrides", () => {
  const defaults = {cookieFile: "/configured/cookies.txt", outputRoot: "gemini-images"};
  const options = easyOptions(["a", "yellow canoe"], defaults);
  assert.equal(options.prompt, "a yellow canoe"); assert.equal(options.cookieFile, defaults.cookieFile);
  assert.ok(options.directory.startsWith(resolve("gemini-images")));
  const explicit = easyOptions(["--cookies", "other.txt", "--out", "my-output", "lake"], defaults);
  assert.equal(explicit.cookieFile, resolve("other.txt")); assert.equal(explicit.directory, resolve("my-output"));
  assert.equal(easyOptions(["--help"], defaults).help, true);
  assert.throws(() => easyOptions(["--unknown"], defaults));
  assert.equal(easyOptions(["--refine", "latest", "make it blue"], defaults).refine, "latest");
});
test("mocked local: help and noninteractive missing-prompt paths need no credentials", () => {
  const entry = fileURLToPath(new URL("./easy-image.mjs", import.meta.url));
  for (const [args, status] of [[['--help'], 0], [[], 2]]) {
    const result = spawnSync(process.execPath, [entry, .../** @type {string[]} */ (args)], {encoding: "utf8"});
    assert.equal(result.status, status); assert.match(result.stdout, /Usage:.*gemini-image/);
    assert.equal(result.stderr, "");
  }
});