import { execFileSync } from "node:child_process";
import { readFileSync } from "node:fs";
import { cookieJar } from "./cookie-jar.mjs";
import { ImageError } from "./image-error.mjs";

/** Load scoped cookies from Vault or an explicitly selected Netscape file.
 * @param {string | undefined} [cookieFile] Defaults to GEMINI_WEB_COOKIE_FILE.
 * @returns {(url: string) => string} */
export function loadCookieJar(cookieFile = process.env.GEMINI_WEB_COOKIE_FILE) {
  if (cookieFile) {
    /** @type {Buffer} */ let raw;
    try { raw = readFileSync(cookieFile); } catch { throw new ImageError("cookie_file_read_failed"); }
    const text = raw.toString(raw[0] === 255 && raw[1] === 254 ? "utf16le" : "utf8");
    return cookieJar(text.replace(/^\uFEFF/, ""));
  }
  /** @type {unknown} */ let value;
  try {
    value = JSON.parse(execFileSync("vault", ["kv", "get", "-format=json",
      "secret/codetether/providers/gemini-web"], { encoding: "utf8", maxBuffer: 1024 * 1024,
      timeout: 30000, stdio: ["ignore", "pipe", "pipe"] }));
  } catch { throw new ImageError("vault_cookie_read_failed"); }
  for (const key of ["data", "data", "cookies"]) {
    if (!value || typeof value !== "object" || !(key in value)) throw new ImageError("cookie_secret_unavailable");
    value = /** @type {Record<string, unknown>} */ (value)[key];
  }
  if (typeof value !== "string") throw new ImageError("cookie_secret_unavailable");
  return cookieJar(value);
}