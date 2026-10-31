/** Backpressure follows the archive writer, not an unbounded readline queue. */
export async function* lines(stream) {
  let pending = '', oversized = false;
  for await (const chunk of stream) {
    pending += chunk.toString('utf8');
    let end;
    while ((end = pending.indexOf('\n')) >= 0) {
      const line = pending.slice(0, end); pending = pending.slice(end + 1);
      if (!oversized && line.length <= 4 * 1024 * 1024) yield line;
      oversized = false;
    }
    if (pending.length > 4 * 1024 * 1024) { pending = ''; oversized = true; }
  }
  if (pending && !oversized) yield pending;
}