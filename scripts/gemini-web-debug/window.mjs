const OMITTED = "System: [Earlier conversation omitted to fit Gemini Web request limit.]";

export function fit(messages, rendered, limit) {
  if (bytes(rendered) <= limit) return rendered;
  const prefix = messages.findIndex(message => !["system", "developer"].includes(message.role));
  const fixed = bytes(rendered.slice(0, Math.max(prefix, 0))) + Buffer.byteLength(OMITTED) + 2;
  let available = Math.max(0, limit - fixed);
  const starts = turnStarts(messages, Math.max(prefix, 0));
  let selected = rendered.length;
  for (let index = starts.length - 2; index >= 0; index -= 1) {
    const size = bytes(rendered.slice(starts[index], starts[index + 1])) + 1;
    if (size > available) break;
    available -= size;
    selected = starts[index];
  }
  const latest = starts.at(-2) ?? rendered.length - 1;
  const tail = selected < rendered.length ? rendered.slice(selected) : [rendered[latest]];
  return [...rendered.slice(0, Math.max(prefix, 0)), OMITTED, ...tail];
}

function turnStarts(messages, prefix) {
  const starts = messages.flatMap((message, index) =>
    index >= prefix && message.role === "user" ? [index] : []);
  if (starts[0] !== prefix) starts.unshift(prefix);
  starts.push(messages.length);
  return starts;
}

function bytes(items) {
  return items.reduce((total, item) => total + Buffer.byteLength(item), 0)
    + Math.max(0, items.length - 1);
}
