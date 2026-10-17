import { createHash } from "node:crypto";

/** @param {import("./curl-wire.mjs").CurlResult} response
 * @param {string[]} secrets @param {string | null} expected
 * @returns {Record<string, unknown>} */
export function summarizeCurl(response, secrets, expected = null) {
  const text = response.body.toString("utf8");
  /** @type {Array<{kind: string, fieldCount: number}>} */ const frames = [];
  let answer = "";
  for (const line of text.split("\n")) {
    /** @type {unknown} */ let batch;
    try { batch = JSON.parse(line); } catch { continue; }
    if (!Array.isArray(batch)) continue;
    for (const frame of batch) {
      if (!Array.isArray(frame)) continue;
      frames.push({ kind: ["wrb.fr", "di", "af.httprm", "e", "er"].includes(frame[0]) ? frame[0] : "other", fieldCount: frame.length });
      if (frame[0] !== "wrb.fr" || typeof frame[2] !== "string") continue;
      try {
        const inner = JSON.parse(frame[2]);
        const candidate = inner?.[4]?.[0]?.[1]?.[0];
        if (typeof candidate === "string" && candidate.trim()) answer = candidate;
      } catch {}
    }
  }
  for (const secret of secrets.filter(Boolean)) answer = answer.replaceAll(secret, "[REDACTED]");
  const contentType = response.metrics.content_type ?? response.headers.match(/^content-type:\s*(.+)$/im)?.[1]?.trim() ?? null;
  const sseDataLines = text.split(/\r?\n/).filter(line => /^data:/.test(line)).length;
  const sseContentType = /^text\/event-stream(?:;|$)/i.test(contentType ?? "");
  return { curlExitCode: response.exitCode, status: response.metrics.http_code ?? 0,
    httpVersion: response.metrics.http_version ?? null, contentType,
    timeToFirstByteSeconds: response.metrics.time_starttransfer ?? null,
    totalSeconds: response.metrics.time_total ?? null, responseBytes: response.body.length,
    responseSha256: createHash("sha256").update(response.body).digest("hex"),
    chunks: response.chunks, chunksSource: "curl --include stdout; chunk bytes include response headers",
    sseContentType, sseDataLines,
    isSse: sseContentType && sseDataLines > 0, antiXssiPrefix: text.startsWith(")]}'"),
    lengthPrefixCount: text.split("\n").filter(line => /^\d+$/.test(line.trim())).length,
    frames, answer, smokeAnswerMatches: expected !== null && response.exitCode === 0 && answer.trim() === expected,
    backendIdentity: "Not attested", rawResponseSaved: false, requestCredentialsSaved: false };
}
