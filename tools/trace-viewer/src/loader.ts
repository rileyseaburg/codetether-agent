import type { TraceRecord } from "./types.js";

function object(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null;
}

function record(value: unknown): value is TraceRecord {
  return object(value) && typeof value.line === "number" && typeof value.raw === "string"
    && typeof value.raw_base64 === "string" && Array.isArray(value.tokens) && object(value.parsed);
}

export function parseJsonl(text: string): TraceRecord[] {
  return text.split("\n").filter(Boolean).map((line, index) => {
    const value: unknown = JSON.parse(line);
    if (!record(value)) throw new Error(`Invalid trace record at JSONL line ${index + 1}`);
    return value;
  });
}

async function text(url: string): Promise<string> {
  for (let attempt = 0; attempt < 2; attempt += 1) {
    try {
      const response = await fetch(url);
      if (!response.ok) throw new Error(`Trace request failed: HTTP ${response.status}`);
      return response.text();
    } catch (error: unknown) {
      if (attempt === 1) throw error;
    }
  }
  throw new Error("Trace request failed");
}

export async function loadUrl(url: string): Promise<TraceRecord[]> {
  return parseJsonl(await text(url));
}

export async function loadFile(file: File): Promise<TraceRecord[]> {
  return parseJsonl(await file.text());
}
