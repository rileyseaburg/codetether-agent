// Strict diagnostic parser: only a single raw JSON tool_call is executable.
export function parseCall(answer) {
  if (answer.includes("<tool_result")) throw new Error("fabricated_tool_result");
  const match = answer.trim().match(/^<tool_call>\s*([\s\S]*?)\s*<\/tool_call>$/);
  if (!match) {
    if (answer.includes("<tool_call")) throw new Error("noncanonical_tool_call");
    return null;
  }
  try {
    return JSON.parse(match[1]);
  } catch {
    throw new Error("invalid_tool_call_json");
  }
}
