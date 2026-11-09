import { loadCookieJar } from "./vault-cookie-jar.mjs";
import { createImageClient } from "./image-client.mjs";
import { downloadImage } from "./image-download.mjs";
import { saveImage } from "./image-output.mjs";
import { loadImageState, resolveImageState, saveImageState } from "./image-state.mjs";
import { ImageError } from "./image-error.mjs";

/** @typedef {{stage: string, images: Array<ReturnType<typeof saveImage>>}} WorkflowState */

/** Run one generation/refinement while retaining partial-output evidence.
 * @param {{prompt: string, directory: string, previousSelection?: string}} options
 * @param {WorkflowState} state
 * @param {(label: string, value: Record<string, unknown>) => void} save
 * @returns {Promise<{refined: boolean, refinable: boolean, refinementSource: string | null}>} */
export async function imageWorkflow(options, state, save) {
  state.stage = "conversation";
  const refinementSource = options.previousSelection ? resolveImageState(options.previousSelection) : null;
  const previous = refinementSource ? loadImageState(refinementSource) : undefined;
  if (refinementSource) save("refinement-source", {path: refinementSource});
  state.stage = "cookies";
  const base = loadCookieJar();
  state.stage = "session";
  const {client, cookies} = await createImageClient(base, save);
  state.stage = "generation";
  const generated = await client.generateImages(options.prompt, previous);
  if (previous && generated.conversation?.conversationId !== previous.conversationId) throw new ImageError("image_refinement_lost_context");
  save("generation", {batches: generated.batches, imageCount: generated.urls.length});
  state.stage = "download";
  for (const [index, url] of generated.urls.entries()) {
    const image = await downloadImage(url, cookies, hop => save(`image-${index + 1}-hop-${hop.hop}`, hop));
    state.images.push(saveImage(options.directory, index, image));
  }
  state.stage = "conversation-save";
  if (generated.conversation) saveImageState(options.directory, generated.conversation);
  return {refined: Boolean(previous), refinable: Boolean(generated.conversation), refinementSource};
}