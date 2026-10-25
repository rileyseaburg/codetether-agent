/** @typedef {{at: string, bl: string, sid: string, authenticatedToken: boolean}} SessionTokens */

/** Extract a JSON string field without returning HTML or leaking parse errors.
 * @param {string} html @param {string} name @returns {string} */
function field(html, name) {
  const match = html.match(new RegExp(`"${name}"\\s*:\\s*("(?:[^"\\\\]|\\\\.)*")`));
  if (!match) return "";
  try { return JSON.parse(match[1]); } catch { return ""; }
}

/** Prefer the signed-in app token even when the anonymous token appears first.
 * Both can occur in one bootstrap document. Presence is a prerequisite for
 * image creation, not a guarantee of account capability or token validity.
 * @param {string} html @returns {SessionTokens} */
export function sessionTokens(html) {
  const authenticated = field(html, "SNlM0e");
  return {
    at: authenticated || field(html, "thykhd"),
    bl: field(html, "cfb2h"),
    sid: field(html, "FdrFJe"),
    authenticatedToken: Boolean(authenticated),
  };
}