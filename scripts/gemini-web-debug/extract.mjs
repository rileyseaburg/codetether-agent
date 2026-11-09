export function longestText(raw) {
  let best = "";
  for (const line of raw.split("\n")) {
    if (!line.trim().startsWith("[")) continue;
    try {
      for (const event of JSON.parse(line)) {
        if (typeof event?.[2] !== "string" || !event[2].startsWith("[")) continue;
        const text = JSON.parse(event[2])?.[4]?.[0]?.[1]?.[0];
        if (typeof text === "string" && text.length > best.length) best = text;
      }
    } catch {
      // Length and XSSI framing lines are intentionally ignored.
    }
  }
  return best;
}
