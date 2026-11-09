import { mkdirSync, writeFileSync } from "node:fs";
import { longestText } from "./extract.mjs";
import { streamGenerate } from "./request.mjs";
import { scrape } from "./tokens.mjs";
import { loadCookieHeader } from "./vault.mjs";

const questions = [
  "Hi Gemini. We are testing a seven-turn conversation. Call me Riley and reply briefly.",
  "What name did I ask you to call me, and how many turns are we testing?",
  "Our bug was that a 1.3 MB resumed session failed, while a 240 KB bounded one worked. What is the likely lesson?",
  "Give one reason complete user turns are safer than cutting the transcript at an arbitrary byte offset.",
  "If a tool result is retained, what related item must also be retained?",
  "Summarize the validation so far in exactly two short bullet points.",
  "Final continuity check: state my name, the original failing size, and the successful bounded size.",
];
const directory = process.argv[2] ?? ".codetether-diagnostics/gemini-web-seven-turn";
mkdirSync(directory, { recursive: true });
const cookie = loadCookieHeader();
const tokens = await scrape(cookie);
const messages = [{ role: "System", text: "Be concise. Do not call tools; answer conversationally." }];
const transcript = [];

for (const [index, question] of questions.entries()) {
  messages.push({ role: "User", text: question });
  const prompt = messages.map(item => `${item.role}: ${item.text}`).join("\n");
  const started = performance.now();
  const response = await streamGenerate(prompt, cookie, tokens);
  const body = Buffer.from(await response.arrayBuffer());
  const elapsedMs = performance.now() - started;
  const answer = longestText(body.toString("utf8"));
  const estimatedTokens = Math.ceil(answer.length / 4);
  messages.push({ role: "Assistant", text: answer });
  transcript.push({ turn: index + 1, question, answer, status: response.status,
    promptBytes: Buffer.byteLength(prompt), responseBytes: body.length, elapsedMs,
    outputChars: answer.length, charsPerSecond: answer.length * 1000 / elapsedMs,
    estimatedTokens, estimatedTps: estimatedTokens * 1000 / elapsedMs });
  writeFileSync(`${directory}/turn-${index + 1}.body.raw`, body);
  console.log(JSON.stringify(transcript.at(-1)));
}

writeFileSync(`${directory}/transcript.json`, JSON.stringify(transcript, null, 2));
