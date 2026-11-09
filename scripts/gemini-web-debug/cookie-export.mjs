/** @typedef {{domain: string, hostOnly: boolean, secure: boolean, path: string, httpOnly: boolean, name: string, value: string, expirationDate: number}} CookieExport */

/** Convert the supplied Gemini export into provider-compatible Netscape text.
 * @param {CookieExport[]} entries
 * @returns {string}
 */
export function netscapeCookies(entries) {
  const seen = new Set();
  const rows = entries.map(cookie => {
    if (!cookie || ![".google.com", ".gemini.google.com"].includes(cookie.domain)
      || cookie.hostOnly !== false || cookie.secure !== true || cookie.path !== "/"
      || typeof cookie.httpOnly !== "boolean" || typeof cookie.name !== "string"
      || !cookie.name || typeof cookie.value !== "string" || !cookie.value
      || !Number.isFinite(cookie.expirationDate)) {
      throw new Error("Invalid cookie export entry");
    }
    if ([cookie.domain, cookie.path, cookie.name, cookie.value].some(field => /[\r\n\t]/.test(field))
      || seen.has(cookie.name)) throw new Error("Invalid cookie delimiter or duplicate name");
    seen.add(cookie.name);
    return [`${cookie.httpOnly ? "#HttpOnly_" : ""}${cookie.domain}`, "TRUE",
      cookie.path, "TRUE", String(Math.floor(cookie.expirationDate)),
      cookie.name, cookie.value].join("\t");
  });
  if (rows.length !== 14 || !seen.has("__Secure-1PSID") || !seen.has("__Secure-3PSID")) {
    throw new Error("Cookie export is incomplete");
  }
  return "# Netscape HTTP Cookie File\n" + rows.join("\n") + "\n";
}
