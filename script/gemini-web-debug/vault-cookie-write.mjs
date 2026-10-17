import { spawnSync } from "node:child_process";
import { isDeepStrictEqual } from "node:util";

/** @typedef {{data: {data: Record<string, unknown>, metadata: {version: number, created_time: string}}}} SecretRead */
/** @typedef {{data: {version: number}}} SecretWrite */
const path = "codetether/providers/gemini-web";

/** Update only cookies using CAS, then check exact read-back without exposing data.
 * @param {string} token Temporary token; never persisted through vault login.
 * @param {string} cookies Netscape cookie text, sent to Vault exclusively on stdin.
 * @returns {Record<string, string | number | boolean>} Redacted verification metadata.
 */
export function storeCookies(token, cookies) {
  const env = { ...process.env, VAULT_TOKEN: token, VAULT_LOG_LEVEL: "error", VAULT_DISABLE_REDIRECTS: "true" };
  /** @param {string[]} args @param {string | undefined} payload @returns {SecretRead | SecretWrite} */
  function run(args, payload) {
    const result = spawnSync("vault", args, { env, input: payload, encoding: "utf8",
      maxBuffer: 2097152, timeout: 45000, stdio: ["pipe", "pipe", "pipe"] });
    if (result.error || result.status !== 0) {
      const diagnostic = /permission denied|Code: 403/i.test(result.stderr || "") ? "authorization denied"
        : /check-and-set/i.test(result.stderr || "") ? "concurrent update blocked by CAS"
        : /expired|invalid token/i.test(result.stderr || "") ? "token rejected" : "Vault command failed";
      console.error(JSON.stringify({ phase: args[1], diagnostic, credentialsRedacted: true }));
      throw new Error("Vault operation failed; raw output suppressed");
    }
    return JSON.parse(result.stdout);
  }
  try {
    const before = /** @type {SecretRead} */ (run(["kv", "get", "-non-interactive", "-format=json", "-mount=secret", path], undefined));
    const version = before?.data?.metadata?.version;
    const previous = before?.data?.data;
    if (!Number.isInteger(version) || version < 1 || !previous || typeof previous !== "object"
      || Array.isArray(previous)) throw new Error("Expected an existing KV-v2 provider secret");
    const payload = { ...previous, cookies };
    const written = /** @type {SecretWrite} */ (run(["kv", "put", "-non-interactive", "-format=json", `-cas=${version}`,
      "-mount=secret", path, "-"], JSON.stringify(payload)));
    const newVersion = written?.data?.version;
    if (newVersion !== version + 1) throw new Error("Unexpected write version");
    console.log(JSON.stringify({ phase: "write", path: `secret/${path}`, key: "cookies", version: newVersion, credentialsRedacted: true }));
    const after = /** @type {SecretRead} */ (run(["kv", "get", "-non-interactive", "-format=json", "-mount=secret", path], undefined));
    if (after?.data?.metadata?.version !== newVersion || !isDeepStrictEqual(after?.data?.data, payload)) {
      throw new Error("Write succeeded but exact read-back verification failed");
    }
    return { phase: "verified", path: `secret/${path}`, key: "cookies", version: newVersion,
      cookieCount: 14, format: "Netscape cookies.txt", exactCookieReadBack: true,
      otherFieldsPreserved: true, temporaryTokenSaved: false, plaintextSecretFilesCreated: false,
      createdTime: after.data.metadata.created_time };
  } finally { env.VAULT_TOKEN = ""; }
}