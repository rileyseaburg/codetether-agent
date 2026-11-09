// Compare prompts while holding the tool schema and hidden-data task constant.
import { tools } from "./tool-probe-fixture.mjs";

export const variants = [
  { name: "schema_only", instructions: "You have the supplied tools. Use them to fulfill the user's request." },
  { name: "explicit_xml", instructions: [
    "You are connected to a real tool executor. To invoke a tool, output exactly:",
    '<tool_call>{"name":"TOOL_NAME","arguments":{"PARAMETER":"VALUE"}}</tool_call>',
    "Use actual tool names and arguments from the schema. Emit one call, then STOP.",
    "Wait for the executor's tool_result before continuing. Never invent results.",
    "Do not write Python, markdown, or prose announcing a call. Output the raw tag.",
    "After both tools return, output only the requested FINAL line.",
  ].join("\n") },
  { name: "few_shot_xml", instructions: [
    "You are the controller of an external tool runner, not a code interpreter.",
    'Example invocation: <tool_call>{"name":"lookup_probe","arguments":{"id":"example-record"}}</tool_call>',
    "That is an example only; use the user's actual record ID. Stop after the call.",
    "The runner will supply a tool_result. Use that data for the next tool call.",
    "Exactly one raw tool_call tag per response; no fences, simulated output, or commentary.",
    "When the task is finished, emit only its FINAL line, not another tool call.",
  ].join("\n") },
];

export function prompt(variant, id) {
  return `System: ${variant.instructions}\n<available_tools>${JSON.stringify(tools)}</available_tools>\nUser: Look up record ${id} using lookup_probe. Then use multiply to multiply its quantity by 7. After receiving both real tool results, return exactly FINAL followed by the record's verification_code and the product, separated by spaces. The code and quantity are not supplied here; you must obtain them from the tool.\nAssistant:`;
}