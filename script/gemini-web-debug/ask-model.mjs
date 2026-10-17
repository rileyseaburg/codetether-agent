import { mkdirSync, writeFileSync } from "node:fs";
import { loadCookieHeader } from "./vault.mjs";
import { form, modelHeader } from "./payload.mjs";
import { curlRequest } from "./curl-wire.mjs";
import { curlSession, USER_AGENT } from "./curl-session.mjs";
import { summarizeCurl } from "./curl-response.mjs";

const directory = `artifacts/gemini-model-identity-${new Date().toISOString().replace(/[-:.]/g, "")}`;
const evidenceLevel = "static/local curl diagnostic against live Google HTTP";
const model = "gemini-web-pro", mode = "9d8ca3786ebdfbea";
const prompt = process.argv[2] ?? "What model are you? What is your exact model name and version? If you cannot verify the exact version, say so. Reply briefly without using tools.";
mkdirSync(directory, { recursive: true, mode: 0o700 });
/** @param {string} label @param {Record<string, unknown>} summary @returns {void} */
function save(label, summary) {
  const record = { evidenceLevel, runtime: "curl --no-buffer HTTP/2, not browser or Rust agent loop", ...summary };
  writeFileSync(`${directory}/${label}.json`, `${JSON.stringify(record, null, 2)}\n`, { mode: 0o600 });
  console.log(JSON.stringify({ directory, label, ...record }));
}
try {
  const cookie = loadCookieHeader();
  const session = await curlSession(cookie, "--http2", save);
  if (!session) throw new Error("Session unavailable");
  const url = new URL(`https://gemini.google.com${session.prefix}/_/BardChatUi/data/assistant.lamda.BardFrontendService/StreamGenerate`);
  url.search = new URLSearchParams({ bl: session.bl, "f.sid": session.sid, hl: "en",
    _reqid: String(Date.now() % 900000 + 100000), rt: "c" }).toString();
  const headers = { Cookie: cookie, "User-Agent": USER_AGENT, Accept: "text/event-stream",
    "Content-Type": "application/x-www-form-urlencoded;charset=UTF-8", Origin: "https://gemini.google.com",
    Referer: "https://gemini.google.com/", "x-goog-ext-525001261-jspb": modelHeader(mode) };
  console.log(JSON.stringify({ event: "request", directory, model, mode, prompt }));
  const response = await curlRequest(url.href, headers, form(prompt, session.at).toString(), "--http2");
  const secrets = [cookie, session.at, session.sid, process.env.VAULT_TOKEN ?? "",
    ...cookie.split(/;\s*/).map(pair => pair.slice(pair.indexOf("=") + 1))];
  const summary = summarizeCurl(response, secrets);
  save("answer", { timestamp: new Date().toISOString(), model, mode, prompt,
    modelSelectionHeader: headers["x-goog-ext-525001261-jspb"],
    identityEvidence: "Model self-description only; backend version not independently attested", ...summary });
  if (response.exitCode !== 0 || response.metrics.http_code !== 200 || !summary.answer) process.exitCode = 2;
} catch {
  save("failure", { timestamp: new Date().toISOString(), model, mode,
    error: "Sensitive diagnostic details suppressed" });
  process.exitCode = 2;
}
