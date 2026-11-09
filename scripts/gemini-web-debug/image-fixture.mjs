import { framed } from "./rpc-fixture.mjs";

/** @param {string[]} urls @param {string} [answer] @returns {unknown[]} */
export function imageFrame(urls, answer = "Generated") {
  /** @type {unknown[]} */ const candidate = Array(13).fill(null);
  candidate[1] = [answer];
  const images = urls.map(url => [[null, null, null, [null, null, null, url]]]);
  candidate[12] = [null, null, null, null, null, null, null, [images]];
  return ["wrb.fr", null, JSON.stringify([null, null, null, null, [candidate]])];
}

/** @param {Buffer | string} body @param {string} [mime] @param {number} [status]
 * @returns {import("./curl-wire.mjs").CurlResult} */
export function response(body, mime = "text/plain", status = 200) {
  return { exitCode: 0, body: Buffer.from(body), headers: "", chunks: [], metrics: { http_code: status, content_type: mime } };
}

/** @param {unknown[][]} frames @returns {import("./curl-wire.mjs").CurlResult} */
export const imageResponse = frames => response(framed(frames), "application/json");
export const png = Buffer.from("iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mP8/x8AAwMCAO+/l8sAAAAASUVORK5CYII=", "base64");