import { GoogleRpcDecoder } from "./rpc-framing.mjs";
import { inspectFrames, isErrorAnswer } from "./rpc-frames.mjs";
import { ImageError } from "./image-error.mjs";
import { imageAttachments } from "./image-attachments.mjs";
import { rpcLengths } from "./rpc-lengths.mjs";


/** Extract structured attachments, never prose URLs. Latest candidate-bearing
 * frame replaces previous snapshots; trailers do not. URLs must not be logged.
 * @param {import("./curl-wire.mjs").CurlResult} response
 * @returns {{urls: string[], batches: number, conversation: import("./image-continuation.mjs").ImageContinuation | null}} */
export function parseImageResponse(response) {
  if (response.exitCode !== 0) throw new ImageError("generation_transport_error", { exitCode: response.exitCode });
  if (response.metrics.http_code !== 200) throw new ImageError("generation_http_error", { status: response.metrics.http_code });
  const decoder = new GoogleRpcDecoder();
  /** @type {unknown[][]} */ let frames;
  try { frames = decoder.push(response.body); decoder.end(); }
  catch (error) {
    const reason = error instanceof Error && /^[a-z_]+$/.test(error.message) ? error.message : "decode_error";
    throw new ImageError("generation_framing_error", {reason, bytes: response.body.length,
      batches: decoder.batchCount, lengths: rpcLengths(response.body).slice(0, 32)});
  }
  /** @type {ReturnType<typeof inspectFrames>} */ let parsed;
  try { parsed = inspectFrames(frames); } catch { throw new ImageError("generation_payload_error"); }
  if (parsed.explicitErrorFrames) throw new ImageError("generation_rpc_error");
  if (isErrorAnswer(parsed.answer)) throw new ImageError("generation_answer_error");
  const {urls, conversation} = imageAttachments(frames);
  if (!urls.length) {
    const unavailable = /image creation (?:isn't|may not be) available|can't create (?:it right now|any for you at the moment)/i.test(parsed.answer);
    throw new ImageError(unavailable ? "generation_image_creation_unavailable" : "generation_no_images", { batches: decoder.batchCount });
  }
  if (urls.length > 8) throw new ImageError("generation_image_limit");
  return { urls, batches: decoder.batchCount, conversation };
}