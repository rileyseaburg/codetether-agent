import { imageRendition } from "./image-url.mjs";
import { candidateContinuation } from "./image-continuation.mjs";

/** @param {unknown} value @param {number[]} path @returns {unknown} */
function at(value, path) {
  for (const index of path) { if (!Array.isArray(value)) return undefined; value = value[index]; }
  return value;
}

/** Latest candidate snapshot and unambiguous continuation metadata.
 * @param {unknown[][]} frames
 * @returns {{urls: string[], conversation: import("./image-continuation.mjs").ImageContinuation | null}} */
export function imageAttachments(frames) {
  /** @type {string[]} */ let urls = [];
  /** @type {import("./image-continuation.mjs").ImageContinuation | null} */ let conversation = null;
  for (const frame of frames) {
    if (frame[0] !== "wrb.fr" || typeof frame[2] !== "string") continue;
    /** @type {unknown} */ const inner = JSON.parse(frame[2]);
    const candidates = at(inner, [4]);
    if (!Array.isArray(candidates)) continue;
    urls = []; conversation = null; let imageCandidates = 0;
    for (const candidate of candidates) {
      const images = at(candidate, [12, 7, 0]);
      if (!Array.isArray(images) || !images.length) continue;
      imageCandidates++; conversation = candidateContinuation(inner, candidate);
      for (const image of images) {
        const url = at(image, [0, 3, 3]);
        if (typeof url === "string") urls.push(imageRendition(url));
      }
    }
    if (imageCandidates !== 1) conversation = null;
  }
  return {urls: [...new Set(urls)], conversation};
}