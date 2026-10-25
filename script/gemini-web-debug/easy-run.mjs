import { spawnSync } from "node:child_process";
import { existsSync, mkdirSync, readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

/** @typedef {(command: string, args: string[], options: import("node:child_process").SpawnSyncOptionsWithStringEncoding) => import("node:child_process").SpawnSyncReturns<string>} Run */
/** Friendly presentation around the already-tested diagnostic CLI.
 * Child stdout/stderr are not echoed: credentials and raw transport errors must
 * never leak through the convenience wrapper. Detailed evidence stays on disk.
 * @param {import("./easy-options.mjs").EasyOptions} options
 * @param {Run} [run] Test seam; production executes Node directly.
 * @param {(message: string) => void} [log] @returns {number} */
export function runEasy(options, run = spawnSync, log = console.log) {
  if (existsSync(options.directory)) { log("That output folder already exists. Choose a new --out folder."); return 2; }
  mkdirSync(dirname(options.directory), {recursive: true, mode: 0o700});
  log(options.refine ? "Refining your image…" : "Generating your image…");
  const env = {...process.env};
  if (options.cookieFile) env.GEMINI_WEB_COOKIE_FILE = options.cookieFile;
  const worker = fileURLToPath(new URL("./generate-image.mjs", import.meta.url));
  const args = [worker, options.prompt, options.directory, ...(options.refine ? [options.refine] : [])];
  const result = run(process.execPath, args, {
    env, encoding: "utf8", maxBuffer: 1024 * 1024,
  });
  /** @type {unknown} */ let report;
  try { report = JSON.parse(readFileSync(join(options.directory, result.status === 0 ? "summary.json" : "failure.json"), "utf8")); }
  catch { log(`Could not finish. Check Node/curl and output permissions. Output: ${options.directory}`); return 2; }
  if (!report || typeof report !== "object") { log("Invalid result file."); return 2; }
  if (result.status === 0 && "images" in report && Array.isArray(report.images) && report.images.length) {
    for (const image of report.images) {
      if (!image || typeof image !== "object" || typeof image.path !== "string") { log("Invalid image result."); return 2; }
      log(`Saved: ${image.path}`);
    }
    if ("refinementSource" in report && typeof report.refinementSource === "string") log(`Refined from: ${dirname(report.refinementSource)}`);
    log(`Details: ${join(options.directory, "summary.json")}`); return 0;
  }
  const code = "code" in report && typeof report.code === "string" ? report.code : "generation_failed";
  if (["image_session_not_authenticated", "generation_image_creation_unavailable", "cookie_refresh_rejected", "cookie_refresh_missing_session"].includes(code)) {
    log("Gemini is not accepting image creation with this session. Sign in, export fresh cookies, and retry.");
  } else if (["cookie_file_read_failed", "cookie_domain_metadata_required", "invalid_cookie_export"].includes(code)) {
    log("Cannot use that cookie file. Supply a Netscape export with --cookies /path/cookies.txt.");
  } else if (code.startsWith("image_state_") || code === "image_refinement_lost_context") {
    log("Cannot continue that image conversation. Choose an output folder with conversation.json, or generate a new image first.");
  } else {
    log("Image generation failed. The diagnostic file below contains the failure code.");
  }
  log(`Details: ${join(options.directory, "failure.json")}`); return 2;
}