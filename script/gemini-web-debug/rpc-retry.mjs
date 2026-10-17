import { mkdirSync, writeFileSync } from "node:fs";
import { loadCookieHeader } from "./vault.mjs";
import { createRpcClient, RPC_MODES } from "./rpc-client.mjs";

const directory = `artifacts/gemini-rpc-client-${new Date().toISOString().replace(/[-:.]/g, "")}`;
const expected = "RPC_CLIENT_OK 1591";
const prompt = `Reply exactly ${expected}. This checks 37 * 43. Do not use tools.`;
const evidenceLevel = "static/local client diagnostic against live Google HTTP";
mkdirSync(directory, { recursive: true, mode: 0o700 });
/** @param {string} label @param {Record<string, unknown>} summary @returns {void} */
function save(label, summary) {
  const record = { evidenceLevel, runtime: "Node RPC client with curl HTTP/2; not browser or Rust agent loop", ...summary };
  writeFileSync(`${directory}/${label}.json`, `${JSON.stringify(record, null, 2)}\n`, { mode: 0o600 });
  console.log(JSON.stringify({ directory, label, ...record }));
}
/** @type {Array<Record<string, unknown>>} */ const results = [];
try {
  const client = await createRpcClient(loadCookieHeader(), save);
  for (const model of /** @type {Array<keyof typeof RPC_MODES>} */ (Object.keys(RPC_MODES))) {
    for (let attempt = 1; attempt <= 2; attempt++) {
      console.log(JSON.stringify({ event: "request", model, mode: RPC_MODES[model], attempt, directory }));
      const result = await client.generate(model, prompt, expected);
      const record = { model, mode: RPC_MODES[model], attempt, prompt, expected,
        timestamp: new Date().toISOString(), ...result };
      save(`${model}-${attempt}`, record);
      results.push(record);
      if (result.outcome === "ok" || !["answer_error", "no_answer", "answer_mismatch"].includes(result.outcome)) break;
    }
  }
  const successfulModels = results.filter(result => result.outcome === "ok").map(result => result.model);
  save("summary", { results, successfulModels, backendIdentity: "Not attested" });
  if (successfulModels.length !== Object.keys(RPC_MODES).length) process.exitCode = 2;
} catch {
  save("failure", { error: "client_startup_or_request_failure; sensitive diagnostic details suppressed", results });
  process.exitCode = 2;
}