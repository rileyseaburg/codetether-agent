import type { TraceMetrics, TraceRecord } from "./types.js";

export function metrics(records: TraceRecord[]): TraceMetrics {
  const calls = new Map<string, number>();
  const descriptors = new Set<string>();
  let bytes = 0;
  let duration = 0;
  const times: number[] = [];
  records.forEach(({ parsed }) => {
    const name = parsed.syscall ?? parsed.record_type;
    calls.set(name, (calls.get(name) ?? 0) + 1);
    const result = Number.parseInt(parsed.result ?? "", 10);
    if (result > 0) bytes += result;
    duration += Number.parseFloat(parsed.duration ?? "0") || 0;
    const timestamp = Number.parseFloat(parsed.timestamp ?? "");
    if (Number.isFinite(timestamp)) times.push(timestamp);
    const descriptor = parsed.arguments?.match(/^(\d+)</)?.[1];
    if (descriptor) descriptors.add(descriptor);
  });
  const first = times.length ? Math.min(...times) : 0;
  const last = times.length ? Math.max(...times) : 0;
  return { records: records.length, bytes, duration, span: last - first, sockets: descriptors.size, calls, first, last };
}

export function filtered(records: TraceRecord[], query: string, syscall: string): TraceRecord[] {
  const needle = query.trim().toLowerCase();
  return records.filter((item) => (!syscall || item.parsed.syscall === syscall)
    && (!needle || item.raw.toLowerCase().includes(needle)));
}
