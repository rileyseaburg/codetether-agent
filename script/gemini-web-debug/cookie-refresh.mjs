import { createHash } from "node:crypto";
import { homedir } from "node:os";
import { join } from "node:path";
import { cookieUpdates, withCookieUpdates } from "./cookie-refresh-update.mjs";
import { readRefreshCache, writeRefreshCache } from "./cookie-refresh-cache.mjs";
import { rotateCookieRequest } from "./cookie-rotate-request.mjs";
import { ImageError } from "./image-error.mjs";

/** @typedef {{force?: boolean, directory?: string, request?: typeof rotateCookieRequest}} RefreshOptions */

/** Rotate renewable cookies and cache them privately, keyed to the source export.
 * Cache hits under one minute avoid hammering Google's rotation service. A caller
 * may force one renewal if a cached session unexpectedly fails authentication.
 * @param {(url: string) => string} base
 * @param {(value: Record<string, unknown>) => void} [observe]
 * @param {RefreshOptions} [options]
 * @returns {Promise<{cookies: (url: string) => string, cacheHit: boolean}>} */
export async function refreshCookieJar(base, observe = () => {}, options = {}) {
  const accountUrl = "https://accounts.google.com/RotateCookies", original = base(accountUrl);
  if (!/(?:^|;\s*)__Secure-1PSID=./.test(original)) throw new ImageError("cookie_refresh_missing_session");
  const key = createHash("sha256").update(original).digest("hex");
  const directory = options.directory ?? join(process.env.XDG_CACHE_HOME ?? join(homedir(), ".cache"), "codetether", "gemini-web");
  const cached = readRefreshCache(join(directory, `${key}.json`));
  const cookies = cached ? withCookieUpdates(base, cached.cookies) : base;
  const age = cached ? Date.now() - cached.updated : Infinity;
  if (!options.force && cached && age >= 0 && age < 60000 && cached.cookies.every(c => c.expires > Date.now())) {
    observe({source: "cache", refreshed: false}); return {cookies, cacheHit: true};
  }
  const response = await (options.request ?? rotateCookieRequest)(cookies(accountUrl));
  if (response.status !== 200) throw new ImageError("cookie_refresh_rejected", {status: response.status});
  const updates = cookieUpdates(response.headers);
  if (!updates.some(c => c.name === "__Secure-1PSIDTS" && c.expires > Date.now())) throw new ImageError("cookie_refresh_no_updates");
  const merged = new Map((cached?.cookies ?? []).map(c => [c.name, c]));
  for (const update of updates) merged.set(update.name, update);
  const data = {updated: Date.now(), cookies: [...merged.values()]};
  writeRefreshCache(directory, key, data);
  observe({source: "rotation", refreshed: true, status: response.status, names: updates.map(c => c.name)});
  return {cookies: withCookieUpdates(base, data.cookies), cacheHit: false};
}