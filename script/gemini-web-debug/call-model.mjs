import { spawnSync } from "node:child_process";
import { writeFileSync } from "node:fs";

/** @typedef {{model: string, configuredMode: string, exitCode: number|null, signal: string|null, elapsedMs: number, answer: string, smokeAnswerMatches: boolean, errorCode: string|null, log: string}} CallResult */
/**
 * Call the installed Rust provider, retaining only credential-redacted evidence.
 * @param {string} model
 * @param {string} configuredMode
 * @param {string} directory
 * @param {string[]} secrets
 * @returns {CallResult}
 */
export function callModel(model, configuredMode, directory, secrets) {
  const expected = "GEMINI_WEB_OK 37 * 43 = 1591";
  const args = ["run", "--access-mode", "ask", "--max-steps", "1", "--format", "json", "--model", `gemini-web/${model}`,
    `Do not use any tools, do not read or change any files. Reply with exactly this one line: ${expected}`];
  console.log(JSON.stringify({ event: "call_started", requestedModel: `gemini-web/${model}`, configuredMode }));
  const start = Date.now();
  const result = spawnSync("codetether", args, {
    encoding: "utf8", timeout: 150000, killSignal: "SIGKILL", maxBuffer: 4 * 1024 * 1024,
    env: { ...process.env, RUST_LOG: "error" }, stdio: ["ignore", "pipe", "pipe"],
  });
  let stdout = result.stdout ?? "";
  let stderr = result.stderr ?? "";
  for (const secret of secrets.filter(value => value.length > 4)) {
    stdout = stdout.replaceAll(secret, "[REDACTED]");
    stderr = stderr.replaceAll(secret, "[REDACTED]");
  }
  let answer = "";
  try {
    const parsed = JSON.parse(stdout.slice(stdout.indexOf("{")));
    if (typeof parsed.text === "string") answer = parsed.text;
  } catch { /* Non-JSON failures remain available in the redacted log. */ }
  const log = `${directory}/${model}.log`;
  writeFileSync(log, `stdout:\n${stdout}\nstderr:\n${stderr}`, { mode: 0o600 });
  return { model: `gemini-web/${model}`, configuredMode, exitCode: result.status, signal: result.signal, elapsedMs: Date.now() - start, answer, smokeAnswerMatches: answer.trim() === expected, errorCode: result.error?.code ?? null, log };
}