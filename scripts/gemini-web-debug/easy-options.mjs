import { parseArgs } from "node:util";
import { join, resolve } from "node:path";

/** @typedef {{prompt: string, directory: string, cookieFile: string | undefined, help: boolean, refine?: string}} EasyOptions */

/** Resolve the short CLI without opening files or contacting Gemini.
 * @param {string[]} args
 * @param {{cookieFile?: string, outputRoot: string}} defaults
 * @returns {EasyOptions} */
export function easyOptions(args, defaults) {
  const {values, positionals} = parseArgs({ args, allowPositionals: true, options: {
    cookies: {type: "string"}, out: {type: "string"}, refine: {type: "string"}, help: {type: "boolean", short: "h"},
  } });
  return { prompt: positionals.join(" ").trim(), help: values.help ?? false, refine: values.refine,
    cookieFile: values.cookies ? resolve(values.cookies) : defaults.cookieFile,
    directory: resolve(values.out ?? join(defaults.outputRoot, new Date().toISOString().replace(/[-:.]/g, ""))) };
}

export const EASY_HELP = `Usage: ./gemini-image "describe the image"
       ./gemini-image                         (interactive prompt)
       ./gemini-image "your prompt" --out ./new-folder --cookies /path/cookies.txt
       ./gemini-image --refine latest "Make the lighting warmer"

Images go into a new folder under ./gemini-images by default.
Cookies: --cookies, then GEMINI_WEB_COOKIE_FILE, then ~/NAS/wireshark/cookie.txt
when present; otherwise uses Vault. Renewed cookies stay in a private local cache.`;