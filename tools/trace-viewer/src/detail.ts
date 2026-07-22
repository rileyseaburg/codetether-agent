import { clear, element } from "./dom.js";
import type { TraceRecord } from "./types.js";

function section(title: string, value: string): HTMLElement {
  const container = document.createElement("section");
  const heading = document.createElement("h3");
  heading.textContent = title;
  const content = document.createElement("pre");
  content.textContent = value;
  container.append(heading, content);
  return container;
}

export function detail(record: TraceRecord): void {
  const drawer = element<HTMLElement>("detail");
  const body = element<HTMLElement>("detail-body");
  clear(body);
  const title = document.createElement("h2");
  title.textContent = `Record ${record.line} · ${record.parsed.syscall ?? record.parsed.record_type}`;
  body.append(title, section("Parsed fields", JSON.stringify(record.parsed, null, 2)), section("Exact raw text", record.raw), section("Lexical tokens", JSON.stringify(record.tokens, null, 2)), section("Exact source bytes · Base64", record.raw_base64));
  drawer.classList.add("open");
  drawer.setAttribute("aria-hidden", "false");
}

export function closeDetail(): void {
  const drawer = element<HTMLElement>("detail");
  drawer.classList.remove("open");
  drawer.setAttribute("aria-hidden", "true");
}
