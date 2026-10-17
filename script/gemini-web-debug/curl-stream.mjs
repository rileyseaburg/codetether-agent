import { mkdirSync, writeFileSync } from "node:fs";
import { loadCookieHeader } from "./vault.mjs";
import { form, modelHeader } from "./payload.mjs";
import { curlRequest } from "./curl-wire.mjs";
import { summarizeCurl } from "./curl-response.mjs";
import { curlSession, USER_AGENT } from "./curl-session.mjs";

const directory = `artifacts/gemini-curl-stream-${new Date().toISOString().replace(/[-:.]/g, "")}`;
mkdirSync(directory, { recursive: true, mode: 0o700 });
/** @type {Array<Record<string, unknown>>} */ const results = [];
/** @param {string} label @param {Record<string, unknown>} summary @returns {void} */
function save(label, summary) {
  const record = { evidenceLevel: "static/local curl diagnostic against live Google HTTP",
    runtime: "curl 8.5.0 --no-buffer, not browser or Rust agent loop", label, ...summary };
  results.push(record);
  writeFileSync(`${directory}/${label}.json`, `${JSON.stringify(record, null, 2)}\n`, { mode: 0o600 });
  writeFileSync(`${directory}/summary.json`, `${JSON.stringify(results, null, 2)}\n`, { mode: 0o600 });
  console.log(JSON.stringify({ event: "result", directory, ...record }));
}
try {
  const cookie = loadCookieHeader();
  const h2 = await curlSession(cookie, "--http2", save);
  const h1 = await curlSession(cookie, "--http1.1", save);
  const session = h2 ?? h1;
  if (!session) throw new Error("No curl session tokens available");
  const expected = "CURL_STREAM_OK 1591";
  for (const [model, mode] of [["gemini-web-thinking", "5bf011840784117a"], ["gemini-web-pro", "9d8ca3786ebdfbea"]]) {
    for (const protocol of ["--http2", "--http1.1"]) {
      const label = `${model}-${protocol.slice(2)}`;
      const tokens = protocol === "--http2" ? h2 ?? session : h1 ?? session;
      const url = new URL(`https://gemini.google.com${tokens.prefix}/_/BardChatUi/data/assistant.lamda.BardFrontendService/StreamGenerate`);
      url.search = new URLSearchParams({ bl: tokens.bl, "f.sid": tokens.sid, hl: "en", _reqid: "100001", rt: "c" }).toString();
      const headers = { Cookie: cookie, "User-Agent": USER_AGENT, Accept: "text/event-stream",
        "Content-Type": "application/x-www-form-urlencoded;charset=UTF-8", Origin: "https://gemini.google.com",
        Referer: "https://gemini.google.com/", "x-goog-ext-525001261-jspb": modelHeader(mode) };
      console.log(JSON.stringify({ event: "request", label, mode, requestedProtocol: protocol, accept: headers.Accept, directory }));
      const response = await curlRequest(url.href, headers,
        form(`No tools. Reply with exactly ${expected}. This is 37 times 43.`, tokens.at).toString(), protocol);
      const secrets = [cookie, tokens.at, tokens.sid, process.env.VAULT_TOKEN ?? "", ...cookie.split(/;\s*/).map(pair => pair.slice(pair.indexOf("=") + 1))];
      save(label, { ...summarizeCurl(response, secrets, expected), model, requestedMode: mode, requestedProtocol: protocol, accept: headers.Accept });
    }
  }
} catch {
  console.error(JSON.stringify({ event: "probe-failure", error: "Sensitive diagnostic details suppressed", directory }));
  process.exitCode = 2;
}
