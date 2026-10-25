import { ImageError } from "./image-error.mjs";
/** @typedef {{domain: string, subdomains: boolean, path: string, secure: boolean, expires: number, name: string, value: string}} Cookie */

/** Parse Netscape cookies without discarding domain/path restrictions.
 * Two-column exports fail closed: their cross-host scope is unknown.
 * @param {string} text @returns {(url: string) => string} */
export function cookieJar(text) {
  /** @type {Cookie[]} */ const cookies = [];
  for (const raw of text.split(/\r?\n/)) {
    const line = raw.trimStart();
    if (!line.trim() || (line.startsWith("#") && !line.startsWith("#HttpOnly_"))) continue;
    const fields = line.replace(/^#HttpOnly_/, "").split("\t");
    if (fields.length !== 7) throw new ImageError("cookie_domain_metadata_required");
    const [domain, subdomains, path, secure, expiry, name, value] = fields;
    if (!/^\.?[a-z0-9.-]+$/i.test(domain) || !path.startsWith("/") ||
        !["TRUE", "FALSE"].includes(subdomains) || !["TRUE", "FALSE"].includes(secure) ||
        !/^\d+$/.test(expiry) || !/^[!#$%&'*+.^_`|~a-z0-9-]+$/i.test(name) || /[;\r\n]/.test(value)) {
      throw new ImageError("invalid_cookie_export");
    }
    cookies.push({ domain: domain.replace(/^\./, "").toLowerCase(), subdomains: subdomains === "TRUE",
      path, secure: secure === "TRUE", expires: Number(expiry), name, value });
  }
  if (!cookies.length) throw new ImageError("cookie_secret_unavailable");
  return value => {
    const url = new URL(value), now = Date.now() / 1000;
    return cookies.filter(c => (!c.expires || c.expires > now) && (!c.secure || url.protocol === "https:") &&
      (url.hostname === c.domain || (c.subdomains && url.hostname.endsWith(`.${c.domain}`))) &&
      (url.pathname === c.path || url.pathname.startsWith(c.path.endsWith("/") ? c.path : `${c.path}/`)))
      .sort((a, b) => b.path.length - a.path.length).map(c => `${c.name}=${c.value}`).join("; ");
  };
}