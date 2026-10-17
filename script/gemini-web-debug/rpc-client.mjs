import { curlRequest } from "./curl-wire.mjs";
import { curlSession, USER_AGENT } from "./curl-session.mjs";
import { form, modelHeader } from "./payload.mjs";
import { rpcResult } from "./rpc-result.mjs";

export const RPC_MODES = Object.freeze({
  "gemini-web-thinking": "5bf011840784117a", "gemini-web-pro": "9d8ca3786ebdfbea",
});

/** Authenticated Gemini web-RPC client using curl HTTP/2, not the Rust provider.
 * @param {string} cookie @param {(label: string, summary: Record<string, unknown>) => void} save
 * @returns {Promise<{generate: (model: keyof typeof RPC_MODES, prompt: string, expected: string) => Promise<import("./rpc-result.mjs").RpcResult>}>} */
export async function createRpcClient(cookie, save) {
  const session = await curlSession(cookie, "--http2", save);
  if (!session) throw new Error("session_unavailable");
  const secrets = [cookie, session.at, session.sid, process.env.VAULT_TOKEN ?? "",
    ...cookie.split(/;\s*/).map(pair => pair.slice(pair.indexOf("=") + 1))];
  return {
    /** @param {keyof typeof RPC_MODES} model @param {string} prompt @param {string} expected
     * @returns {Promise<import("./rpc-result.mjs").RpcResult>} */
    async generate(model, prompt, expected) {
      if (!Object.hasOwn(RPC_MODES, model)) throw new Error("unknown_model");
      const mode = RPC_MODES[model];
      const url = new URL(`https://gemini.google.com${session.prefix}/_/BardChatUi/data/assistant.lamda.BardFrontendService/StreamGenerate`);
      url.search = new URLSearchParams({ bl: session.bl, "f.sid": session.sid, hl: "en",
        _reqid: String(Date.now() % 900000 + 100000), rt: "c" }).toString();
      const headers = { Cookie: cookie, "User-Agent": USER_AGENT, Accept: "application/json",
        "Content-Type": "application/x-www-form-urlencoded;charset=UTF-8", "X-Same-Domain": "1",
        Origin: "https://gemini.google.com", Referer: "https://gemini.google.com/app",
        "x-goog-ext-525001261-jspb": modelHeader(mode) };
      const response = await curlRequest(url.href, headers, form(prompt, session.at).toString(), "--http2");
      return rpcResult(response, secrets, expected);
    },
  };
}