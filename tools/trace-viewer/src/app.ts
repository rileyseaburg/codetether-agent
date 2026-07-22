import { closeDetail } from "./detail.js";
import { element } from "./dom.js";
import { loadFile, loadUrl } from "./loader.js";
import { filtered } from "./metrics.js";
import { render } from "./render.js";
import { table } from "./table.js";
import type { TraceRecord } from "./types.js";

let records: TraceRecord[] = [];

function hideLoading(): void {
  element<HTMLElement>("loading").classList.add("hidden");
}

function applyFilter(): void {
  const query = element<HTMLInputElement>("search").value;
  const syscall = element<HTMLSelectElement>("syscall").value;
  table(filtered(records, query, syscall));
}

async function start(): Promise<void> {
  try {
    records = await loadUrl("/trace/all.jsonl");
    render(records);
  } catch (error: unknown) {
    element<HTMLElement>("source-name").textContent = error instanceof Error ? error.message : String(error);
  } finally {
    hideLoading();
  }
}

element<HTMLInputElement>("file-input").addEventListener("change", async (event) => {
  const input = event.currentTarget as HTMLInputElement;
  if (!input.files?.[0]) return;
  records = await loadFile(input.files[0]);
  element<HTMLElement>("source-name").textContent = input.files[0].name;
  render(records);
});
element<HTMLInputElement>("search").addEventListener("input", applyFilter);
element<HTMLSelectElement>("syscall").addEventListener("change", applyFilter);
element<HTMLButtonElement>("close-detail").addEventListener("click", closeDetail);
void start();
