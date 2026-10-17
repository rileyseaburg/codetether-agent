import { spawn } from "node:child_process";

/** @typedef {{http_code?: number, http_version?: string, content_type?: string, time_starttransfer?: number, time_total?: number}} Metrics */
/** @typedef {{exitCode: number, body: Buffer, headers: string, metrics: Metrics, chunks: Array<{elapsedMs: number, bytes: number}>}} CurlResult */

/** @param {string} value @returns {string} */
function quote(value) {
  return `"${value.replaceAll("\\", "\\\\").replaceAll('"', '\\"').replaceAll("\n", "\\n").replaceAll("\r", "\\r")}"`;
}

/** Runs curl with credential-bearing configuration exclusively on stdin.
 * @param {string} url @param {Record<string, string>} headers
 * @param {string | null} body @param {"--http2" | "--http1.1"} protocol
 * @returns {Promise<CurlResult>} */
export function curlRequest(url, headers, body, protocol) {
  if (new URL(url).origin !== "https://gemini.google.com") throw new Error("Unexpected Gemini origin");
  const config = [`url = ${quote(url)}`, ...Object.entries(headers).map(([key, value]) => `header = ${quote(`${key}: ${value}`)}`)];
  if (body !== null) config.push(`data = ${quote(body)}`);
  const args = ["--disable", "--no-buffer", "--silent", "--show-error", "--compressed", protocol,
    "--connect-timeout", "20", "--max-time", "180", "--config", "-",
    "--include", "--write-out", "%{stderr}CURL_RESULT:%{json}\n"];
  return new Promise((resolve, reject) => {
    const start = Date.now();
    const child = spawn("curl", args, { stdio: ["pipe", "pipe", "pipe"] });
    /** @type {Buffer[]} */ const buffers = [];
    /** @type {CurlResult["chunks"]} */ const chunks = [];
    let responseHeaders = "", stderr = "";
    child.stdout.on("data", chunk => {
      buffers.push(Buffer.from(chunk));
      chunks.push({ elapsedMs: Date.now() - start, bytes: chunk.length });
    });
    child.stderr.on("data", chunk => { stderr += chunk.toString(); });
    child.on("error", () => reject(new Error("curl process could not start")));
    child.on("close", code => {
      const raw = stderr.match(/CURL_RESULT:(\{[^\n]*\})/)?.[1];
      /** @type {Metrics} */ let metrics = {};
      try { metrics = JSON.parse(raw ?? "{}"); } catch {}
      let received = Buffer.concat(buffers);
      while (received.subarray(0, 5).toString() === "HTTP/") {
        const boundary = received.indexOf("\r\n\r\n");
        if (boundary < 0) break;
        responseHeaders += received.subarray(0, boundary + 4).toString();
        received = received.subarray(boundary + 4);
      }
      resolve({ exitCode: code ?? -1, body: received, headers: responseHeaders, metrics, chunks });
    });
    child.stdin.on("error", () => {});
    child.stdin.end(`${config.join("\n")}\n`);
  });
}