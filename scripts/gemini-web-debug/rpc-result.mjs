import { GoogleRpcDecoder } from "./rpc-framing.mjs";
import { rpcLengths } from "./rpc-lengths.mjs";
import { inspectFrames, isErrorAnswer } from "./rpc-frames.mjs";
import { summarizeCurl } from "./curl-response.mjs";

/** @typedef {"ok" | "transport_error" | "http_error" | "framing_error" | "rpc_error" | "answer_error" | "no_answer" | "answer_mismatch"} Outcome */
/** @typedef {{outcome: Outcome, answer: string, framingError: string | null, batchCount: number, frameKinds: string[], explicitErrorFrames: number, evidence: Record<string, unknown>}} RpcResult */

/** Checks transfer, HTTP, RPC framing and answer independently, with redacted evidence.
 * @param {import("./curl-wire.mjs").CurlResult} response
 * @param {string[]} secrets @param {string} expected @returns {RpcResult} */
export function rpcResult(response, secrets, expected) {
  const decoder = new GoogleRpcDecoder();
  /** @type {import("./rpc-frames.mjs").RpcFrames} */ let parsed = { kinds: [], answer: "", explicitErrorFrames: 0 };
  /** @type {string | null} */ let framingError = null;
  try {
    parsed = inspectFrames(decoder.push(response.body));
    decoder.end();
  } catch (error) { framingError = error instanceof Error ? error.message : "rpc_decode_error"; }
  let answer = parsed.answer;
  for (const secret of secrets.filter(Boolean)) answer = answer.replaceAll(secret, "[REDACTED]");
  const outcome = classify(response, parsed, framingError, expected);
  return { outcome, answer, framingError, batchCount: decoder.batchCount, frameKinds: parsed.kinds,
    explicitErrorFrames: parsed.explicitErrorFrames,
    evidence: { ...summarizeCurl(response, secrets, expected), answer,
      rpcLengths: rpcLengths(response.body), smokeAnswerMatches: outcome === "ok", framingValidated: framingError === null } };
}

/** @param {import("./curl-wire.mjs").CurlResult} response
 * @param {import("./rpc-frames.mjs").RpcFrames} parsed
 * @param {string | null} error @param {string} expected @returns {Outcome} */
function classify(response, parsed, error, expected) {
  if (response.exitCode !== 0) return "transport_error";
  if (response.metrics.http_code !== 200) return "http_error";
  if (error) return "framing_error";
  if (parsed.explicitErrorFrames) return "rpc_error";
  if (isErrorAnswer(parsed.answer)) return "answer_error";
  if (!parsed.answer.trim()) return "no_answer";
  return parsed.answer.trim() === expected ? "ok" : "answer_mismatch";
}