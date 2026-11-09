import { readFileSync } from "node:fs";
import { fit } from "./window.mjs";

export function sessionPrompt(path, maxBytes = Number.POSITIVE_INFINITY) {
  const session = JSON.parse(readFileSync(path, "utf8"));
  const rendered = session.messages.map(message => {
    const role = message.role[0].toUpperCase() + message.role.slice(1);
    return `${role}: ${message.content.map(part).filter(Boolean).join("\n")}`;
  });
  return fit(session.messages, rendered, maxBytes).join("\n");
}

function part(value) {
  if (value.type === "text") return value.text;
  if (value.type === "thinking") return `<thinking>${value.text}</thinking>`;
  if (value.type === "tool_call") return tag("tool_call", {
    name: value.name, arguments: parse(value.arguments),
  });
  if (value.type === "tool_result") return tag("tool_result", {
    tool_call_id: value.tool_call_id, content: value.content,
  });
  return null;
}

function tag(name, value) {
  return `<${name}>${JSON.stringify(value).replaceAll("</", "<\\/")}</${name}>`;
}

function parse(value) {
  try { return JSON.parse(value); } catch { return value; }
}
