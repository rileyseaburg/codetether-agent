import assert from "node:assert/strict";
import test from "node:test";
import { mkdirSync, mkdtempSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { tmpdir } from "node:os";
import { runEasy } from "./easy-run.mjs";

/** @returns {import("./easy-options.mjs").EasyOptions} */
function options() {
  return {prompt: "a lake; $(not-a-command)", cookieFile: "/test/cookies.txt", help: false,
    directory: join(mkdtempSync(join(tmpdir(), "gemini-easy-")), "nested", "run")};
}
/** @param {number} status @returns {import("node:child_process").SpawnSyncReturns<string>} */
function result(status) {
  return {pid: 1, output: [], stdout: "private raw data", stderr: "private raw data", status, signal: null};
}
test("mocked local: short launcher forwards prompt safely and prints image paths", () => {
  const plan = {...options(), refine: "latest"};
  /** @type {string[]} */ const output = [];
  /** @type {import("./easy-run.mjs").Run} */ const run = (command, args, config) => {
    assert.equal(command, process.execPath); assert.equal(args[1], plan.prompt);
    assert.equal(args[3], "latest");
    assert.equal(config.shell, undefined); assert.equal(config.env?.GEMINI_WEB_COOKIE_FILE, plan.cookieFile);
    mkdirSync(plan.directory);
    writeFileSync(join(plan.directory, "summary.json"), JSON.stringify({images: [{path: join(plan.directory, "image-1.jpg") }]}));
    return result(0);
  };
  assert.equal(runEasy(plan, run, v => output.push(v)), 0);
  assert.ok(output.some(v => v.startsWith("Saved:")));
  assert.ok(!output.join("\n").includes("private raw data"));
});
test("mocked local: authentication failures become useful instructions", () => {
  const plan = options();
  /** @type {string[]} */ const output = [];
  assert.equal(runEasy(plan, () => {
    mkdirSync(plan.directory);
    writeFileSync(join(plan.directory, "failure.json"), JSON.stringify({code: "image_session_not_authenticated"}));
    return result(2);
  }, v => output.push(v)), 2);
  assert.ok(output.some(v => v.includes("export fresh cookies")));
  assert.ok(!output.join("\n").includes("private raw data"));
});
test("mocked local: existing output is preserved without starting the worker", () => {
  const plan = options(); mkdirSync(plan.directory, {recursive: true});
  assert.equal(runEasy(plan, () => { throw new Error("worker must not run"); }, () => {}), 2);
});