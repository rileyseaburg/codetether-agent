/** @typedef {{kinds: string[], answer: string, explicitErrorFrames: number}} RpcFrames */

/** Interprets wrb.fr answer slots without treating opaque e trailers as errors.
 * @param {unknown[][]} frames @returns {RpcFrames} */
export function inspectFrames(frames) {
  /** @type {RpcFrames} */ const result = { kinds: [], answer: "", explicitErrorFrames: 0 };
  for (const frame of frames) {
    const kind = typeof frame[0] === "string" ? frame[0] : "other";
    result.kinds.push(["wrb.fr", "di", "af.httprm", "e", "er"].includes(kind) ? kind : "other");
    if (kind === "er") result.explicitErrorFrames++;
    if (kind !== "wrb.fr" || frame[2] === null) continue;
    if (typeof frame[2] !== "string") throw new Error("invalid_inner_payload");
    /** @type {unknown} */ let inner;
    try { inner = JSON.parse(frame[2]); } catch { throw new Error("invalid_inner_json"); }
    const candidate = atPath(inner, [4, 0, 1, 0]);
    if (typeof candidate === "string" && candidate.trim()) result.answer = candidate;
  }
  return result;
}

/** @param {unknown} value @param {number[]} path @returns {unknown} */
function atPath(value, path) {
  for (const index of path) {
    if (!Array.isArray(value)) return undefined;
    value = value[index];
  }
  return value;
}

/** Detects observed generic-error replies; an exact smoke check is still required.
 * @param {string} answer @returns {boolean} */
export function isErrorAnswer(answer) {
  return /sorry,? something went wrong|encounter(?:ed|ing) an error|having a hard time fulfilling|unable to (?:fulfill|process)|could(?:n't| not) (?:fulfill|process)/i.test(answer);
}