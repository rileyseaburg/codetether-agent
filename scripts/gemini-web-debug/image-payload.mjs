import { randomUUID } from "node:crypto";
import { imageContinuation } from "./image-continuation.mjs";

/** Observed selector, not an attestation of the backend model version. */
export const IMAGE_MODE = "e051ce1aa80aa576";

/** Fresh-chat image payload. Opaque browser/account fields are not replayed.
 * @param {string} prompt @param {string} at
 * @param {import("./image-continuation.mjs").ImageContinuation} [previous] @returns {URLSearchParams} */
export function imageForm(prompt, at, previous) {
  if (!prompt.trim()) throw new Error("image_prompt_empty");
  /** @type {unknown[]} */ const inner = Array(99).fill(null);
  inner[0] = [prompt, 0, null, null, null, null, 0]; inner[1] = ["en"];
  inner[6] = [0]; inner[7] = 1; inner[10] = 1; inner[11] = 0;
  inner[17] = [[1]]; inner[18] = 0; inner[27] = 1; inner[30] = [4, 16, 19];
  inner[49] = 14; inner[53] = 0; inner[54] = []; inner[55] = [];
  inner[59] = randomUUID(); inner[61] = []; inner[67] = 0; inner[68] = 1;
  inner[79] = 5; inner[91] = 1; inner[96] = 0; inner[98] = 1;
  if (previous) {
    const {conversationId, responseId, choiceId} = imageContinuation(previous);
    inner[2] = [conversationId, responseId, choiceId, null, null, null, null, null, null, ""];
    inner[71] = [conversationId, null, null, null, null, null, null, ""];
  }
  return new URLSearchParams({ "f.req": JSON.stringify([null, JSON.stringify(inner)]), at });
}

/** Current browser image-mode header, without captured account/session values.
 * @param {string} buildLabel @returns {string} */
export function imageModelHeader(buildLabel) {
  return JSON.stringify([1, null, null, null, IMAGE_MODE, null, null, 0,
    [4, 5, 6, 8, 16, 19, 20], null, null, 2, null, null, 5, null, randomUUID(), [buildLabel]]);
}