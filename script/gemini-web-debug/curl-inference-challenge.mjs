// Full curl inference with an expected result withheld from the model prompt.
import { randomInt } from "node:crypto";
import { mkdirSync, writeFileSync } from "node:fs";
import { loadCookieHeader } from "./vault.mjs";
import { curlSession, USER_AGENT } from "./curl-session.mjs";
import { curlRequest } from "./curl-wire.mjs";
import { form, modelHeader } from "./payload.mjs";
import { summarizeCurl } from "./curl-response.mjs";

const modes = new Map([
  ["gemini-web-fast", "fbb127bbb056c959"],
  ["gemini-web-thinking", "5bf011840784117a"],
  ["gemini-web-pro", "9d8ca3786ebdfbea"],
]);
const requestedModel = process.argv[2] ?? "gemini-web-thinking";
const requestedMode = modes.get(requestedModel);
if (!requestedMode) throw new Error("Choose gemini-web-fast, gemini-web-thinking, or gemini-web-pro");
const directory = `artifacts/gemini-curl-inference-${new Date().toISOString().replace(/[-:.]/g, "")}`;
mkdirSync(directory, { recursive: true, mode: 0o700 });
const results = [];
try {
  const cookie = loadCookieHeader();
  for (const protocol of ["--http2", "--http1.1"]) {
    const tokens = await curlSession(cookie, protocol, () => {});
    if (!tokens) throw new Error("bootstrap_failed");
    const a = randomInt(100, 300), b = randomInt(20, 80), expected = String(a * b);
    const prompt = `Compute ${a} multiplied by ${b}. Return only the decimal integer product, no explanation and no tools.`;
    const url = new URL(`https://gemini.google.com${tokens.prefix}/_/BardChatUi/data/assistant.lamda.BardFrontendService/StreamGenerate`);
    url.search = new URLSearchParams({ bl: tokens.bl, "f.sid": tokens.sid, hl: "en", _reqid: "100001", rt: "c" }).toString();
    const response = await curlRequest(url.href, {
      Cookie: cookie, "User-Agent": USER_AGENT, Accept: "text/event-stream",
      "Content-Type": "application/x-www-form-urlencoded;charset=UTF-8",
      Origin: "https://gemini.google.com", Referer: "https://gemini.google.com/",
      "x-goog-ext-525001261-jspb": modelHeader(requestedMode),
    }, form(prompt, tokens.at).toString(), protocol);
    const secrets = [cookie, tokens.at, tokens.sid, process.env.VAULT_TOKEN ?? "",
      ...cookie.split(/;\s*/).map(pair => pair.slice(pair.indexOf("=") + 1))];
    const result = summarizeCurl(response, secrets, expected);
    const record = { evidenceLevel: "static/local curl diagnostic against live Google HTTP",
      runtime: "curl; Node only orchestrates serialization and parsing", requestedModel,
      requestedMode, requestedProtocol: protocol, prompt,
      expectedAnswer: expected, expectedAnswerIncludedInPrompt: false,
      authenticatedTokenMarker: tokens.authenticatedToken, ...result };
    results.push(record);
    writeFileSync(`${directory}/summary.json`, `${JSON.stringify(results, null, 2)}\n`, { mode: 0o600 });
    console.log(JSON.stringify({ directory, ...record, chunks: undefined, frames: undefined }));
    if (result.status !== 200 || !result.smokeAnswerMatches) process.exitCode = 1;
  }
} catch {
  console.error(JSON.stringify({ directory, error: "curl_inference_failed_sensitive_details_suppressed" }));
  process.exitCode = 2;
}