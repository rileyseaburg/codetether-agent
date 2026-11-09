/** @typedef {{name: string, value: string, expires: number}} CookieUpdate */
export const ROTATED_NAMES = ["__Secure-1PSIDTS", "__Secure-3PSIDTS", "__Secure-1PSIDCC", "__Secure-3PSIDCC"];

/** Accept only scoped session updates from Google's rotation response.
 * @param {string[]} headers @returns {CookieUpdate[]} */
export function cookieUpdates(headers) {
  /** @type {CookieUpdate[]} */ const updates = [];
  for (const header of headers) {
    const [pair, ...attributes] = header.split(";"), split = pair.indexOf("=");
    const name = pair.slice(0, split), value = pair.slice(split + 1);
    const attrs = new Map(attributes.map(part => {
      const [key, ...rest] = part.trim().split("="); return [key.toLowerCase(), rest.join("=")];
    }));
    if (!ROTATED_NAMES.includes(name) || attrs.get("domain")?.replace(/^\./, "").toLowerCase() !== "google.com" ||
        attrs.get("path") !== "/" || !attrs.has("secure") || /[;\r\n]/.test(value)) continue;
    const expiry = Date.parse(attrs.get("expires") ?? "");
    const age = attrs.get("max-age");
    const expires = age && /^-?\d+$/.test(age) ? Date.now() + Number(age) * 1000 : expiry;
    if (!Number.isFinite(expires)) continue;
    updates.push({name, value, expires});
  }
  return updates;
}

/** Overlay only unexpired secure google.com cookies; never widen base scope.
 * @param {(url: string) => string} base @param {CookieUpdate[]} updates
 * @returns {(url: string) => string} */
export function withCookieUpdates(base, updates) {
  return value => {
    const url = new URL(value), original = base(value);
    if (url.protocol !== "https:" || !(url.hostname === "google.com" || url.hostname.endsWith(".google.com"))) return original;
    const pairs = new Map(original.split(/;\s*/).filter(Boolean).map(v => {
      const i = v.indexOf("="); return [v.slice(0, i), v.slice(i + 1)];
    }));
    for (const item of updates) if (ROTATED_NAMES.includes(item.name)) {
      if (item.expires > Date.now() && item.value) pairs.set(item.name, item.value); else pairs.delete(item.name);
    }
    return [...pairs].map(([name, text]) => `${name}=${text}`).join("; ");
  };
}