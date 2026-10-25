import { curlRequest } from "./curl-wire.mjs";
import { USER_AGENT } from "./curl-session.mjs";
import { imageUrl } from "./image-url.mjs";
import { imageInfo } from "./image-file.mjs";
import { ImageError } from "./image-error.mjs";

/** @typedef {(url: string, headers: Record<string, string>) => Promise<import("./curl-wire.mjs").CurlResult>} ImageRequest */
/** @type {ImageRequest} */
const requestImage = (url, headers) => curlRequest(url, headers, null, "--http2", true);

/** Follow bounded text-URL/redirect chains. Recompute cookies per hop and send
 * no account cookies to signed image-CDN URLs, matching the observed browser.
 * @param {string} initial @param {(url: string) => string} cookies
 * @param {(record: Record<string, unknown>) => void} [observe]
 * @param {ImageRequest} [request] Network-free test injection.
 * @returns {Promise<{body: Buffer, extension: string, width: number, height: number}>} */
export async function downloadImage(initial, cookies, observe = () => {}, request = requestImage) {
  /** @type {Set<string>} */ const seen = new Set();
  let next = initial;
  for (let hop = 0; hop < 6; hop++) {
    const url = imageUrl(next);
    if (seen.has(url.href)) throw new ImageError("image_redirect_loop");
    seen.add(url.href);
    /** @type {Record<string, string>} */ const headers = { "User-Agent": USER_AGENT, Accept: "*/*",
      Origin: "https://gemini.google.com", Referer: "https://gemini.google.com/" };
    if (url.hostname === "work.fife.usercontent.google.com") {
      const cookie = cookies(url.href); if (cookie) headers.Cookie = cookie;
    }
    const response = await request(url.href, headers), status = response.metrics.http_code ?? 0;
    const mime = (response.metrics.content_type ?? "").split(";")[0].trim().toLowerCase();
    observe({ hop, host: url.hostname, status, mime: ["text/plain", "image/png", "image/jpeg"].includes(mime) ? mime : "other",
      bytes: response.body.length, exitCode: response.exitCode });
    if (response.exitCode !== 0) throw new ImageError("image_transport_error", { hop, exitCode: response.exitCode });
    if ([301, 302, 303, 307, 308].includes(status)) {
      const location = response.headers.match(/^location:\s*(.+)$/im)?.[1]?.trim();
      if (!location) throw new ImageError("image_redirect_missing");
      try { next = new URL(location, url).href; } catch { throw new ImageError("image_url_invalid"); }
      continue;
    }
    if (status !== 200) throw new ImageError("image_http_error", { hop, status });
    if (mime.startsWith("image/")) return { body: response.body, ...imageInfo(response.body, mime) };
    if (mime !== "text/plain" || response.body.length > 16384) throw new ImageError("image_response_unexpected", { hop });
    next = response.body.toString("utf8").trim();
  }
  throw new ImageError("image_hop_limit");
}