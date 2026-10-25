import { curlSession } from "./curl-session.mjs";
import { form } from "./payload.mjs";
import { rpcResult } from "./rpc-result.mjs";
import { sendRpc } from "./rpc-request.mjs";
import { imageForm, imageModelHeader, IMAGE_MODE } from "./image-payload.mjs";
import { parseImageResponse } from "./image-protocol.mjs";
import { ImageError } from "./image-error.mjs";

export const RPC_MODES = Object.freeze({
  "gemini-web-thinking": "5bf011840784117a", "gemini-web-pro": "9d8ca3786ebdfbea",
});

/** Authenticated Gemini web-RPC client using curl HTTP/2, not the Rust provider.
 * @param {string} cookie @param {(label: string, summary: Record<string, unknown>) => void} save
 * @returns {Promise<{authenticatedToken: boolean, generate: (model: keyof typeof RPC_MODES, prompt: string, expected: string) => Promise<import("./rpc-result.mjs").RpcResult>,
 * generateImages: (prompt: string, previous?: import("./image-continuation.mjs").ImageContinuation) => Promise<ReturnType<typeof parseImageResponse>>}>} */
export async function createRpcClient(cookie, save) {
  const session = await curlSession(cookie, "--http2", save);
  if (!session) throw new Error("session_unavailable");
  const secrets = [cookie, session.at, session.sid, process.env.VAULT_TOKEN ?? "",
    ...cookie.split(/;\s*/).map(pair => pair.slice(pair.indexOf("=") + 1))];
  return {
    authenticatedToken: session.authenticatedToken,
    /** @param {keyof typeof RPC_MODES} model @param {string} prompt @param {string} expected
     * @returns {Promise<import("./rpc-result.mjs").RpcResult>} */
    async generate(model, prompt, expected) {
      if (!Object.hasOwn(RPC_MODES, model)) throw new Error("unknown_model");
      const response = await sendRpc(cookie, session, RPC_MODES[model], form(prompt, session.at));
      return rpcResult(response, secrets, expected);
    },
    /** Generate image references. Returned URLs are sensitive and must not be logged.
     * @param {string} prompt @param {import("./image-continuation.mjs").ImageContinuation} [previous]
     * @returns {Promise<ReturnType<typeof parseImageResponse>>} */
    async generateImages(prompt, previous) {
      if (!session.authenticatedToken) throw new ImageError("image_session_not_authenticated");
      return parseImageResponse(await sendRpc(cookie, session, IMAGE_MODE, imageForm(prompt, session.at, previous), imageModelHeader(session.bl)));
    },
  };
}