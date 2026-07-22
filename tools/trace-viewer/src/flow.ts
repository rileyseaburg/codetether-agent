import { clear, element, svg } from "./dom.js";
import type { TraceRecord } from "./types.js";

export function flow(records: TraceRecord[]): void {
  const target = element<SVGSVGElement>("flow");
  clear(target);
  const routes = records.map((item) => item.parsed.arguments?.match(/TCPv\d+:\[\[(.*?)->\[(.*?)\]>/)).filter((match): match is RegExpMatchArray => match !== null);
  const local = routes[0]?.[1] ?? "local process";
  const remote = routes[0]?.[2] ?? "remote endpoint";
  target.append(svg("path", { d: "M145 110 C220 40 300 180 375 110", class: "flow-path" }));
  const nodes: Array<readonly [number, string, string]> = [
    [25, local, "LOCAL SOCKET"], [375, remote, `${routes.length} EVENTS`],
  ];
  nodes.forEach(([left, name, meta]) => {
    target.append(svg("rect", { x: `${left}`, y: "70", width: "120", height: "80", rx: "9", class: "flow-node" }));
    const title = svg("text", { x: `${left + 12}`, y: "105", class: "flow-title" });
    title.textContent = name.slice(0, 16);
    const detail = svg("text", { x: `${left + 12}`, y: "128", class: "flow-meta" });
    detail.textContent = meta;
    target.append(title, detail);
  });
}
