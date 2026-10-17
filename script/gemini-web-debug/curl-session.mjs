import { curlRequest } from "./curl-wire.mjs";
import { summarizeCurl } from "./curl-response.mjs";

export const USER_AGENT = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 Chrome/131.0.0.0 Safari/537.36";
/** @typedef {{at: string, bl: string, sid: string, prefix: string}} Session */

/** @param {string} cookie @param {"--http2" | "--http1.1"} protocol
 * @param {(label: string, summary: Record<string, unknown>) => void} save
 * @returns {Promise<Session | null>} */
export async function curlSession(cookie, protocol, save) {
  let url = "https://gemini.google.com/";
  for (let attempt = 0; attempt < 4; attempt++) {
    const result = await curlRequest(url, { Cookie: cookie, "User-Agent": USER_AGENT }, null, protocol);
    const html = result.body.toString("utf8");
    const at = html.match(/"(?:thykhd|SNlM0e)":"([^"]+)"/)?.[1] ?? "";
    save(`session-${protocol.slice(2)}-${attempt}`, { ...summarizeCurl(result, [cookie]),
      phase: "session-token GET", requestedProtocol: protocol, tokenPresent: Boolean(at) });
    if (result.exitCode !== 0) return null;
    if (at && result.metrics.http_code === 200) return {
      at, bl: html.match(/"cfb2h":"([^"]+)"/)?.[1] ?? "",
      sid: html.match(/"FdrFJe":"([^"]+)"/)?.[1] ?? "",
      prefix: new URL(url).pathname.match(/(\/u\/\d+)\//)?.[1] ?? "",
    };
    const location = result.headers.match(/^location:\s*(.+)$/im)?.[1]?.trim();
    if (!location || ![301, 302, 303, 307, 308].includes(result.metrics.http_code ?? 0)) return null;
    const next = new URL(location, url);
    if (next.origin !== "https://gemini.google.com") return null;
    url = next.href;
  }
  return null;
}