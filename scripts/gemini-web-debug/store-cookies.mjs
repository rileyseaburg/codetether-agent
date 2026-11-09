// Run with terminal echo disabled. Provide one JSON line on stdin, never argv.
// Input shape: {"token": "temporary Vault credential", "cookies": [cookie export]}.
import { createInterface } from "node:readline";
import { netscapeCookies } from "./cookie-export.mjs";
import { storeCookies } from "./vault-cookie-write.mjs";

/** @type {{token: string, cookies: import("./cookie-export.mjs").CookieExport[]} | undefined} */
let input;
const lines = createInterface({ input: process.stdin, terminal: false });
console.log("READY_FOR_SECRET_STDIN_NO_ECHO");
try {
  for await (const line of lines) {
    input = JSON.parse(line);
    lines.close();
    break;
  }
  if (!input || typeof input.token !== "string" || !input.token || !Array.isArray(input.cookies)) {
    throw new Error("Invalid secret input");
  }
  console.log(JSON.stringify(storeCookies(input.token, netscapeCookies(input.cookies))));
} catch {
  console.error(JSON.stringify({ phase: "failed", diagnostic: "Secret storage or verification failed; details suppressed", credentialsRedacted: true }));
  process.exitCode = 1;
} finally {
  if (input) input.token = "";
  lines.close();
}