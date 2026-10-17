import { execFileSync } from "node:child_process";
import { mkdirSync, writeFileSync } from "node:fs";
import { callModel } from "./call-model.mjs";

const directory = `artifacts/gemini-web-model-calls-${new Date().toISOString().replace(/[-:.]/g, "")}`;
const models = [
  ["gemini-web-fast", "fbb127bbb056c959"],
  ["gemini-web-thinking", "5bf011840784117a"],
  ["gemini-web-pro", "9d8ca3786ebdfbea"],
  ["gemini-web-deep-think", "e6fa609c3fa255c0"],
];
mkdirSync(directory, { recursive: true, mode: 0o700 });
console.log(JSON.stringify({ event: "evidence_directory", directory }));
try {
  const raw = execFileSync("vault", ["kv", "get", "-format=json", "secret/codetether/providers/gemini-web"], {
    encoding: "utf8", maxBuffer: 1024 * 1024, stdio: ["ignore", "pipe", "pipe"], timeout: 15000,
  });
  const secret = JSON.parse(raw);
  const cookies = secret.data.data.cookies;
  if (typeof cookies !== "string" || !cookies) throw new Error("Missing cookies");
  const entries = cookies.split("\n").filter(line => line.trim() && (!line.startsWith("#") || line.startsWith("#HttpOnly_")));
  const metadata = { version: secret.data.metadata.version, cookieCount: entries.length, credentialsRedacted: true };
  writeFileSync(`${directory}/vault-metadata.redacted.json`, `${JSON.stringify(metadata, null, 2)}\n`, { mode: 0o600 });
  console.log(JSON.stringify({ event: "vault_cookie_metadata", ...metadata }));
  const secrets = [cookies, process.env.VAULT_TOKEN ?? "", ...entries.map(line => line.split("\t")[6] ?? "")];
  const attempts = [];
  for (const [model, mode] of models) {
    const attempt = callModel(model, mode, directory, secrets);
    attempts.push(attempt);
    const summary = { evidenceLevel: "static/local invocation against live Google HTTP", runtime: "installed CodeTether 4.7.6-dev.37 Rust provider and agent loop", vault: metadata, attempts,
      backendIdentity: "Configured modes, not independently attested backend versions", directory };
    writeFileSync(`${directory}/summary.json`, `${JSON.stringify(summary, null, 2)}\n`, { mode: 0o600 });
    console.log(JSON.stringify({ event: "call_finished", ...attempt }));
  }
  process.exitCode = attempts.every(attempt => attempt.exitCode === 0 && attempt.smokeAnswerMatches) ? 0 : 2;
} catch {
  const failure = { event: "failure", category: "vault_or_capture_failure", details: "Credential-bearing exception details suppressed", directory };
  writeFileSync(`${directory}/failure.redacted.json`, `${JSON.stringify(failure, null, 2)}\n`, { mode: 0o600 });
  console.error(JSON.stringify(failure));
  process.exitCode = 1;
}
