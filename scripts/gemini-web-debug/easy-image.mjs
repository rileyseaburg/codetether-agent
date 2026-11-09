import { existsSync } from "node:fs";
import { homedir } from "node:os";
import { join } from "node:path";
import { createInterface } from "node:readline/promises";
import { easyOptions, EASY_HELP } from "./easy-options.mjs";
import { runEasy } from "./easy-run.mjs";

try {
  const nasCookies = join(homedir(), "NAS", "wireshark", "cookie.txt");
  const cookieFile = process.env.GEMINI_WEB_COOKIE_FILE || (existsSync(nasCookies) ? nasCookies : undefined);
  const options = easyOptions(process.argv.slice(2), {cookieFile, outputRoot: "gemini-images"});
  if (options.help) {
    console.log(EASY_HELP);
  } else {
    if (!options.prompt && process.stdin.isTTY) {
      const input = createInterface({input: process.stdin, output: process.stdout});
      try { options.prompt = (await input.question("Describe the image: ")).trim(); }
      finally { input.close(); }
    }
    if (!options.prompt) { console.log(EASY_HELP); process.exitCode = 2; }
    else process.exitCode = runEasy(options);
  }
} catch {
  console.error("Could not start. Check your arguments and output permissions. Use --help for usage.");
  process.exitCode = 2;
}