import { mkdirSync, writeFileSync } from "node:fs";
import { longestText } from "./extract.mjs";
import { loadCookieHeader } from "./vault.mjs";
import { sessionPrompt } from "./prompt.mjs";
import { streamGenerate } from "./request.mjs";
import { scrape } from "./tokens.mjs";

const session = process.argv[2];
const output = process.argv[3] ?? ".codetether-diagnostics/gemini-web-direct-node";
const maxBytes = Number(process.argv[4] ?? Number.POSITIVE_INFINITY);
if (!session) throw new Error("usage: node main.mjs <session.json> [output-prefix]");
const cookie = loadCookieHeader();
const tokens = await scrape(cookie);
const prompt = sessionPrompt(session, maxBytes);
const response = await streamGenerate(prompt, cookie, tokens);
const body = Buffer.from(await response.arrayBuffer());
const headers = Object.fromEntries(response.headers);
delete headers["set-cookie"];
mkdirSync(output.slice(0, output.lastIndexOf("/")), { recursive: true });
writeFileSync(`${output}.body.raw`, body);
writeFileSync(`${output}.text.txt`, longestText(body.toString("utf8")));
writeFileSync(`${output}.response.json`, JSON.stringify({
  status: response.status, statusText: response.statusText,
  headers, promptBytes: Buffer.byteLength(prompt), responseBytes: body.length,
}, null, 2));
process.stdout.write(body);
console.error(`\nnode-direct status=${response.status} promptBytes=${Buffer.byteLength(prompt)} responseBytes=${body.length}`);
