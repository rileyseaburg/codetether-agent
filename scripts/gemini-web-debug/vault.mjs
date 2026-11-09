import { execFileSync } from "node:child_process";

/** Load the existing Vault cookie secret without logging credential values.
 * @returns {string} Netscape cookie entries converted to an HTTP Cookie header. */
export function loadCookieHeader() {
  const raw = execFileSync("vault", [
    "kv", "get", "-format=json", "secret/codetether/providers/gemini-web",
  ], { encoding: "utf8", maxBuffer: 1024 * 1024 });
  /** @type {unknown} */
  const cookies = JSON.parse(raw).data.data.cookies;
  if (typeof cookies !== "string" || !cookies.trim()) throw new Error("cookie_secret_unavailable");
  /** @param {string} line @returns {string[]} */
  return cookies.split("\n").flatMap(line => {
    const text = line.trim();
    if (!text || (text.startsWith("#") && !text.startsWith("#HttpOnly_"))) return [];
    const fields = text.replace(/^#HttpOnly_/, "").split("\t");
    const pair = fields.length >= 7 ? [fields[5], fields[6]] : fields.slice(0, 2);
    return pair[0] ? [`${pair[0].trim()}=${pair[1].trim()}`] : [];
  }).join("; ");
}