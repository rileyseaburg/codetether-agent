import { execFileSync } from "node:child_process";

export function loadCookieHeader() {
  const raw = execFileSync("vault", [
    "kv", "get", "-format=json", "secret/codetether/providers/gemini-web",
  ], { encoding: "utf8", maxBuffer: 1024 * 1024 });
  const cookies = JSON.parse(raw).data.data.cookies;
  return cookies.split("\n").flatMap(line => {
    const text = line.trim();
    if (!text || (text.startsWith("#") && !text.startsWith("#HttpOnly_"))) return [];
    const fields = text.replace(/^#HttpOnly_/, "").split("\t");
    const pair = fields.length >= 7 ? [fields[5], fields[6]] : fields.slice(0, 2);
    return pair[0] ? [`${pair[0].trim()}=${pair[1].trim()}`] : [];
  }).join("; ");
}
