import { existsSync, readdirSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { IMAGE_MODE } from "./image-payload.mjs";
import { imageContinuation } from "./image-continuation.mjs";
import { ImageError } from "./image-error.mjs";

/** Persist only private continuation IDs, not cookies or signed image URLs.
 * @param {string} directory @param {import("./image-continuation.mjs").ImageContinuation} conversation
 * @returns {void} */
export function saveImageState(directory, conversation) {
  writeFileSync(join(directory, "conversation.json"), JSON.stringify({version: 1, model: IMAGE_MODE,
    conversation: imageContinuation(conversation)}, null, 2), {mode: 0o600, flag: "wx"});
}

/** Resolve a particular version or the latest successful default-output version.
 * @param {string} selection @param {string} [root]
 * @returns {string} Absolute state-file path, suitable for recording provenance. */
export function resolveImageState(selection, root = "gemini-images") {
  let file;
  try {
    if (selection === "latest") {
      const candidates = readdirSync(root, {withFileTypes: true}).filter(e => e.isDirectory())
        .map(e => join(root, e.name)).filter(p => existsSync(join(p, "summary.json")) && existsSync(join(p, "conversation.json")));
      candidates.sort((a, b) => statSync(join(b, "conversation.json")).mtimeMs - statSync(join(a, "conversation.json")).mtimeMs);
      if (!candidates.length) throw new Error("missing");
      file = join(candidates[0], "conversation.json");
    } else {
      const path = resolve(selection); file = statSync(path).isDirectory() ? join(path, "conversation.json") : path;
    }
    if (statSync(file).size > 16384) throw new Error("oversized");
  } catch { throw new ImageError("image_state_missing"); }
  return resolve(file);
}

/** @param {string} selection @param {string} [root]
 * @returns {import("./image-continuation.mjs").ImageContinuation} */
export function loadImageState(selection, root = "gemini-images") {
  const file = resolveImageState(selection, root);
  try {
    /** @type {unknown} */ const value = JSON.parse(readFileSync(file, "utf8"));
    if (!value || typeof value !== "object" || !("version" in value) || value.version !== 1 ||
        !("model" in value) || value.model !== IMAGE_MODE || !("conversation" in value)) throw new Error("invalid");
    return imageContinuation(value.conversation);
  } catch { throw new ImageError("image_state_invalid"); }
}