import { mkdirSync, readFileSync, writeFileSync, renameSync, lstatSync } from "node:fs";
import { dirname, join } from "node:path";
import { randomUUID } from "node:crypto";
import { ROTATED_NAMES } from "./cookie-refresh-update.mjs";
import { ImageError } from "./image-error.mjs";

/** @typedef {{updated: number, cookies: import("./cookie-refresh-update.mjs").CookieUpdate[]}} RefreshCache */

/** Read only valid private rotation data. Invalid/stale files are left intact.
 * @param {string} path @returns {RefreshCache | null} */
export function readRefreshCache(path) {
  try {
    const parent = lstatSync(dirname(path));
    if (!parent.isDirectory() || (process.platform !== "win32" && (parent.mode & 0o077))) return null;
    const stat = lstatSync(path);
    if (!stat.isFile() || stat.size > 65536 || (process.platform !== "win32" && (stat.mode & 0o077))) return null;
    /** @type {unknown} */ const value = JSON.parse(readFileSync(path, "utf8"));
    if (!value || typeof value !== "object" || !("updated" in value) || typeof value.updated !== "number" ||
        !("cookies" in value) || !Array.isArray(value.cookies)) return null;
    /** @type {import("./cookie-refresh-update.mjs").CookieUpdate[]} */ const cookies = [];
    for (const item of value.cookies) {
      if (!item || typeof item !== "object" || typeof item.name !== "string" || !ROTATED_NAMES.includes(item.name) ||
          typeof item.value !== "string" || /[;\r\n]/.test(item.value) || typeof item.expires !== "number") return null;
      cookies.push({name: item.name, value: item.value, expires: item.expires});
    }
    return {updated: value.updated, cookies};
  } catch { return null; }
}

/** Atomically save refreshed credentials in a private local cache, not the NAS.
 * @param {string} directory @param {string} key @param {RefreshCache} data
 * @returns {void} */
export function writeRefreshCache(directory, key, data) {
  mkdirSync(directory, {recursive: true, mode: 0o700});
  const stat = lstatSync(directory);
  if (!stat.isDirectory() || (process.platform !== "win32" && (stat.mode & 0o077))) {
    throw new ImageError("cookie_cache_not_private");
  }
  const temporary = join(directory, `${key}.${randomUUID()}.tmp`);
  writeFileSync(temporary, JSON.stringify(data), {mode: 0o600, flag: "wx"});
  renameSync(temporary, join(directory, `${key}.json`));
}