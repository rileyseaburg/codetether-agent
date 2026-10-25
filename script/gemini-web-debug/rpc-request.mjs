import { curlRequest } from "./curl-wire.mjs";
import { USER_AGENT } from "./curl-session.mjs";
import { modelHeader } from "./payload.mjs";

/** Shared StreamGenerate transport for text and image requests.
 * @param {string} cookie @param {import("./curl-session.mjs").Session} session
 * @param {string} mode @param {URLSearchParams} body
 * @param {string} [header] Mode-specific protobuf-JSON header.
 * @returns {Promise<import("./curl-wire.mjs").CurlResult>} */
export function sendRpc(cookie, session, mode, body, header = modelHeader(mode)) {
  const url = new URL(`https://gemini.google.com${session.prefix}/_/BardChatUi/data/assistant.lamda.BardFrontendService/StreamGenerate`);
  url.search = new URLSearchParams({ bl: session.bl, "f.sid": session.sid, hl: "en",
    _reqid: String(Date.now() % 900000 + 100000), rt: "c" }).toString();
  const headers = { Cookie: cookie, "User-Agent": USER_AGENT, Accept: "application/json",
    "Content-Type": "application/x-www-form-urlencoded;charset=UTF-8", "X-Same-Domain": "1",
    Origin: "https://gemini.google.com", Referer: "https://gemini.google.com/app",
    "x-goog-ext-525001261-jspb": header };
  return curlRequest(url.href, headers, body.toString(), "--http2");
}