import { clear, element } from "./dom.js";
import { bytes, integer, seconds } from "./format.js";
import type { TraceMetrics } from "./types.js";

export function summary(data: TraceMetrics): void {
  const target = element<HTMLElement>("metrics");
  clear(target);
  const values: Array<readonly [string, string]> = [
    ["Captured records", integer(data.records)],
    ["Transferred bytes", bytes(data.bytes)],
    ["Observed span", seconds(data.span)],
    ["Socket descriptors", integer(data.sockets)],
  ];
  values.forEach(([label, value]) => {
    const card = document.createElement("article");
    card.className = "metric";
    const caption = document.createElement("span");
    caption.textContent = label;
    const strong = document.createElement("strong");
    strong.textContent = value;
    card.append(caption, strong);
    target.append(card);
  });
}
