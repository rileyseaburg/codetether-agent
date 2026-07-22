import { clear, element } from "./dom.js";
import { detail } from "./detail.js";
import type { TraceRecord } from "./types.js";

export function syscallOptions(records: TraceRecord[]): void {
  const select = element<HTMLSelectElement>("syscall");
  while (select.options.length > 1) select.remove(1);
  const calls = [...new Set(records.map((item) => item.parsed.syscall).filter((name): name is string => Boolean(name)))].sort();
  calls.forEach((name) => select.add(new Option(name, name)));
}

export function table(records: TraceRecord[]): void {
  const target = element<HTMLTableSectionElement>("records");
  clear(target);
  const visible = records;
  visible.forEach((record) => {
    const row = document.createElement("tr");
    const values = [record.line, record.parsed.timestamp ?? "—", record.parsed.syscall ?? record.parsed.record_type, record.parsed.result ?? record.parsed.exit_code ?? "—", record.parsed.duration ?? "—"];
    values.forEach((value) => {
      const cell = document.createElement("td");
      cell.textContent = String(value);
      row.append(cell);
    });
    row.addEventListener("click", () => detail(record));
    target.append(row);
  });
  element<HTMLElement>("visible-count").textContent = `${visible.length} / ${records.length}`;
}
