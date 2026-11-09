import { ImageError } from "./image-error.mjs";

/** @typedef {{conversationId: string, responseId: string, choiceId: string}} ImageContinuation */

/** Validate opaque server IDs before persisting or reusing a conversation.
 * @param {unknown} value @returns {ImageContinuation} */
export function imageContinuation(value) {
  if (!value || typeof value !== "object") throw new ImageError("image_state_invalid");
  const v = /** @type {Record<string, unknown>} */ (value);
  if (typeof v.conversationId !== "string" || !/^c_[a-zA-Z0-9_-]{1,128}$/.test(v.conversationId) ||
      typeof v.responseId !== "string" || !/^r_[a-zA-Z0-9_-]{1,128}$/.test(v.responseId) ||
      typeof v.choiceId !== "string" || !/^rc_[a-zA-Z0-9_-]{1,128}$/.test(v.choiceId)) {
    throw new ImageError("image_state_invalid");
  }
  return {conversationId: v.conversationId, responseId: v.responseId, choiceId: v.choiceId};
}

/** Read metadata from the same candidate as the image, never from response text.
 * @param {unknown} inner @param {unknown} candidate
 * @returns {ImageContinuation | null} */
export function candidateContinuation(inner, candidate) {
  if (!Array.isArray(inner) || !Array.isArray(inner[1]) || !Array.isArray(candidate)) return null;
  try {
    return imageContinuation({conversationId: inner[1][0], responseId: inner[1][1], choiceId: candidate[0]});
  } catch { return null; }
}