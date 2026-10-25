import { createRpcClient } from "./rpc-client.mjs";
import { refreshCookieJar } from "./cookie-refresh.mjs";

/** Bootstrap an image client with renewable cookies. A stale cache gets exactly
 * one forced refresh; image generation itself is never automatically repeated.
 * @param {(url: string) => string} base
 * @param {(label: string, value: Record<string, unknown>) => void} observe
 * @returns {Promise<{client: Awaited<ReturnType<typeof createRpcClient>>, cookies: (url: string) => string}>} */
export async function createImageClient(base, observe) {
  let refreshed = await refreshCookieJar(base, value => observe("cookie-refresh-0", value));
  /** @param {number} attempt @returns {ReturnType<typeof createRpcClient>} */
  const open = attempt => createRpcClient(refreshed.cookies("https://gemini.google.com/"), (label, value) => {
    observe(`auth-${attempt}-${label}`, {status: value.status, tokenPresent: value.tokenPresent,
      authenticatedToken: value.authenticatedToken});
  });
  let client = await open(0);
  if (!client.authenticatedToken && refreshed.cacheHit) {
    refreshed = await refreshCookieJar(base, value => observe("cookie-refresh-1", value), {force: true});
    client = await open(1);
  }
  return {client, cookies: refreshed.cookies};
}